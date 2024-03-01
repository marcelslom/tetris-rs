use ggez::graphics::Color;
use rand::{distributions::{Distribution, Standard}, Rng};


#[derive(Clone, Copy, PartialEq)]
pub enum TetrominoKind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl TetrominoKind {
    pub fn color(&self) -> Color {
        match self {
            TetrominoKind::I => Color::CYAN,
            TetrominoKind::O => Color::YELLOW,
            TetrominoKind::T => Color::from_rgb(0xa0, 0x20, 0xf0), //Purple,
            TetrominoKind::S => Color::GREEN,
            TetrominoKind::Z => Color::RED,
            TetrominoKind::J => Color::BLUE,
            TetrominoKind::L => Color::from_rgb(0xff, 0xa5, 0x00), //Orange,
        }
    }

    pub fn shape(&self) -> Vec<Vec<bool>> {
        match self {
            TetrominoKind::I => vec![vec![false, false, false, false], vec![true, true, true, true], vec![false, false, false, false], vec![false, false, false, false]],
            TetrominoKind::O => vec![vec![true, true], vec![true, true]],
            TetrominoKind::T => vec![vec![false, true, false], vec![true, true, true], vec![false, false, false]],
            TetrominoKind::S => vec![vec![false, true, true], vec![true, true, false], vec![false, false, false]],
            TetrominoKind::Z => vec![vec![true, true, false], vec![false, true, true], vec![false, false, false]],
            TetrominoKind::J => vec![vec![true, false, false], vec![true, true, true], vec![false, false, false]],
            TetrominoKind::L => vec![vec![false, false, true], vec![true, true, true], vec![false, false, false]],
        }
    }
}

impl Distribution<TetrominoKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetrominoKind {
        match rng.gen_range(0..=6) {
            0 => TetrominoKind::I,
            1 => TetrominoKind::O,
            2 => TetrominoKind::T,
            3 => TetrominoKind::S,
            4 => TetrominoKind::Z,
            5 => TetrominoKind::J,
            6 => TetrominoKind::L,
            _ => panic!("Tetromino distribution - out of range")
        }
    }
}