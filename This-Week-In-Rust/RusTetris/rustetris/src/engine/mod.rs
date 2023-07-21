use self::piece::{Kind as PieceKind, Piece, Rotation};
use cgmath::{Point2, Vector2};
use core::panic;
use rand::{
    prelude::{SliceRandom, ThreadRng},
    thread_rng,
};
use std::{
    ops::{Index, IndexMut},
    time::Duration,
};

pub mod piece;

type Coordinate = Point2<usize>;
type Offset = cgmath::Vector2<isize>;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MoveKind {
    Left,
    Right,
}

impl MoveKind {
    fn offset(&self) -> Offset {
        match self {
            MoveKind::Left => Offset::new(-1, 0),
            MoveKind::Right => Offset::new(1, 0),
        }
    }
}

pub struct Engine {
    matrix: Matrix,
    bag: Vec<PieceKind>,
    rng: ThreadRng,
    cursor: Option<Piece>,
    level: u8,
}

impl Engine {
    pub fn DEBUG_test_cursor(&mut self, kind: PieceKind, position: Offset) {
        let piece = Piece {
            kind,
            rotation: Rotation::N,
            position,
        };

        self.cursor = Some(piece)
    }

    pub fn cells(&self) -> CellIter<'_> {}

    fn cursor_has_hit_bottom(&self) -> bool {
        self.cursor.is_some() && self.ticked_down_cursor().is_none()
    }

    pub fn drop_time(&self) -> Duration {
        let level_index = self.level - 1;
        let seconds_per_line = (0.8 - (level_index as f32 * 0.007)).powi(level_index as _);
        Duration::from_secs_f32(seconds_per_line)
    }

    pub fn hard_drop(&mut self) {
        let cursor = self.ticked_down_cursor().unwrap();

        while let Some(new) = self.ticked_down_cursor() {
            self.cursor = Some(new);
        }
        self.place_cursor();
    }

    pub fn move_cursor(&mut self, kind: MoveKind) -> Result<(), ()> {
        let Some(cursor) = self.cursor.as_mut() else { return Ok(()); };
        let new = cursor.moved_by(kind.offset());

        if self.matrix.is_clipping(&new) {
            return Err(());
        }

        self.cursor = Some(new);
        Ok(())
    }

    pub fn new() -> Self {
        Engine {
            matrix: Matrix::blank(),
            bag: Vec::new(),
            rng: thread_rng(),
            cursor: None,
            level: 1,
        }
    }

    fn place_cursor(&mut self) {
        let cursor = self
            .cursor
            .take()
            .expect("Called place_cursor without a cursor");

        debug_assert!(
            self.matrix.is_placeable(&cursor),
            "Cursor placed onto invalid position: {:?}",
            cursor
        );

        let color = cursor.kind.color();
        for coord in cursor.cells().unwrap() {
            self.matrix[coord] = Some(color);
        }
    }

    fn refill_bag(&mut self) {
        debug_assert!(self.bag.is_empty());
        self.bag.extend_from_slice(PieceKind::ALL.as_slice());
        self.bag.shuffle(&mut self.rng);
    }

    pub fn tick_down(&mut self) {
        self.cursor = Some(self.ticked_down_cursor().unwrap())
    }

    fn ticked_down_cursor(&self) -> Option<Piece> {
        let Some(cursor) = self.cursor else {return None; };
        let new = cursor.moved_by(Offset::new(0, -1));
        (!self.matrix.is_clipping(&new)).then_some(new)
    }

    pub fn with_matrix(matrix: Matrix) -> Self {
        todo!()
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Yellow,
    Cyan,
    Purple,
    Orange,
    Blue,
    Green,
    Red,
}

pub struct Matrix([Option<Color>; Self::SIZE]);

impl Matrix {
    pub const WIDTH: usize = 10;
    pub const HEIGHT: usize = 20;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    pub fn DEBUG_cursor_location(&mut self, kind: PieceKind, location: Coordinate) {
        todo!()
    }

    pub fn blank() -> Self {
        Self([None; Self::SIZE])
    }

    fn in_bounds(Coordinate { x, y }: Coordinate) -> bool {
        x < Self::WIDTH && y < Self::HEIGHT
    }

    fn indexing(Coordinate { x, y }: Coordinate) -> usize {
        y * Self::WIDTH + x
    }

    fn is_clipping(&self, piece: &Piece) -> bool {
        let Some(cells) = piece.cells() else { return true};
        cells
            .into_iter()
            .any(|coord| !Matrix::on_matrix(coord) || self[coord].is_some())
    }

    fn is_placeable(&self, piece: &Piece) -> bool {
        let Some(cells) = piece.cells() else { return false};
        cells
            .into_iter()
            .all(|coord| Matrix::on_matrix(coord) && self[coord].is_none())
    }

    fn on_matrix(coord: Coordinate) -> bool {
        Self::valid_coord(coord) && coord.y < Self::HEIGHT
    }

    fn valid_coord(coord: Coordinate) -> bool {
        coord.x < Self::WIDTH
    }
}

impl Index<Coordinate> for Matrix {
    type Output = Option<Color>;

    fn index(&self, coord: Coordinate) -> &Self::Output {
        assert!(Self::on_matrix(coord));
        &self.0[Self::indexing(coord)]
    }
}

impl IndexMut<Coordinate> for Matrix {
    fn index_mut(&mut self, coord: Coordinate) -> &mut Self::Output {
        assert!(Self::on_matrix(coord));
        &mut self.0[Self::indexing(coord)]
    }
}

pub struct CellIter<'matrix> {}

impl<'matrix> Iterator for Cell<'matrix> {
    type Item = (Coordinate, &'matrix Option<Color>);

    fn next(&mut self) -> Option<Self::Item> {
        let Some(cell) = self.cells.next() else {
        };

        let coord = self.position;
        self.position.grid_inc();

        Some((coord, cell))
    }
}
