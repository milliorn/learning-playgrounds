use bracket_lib::terminal::{main_loop, VirtualKeyCode, RED, RGB, WHITE, YELLOW};
use bracket_lib::terminal::{BError, BTerm, BTermBuilder, GameState};
use rltk::{Rltk};

// y value will be used to determine if the player can jump or should be falling at a given frame.
// going_down indicate my player is falling and perform the position shifts needed
// jumping indicate my player is jumping, and move the player upwards on each frame.
// game_over indicate the game is over.
struct State {
    game_over: bool,
    going_down: bool,
    jumping: bool,
    y: i32,
}

// By defining the GameState trait, the creators of bracket-lib gave developers a means to bring their own Rust struct that can talk to their game engine.
impl GameState for State {
    // first statement will clear the scene each time tick is invoked. By clearing all the objects, I can redraw these objects in different positions if need be.
    fn tick(&mut self, ctx: &mut BTerm) {
        player_input(self, ctx);

        ctx.cls_bg(RGB::named(WHITE));
        ctx.draw_bar_horizontal(0, 40, 80, 50, 50, RGB::named(YELLOW), RGB::named(YELLOW));

        // the shape representing my player. This shape’s y position will be determined by the state struct I defined earlier. For the sake of this game, the player will be a red box.
        ctx.draw_box_double(10, self.y, 5, 5, RGB::named(RED), RGB::named(RED));

        // reading the y value of my state to determine which altitude my box should be at. The x value of my box is constant because the universe will move around the player and not the other way around. If the state has determined my object should be going down it’ll add 1 to the stored y coordinate. By adding to this value, the object will appear to be falling. When the y coordinate is equal to my ground’s coordinate, I’ll prevent the object from falling through the floor.
        if self.going_down {
            self.y += 1;
            if self.y > 34 {
                self.going_down = false;
            }
        }

        // will check to see if the player should be jumping. On each tick, I’ll reduce 1 from my y coordinate to give the illusion of my object moving up. To prevent my box from jumping to infinity, I’ll set the going_down flag to true once the y value is less than the coordinate that represents the highest point the box can jump.
        if self.jumping {
            self.y -= 1;
            if self.y < 7 {
                self.going_down = true;
                self.jumping = false;
            }
        }
    }
}

// To abstract player’s jump mechanism, define a function to update my state’s variables as needed and prevent the player from double jumping. The function will take one parameter, which will be the state struct defined. Passing the ampersand with the mut keyword while declaring function’s parameter type, Indicate that the changes done by the code in this function should be propagated up the call stack.
fn jump(gs: &mut State) {
    // will ensure that the player can only jump while on the ground which is another instance of simulated physics
    if gs.jumping || gs.y < 34 {
        return;
    }
    gs.jumping = true
}

// determine which action to execute based on the variable passed in the statement. In this case, matching actions to keyboard keys and reading the keycode of the pressed key from context object’s key property. On match, the jump function defined earlier will be executed.
fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Up => jump(gs),
            VirtualKeyCode::Space => jump(gs),
            _ => {}
        },
    }
}

fn main() -> BError {
    //  will add a terminal window to my screen of size 80x50 and will have the title Hello Bracket World
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Bracket World")
        .build()?;

    // initialize my state data struct and set the defaults I want the game to start with
    let gs: State = State {
        y: 1,
        // Game over will be set to pause the games animation from happenning
        game_over: false,
        going_down: true,
        jumping: false,
    };

    // invoke the function and pass my state, so that it may be invoked on each tick, and my drawing canvas
    main_loop(context, gs)
}
