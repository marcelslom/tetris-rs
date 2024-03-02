use rusttype::Point;
use ggez::graphics::Color;
use rand::{distributions::{Distribution, Standard}, Rng};

use crate::{board_tile::BoardTile, rotation::{Rotation, RotationDirection}};

#[derive(Clone)]
pub struct Tetromino {
    pub kind: TetrominoKind,
    pub color: Color,
    pub position: Point<i32>,
    pub shape: Vec<Vec<bool>>,
    pub current_rotation: Rotation, 
    pub is_ghost: bool
}

impl Tetromino {
    pub fn new(kind: TetrominoKind) -> Self {
        Tetromino {
            kind,
            color: kind.color(),
            position: Point {x: 0, y: 0},
            shape: kind.shape(),
            current_rotation: Rotation::_0,
            is_ghost: false
        }
    }

    pub fn random() -> Self {
        let random_kind: TetrominoKind = rand::random();
        Tetromino {
            kind: random_kind,
            color: random_kind.color(),
            position: Point {x: 0, y: 0},
            shape: random_kind.shape(),
            current_rotation: Rotation::_0,
            is_ghost: false
        }
    }

    pub fn to_ghost(&self) -> Self {
        let mut ghost = self.clone();
        ghost.color = Color::from_rgb(100, 100, 100);
        let last_row_index = ghost.shape.iter().enumerate().rev().find(|x| x.1.iter().any(|xx| *xx)).map(|x| x.0).unwrap();
        ghost.position.y = (crate::BOARD_HEIGHT - ghost.shape.len() + last_row_index) as i32;
        ghost.is_ghost = true;
        ghost
    }

    pub fn tiles(&self) -> Vec<BoardTile> {
        let capacity = self.shape.iter().map(|x| x.iter().filter(|&&xx| xx).count()).sum::<usize>();
        let mut tiles = Vec::<BoardTile>::with_capacity(capacity);
        let mut y = self.position.y;
        for row in &self.shape {
            let mut x = self.position.x;
            for item in row {
                if *item {
                    tiles.push(BoardTile::new(x as u32, y as u32, self.color))
                }
                x += 1;
            }
            y += 1;
        }
        tiles
    }

     pub fn rotate(&mut self, direction: RotationDirection) {
        match direction{
            RotationDirection::Clockwise => {
                let new_width = self.shape.len();
                let new_height = self.shape[0].len();
                let mut rotated: Vec<Vec<bool>> = Vec::with_capacity(new_height);
                for i in 0..new_height {
                    let mut new_row: Vec<bool> = Vec::with_capacity(new_width);
                    for old_row in self.shape.iter().rev() {
                        new_row.push(old_row[i]);
                    }
                    rotated.push(new_row);
                }
                self.shape = rotated;
                self.current_rotation = self.current_rotation.next(RotationDirection::Clockwise);
            },
            RotationDirection::CounterClockwise => {
                let new_width = self.shape.len();
                let new_height = self.shape[0].len();
                let mut rotated: Vec<Vec<bool>> = Vec::with_capacity(new_height);
                for i in 0..new_height {
                    let mut new_row: Vec<bool> = Vec::with_capacity(new_width);
                    for old_row in &self.shape {
                        new_row.push(old_row[new_height - i - 1]);
                    }
                    rotated.push(new_row);
                }
                self.shape = rotated;
                self.current_rotation = self.current_rotation.next(RotationDirection::CounterClockwise);
            }
        }
    }
}


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
    fn color(&self) -> Color {
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

    fn shape(&self) -> Vec<Vec<bool>> {
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