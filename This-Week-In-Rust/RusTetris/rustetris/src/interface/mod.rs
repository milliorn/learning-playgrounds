mod render_traits;
mod sub_rect;

use std::time::Duration;

use cgmath::{ElementWise, EuclideanSpace, Point2, Vector2};

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, sys::KeyCode,
    video::Window,
};

use crate::engine::{Color as SemanticColor, Engine, Matrix, MoveKind};

use self::{
    render_traits::ScreenColor,
    sub_rect::{Align, SubRect},
};

const INIT_SIZE: Vector2<u32> = Vector2::new(1024, 1024);
const BACKGROUND_COLOR: Color = Color::RGB(0x10, 0x10, 0x18);
const MATRIX_COLOR: Color = Color::RGB(0x66, 0x77, 0x77);

struct Tick;
struct LockdownTick;
struct SoftDropTick;
struct Sleep(Duration);

pub fn run(mut engine: Engine) {
    let sdl = sdl2::init().expect("Failed to initialize SDL2");
    let event_subsystem = sdl.event().expect("Failed to acquire event subsystem");

    event_subsystem.register_custom_event::<Tick>().unwrap();
    event_subsystem
        .register_custom_event::<LockdownTick>()
        .unwrap();

    let mut canvas = {
        let video = sdl.video().expect("Failed to get display.");

        let window = video
            .window("RusTetris", INIT_SIZE.x, INIT_SIZE.y)
            .position_centered()
            .resizable()
            .build()
            .expect("Failed to create window");

        window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .expect("Failed to get rendered canvas")
    };

    let mut events = sdl.event_pump().expect("Failed to get event loop");

    event_subsystem.push_custom_event(Tick).unwrap();
    event_subsystem.push_custom_event(LockdownTick).unwrap();

    let mut dirty: bool = true;

    loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => return,
                Event::User { .. } if event.as_user_event_type::<Tick>().is_some() => {
                    println!("Found tick event");
                    dirty = true
                }
                Event::User { .. } if event.as_user_event_type::<LockdownTick>().is_some() => {
                    println!("Found lockdown tick event");
                    dirty = true
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Ok(input) = Input::try_from(key) {
                        match input {
                            Input::Move(kind) => drop(engine.move_cursor(kind)),
                            Input::HardDrop => engine.hard_drop(),
                            Input::SoftDrop => todo!(),
                        }
                        dirty = true
                    }
                }
                _ => {}
            }
        }

        if dirty {
            draw(&mut canvas, &engine);
            dirty = false;
        }
    }
}

enum Input {
    Move(MoveKind),
    SoftDrop,
    HardDrop,
}

impl TryFrom<Keycode> for Input {
    type Error = ();

    fn try_from(key: Keycode) -> Result<Self, Self::Error> {
        Ok(match key {
            Keycode::Right => Self::Move(MoveKind::Right),
            Keycode::Left => Self::Move(MoveKind::Left),
            Keycode::Up => Self::HardDrop,
            Keycode::Down => Self::SoftDrop,
            _ => return Err(()),
        })
    }
}

fn draw(canvas: &mut Canvas<Window>, engine: &Engine) {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    let viewport = canvas.viewport();
    let ui_square = SubRect::absolute(viewport, (1.0, 1.0), None);

    let matrix = ui_square
        .sub_rect((0.5, 1.0), None)
        .sub_rect((7.0 / 8.0, 7.0 / 8.0), None);

    let up_next = ui_square
        .sub_rect((0.25, 0.25), Some((Align::Far, Align::Near)))
        .sub_rect((0.75, 0.75), None);

    let hold = ui_square
        .sub_rect((0.25, 0.25), Some((Align::Near, Align::Near)))
        .sub_rect((0.75, 0.75), None);

    let queue = ui_square
        .sub_rect((0.25, 0.75), Some((Align::Far, Align::Far)))
        .sub_rect((5.0 / 8.0, 23.0 / 24.0), Some((Align::Center, Align::Near)));

    let score = ui_square
        .sub_rect((0.25, 11.0 / 16.0), Some((Align::Near, Align::Far)))
        .sub_rect((7.0 / 8.0, 8.0 / 11.0), Some((Align::Center, Align::Near)));

    canvas.set_draw_color(MATRIX_COLOR);

    for subrect in [&matrix, &up_next, &hold, &queue, &score] {
        canvas.fill_rect(Rect::from(subrect)).unwrap();
    }

    let cell_draw_ctx = CellDrawContext {
        origin: matrix.bottom_left(),
        dims: matrix.size(),
    };

    for (coord, cell) in engine.cursor_cells() {
        cell_draw_ctx.draw_cell(cell, coord, canvas)
    }

    canvas.present()
}

struct CellDrawContext {
    origin: Point2<i32>,
    dims: Vector2<u32>,
}

impl CellDrawContext {
    const CELL_COUNT: Vector2<usize> = Vector2::new(Matrix::WIDTH as u32, Matrix::HEIGHT as u32);

    fn draw_cell(&self, cell: &Option<Color>, coord: Point2<u32>, canvas: &mut Canvas<Window>) {
        let Some(cell_color) = cell else{return};

        let coord = coord.to_vec().cast::<u32>().unwrap();

        let this = (coord + Vector2::new(0, 1))
            .mul_element_wise(self.dims)
            .div_element_wise(Self::CELL_COUNT);

        let next = (coord + Vector2::new(1, 0))
            .mul_element_wise(self.dims)
            .div_element_wise(Self::CELL_COUNT);

        let cell_rect = Rect::new(
            self.matrix_origin.x + this.x as i32,
            self.matrix_origin.y - this.y as i32,
            next.x - this.x,
            this.y + next.y,
        );

        canvas.set_draw_color(cell_color.screen_color());
        canvas.fill_rect(cell_rect).unwrap();
    }
}
