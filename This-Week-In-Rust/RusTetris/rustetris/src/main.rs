#![allow(dead_code)]
#![allow(warnings, unused)]
#![feature(let_else, bool_to_option)]

use cgmath::Vector2;
use engine::{piece::Kind as PieceKind, Color, Engine, Matrix};

mod engine;
mod interface;

fn main() {
    let mut matrix = Matrix::blank();
    let mut engine = Engine::with_matrix(matrix);

    matrix[(1, 1).into()] = Some(Color::Green);
    engine.DEBUG_test_cursor(PieceKind::T, (5, 5).into());
    interface::run(engine)
}
