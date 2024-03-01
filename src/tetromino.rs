use rusttype::Point;

use crate::{board_tile::BoardTile, rotation::{Rotation, RotationDirection}, tetromino_kind::TetrominoKind};

#[derive(Clone)]
pub struct Tetromino {
    pub kind: TetrominoKind, 
    pub position: Point<i32>,
    pub shape: Vec<Vec<bool>>,
    pub current_rotation: Rotation
}

impl Tetromino {
    pub fn new(kind: TetrominoKind) -> Self {
        Tetromino {
            kind,
            position: Point {x: 0, y: 0},
            shape: kind.shape(),
            current_rotation: Rotation::_0,
        }
    }

    pub fn random() -> Self {
        let random_kind: TetrominoKind = rand::random();
        Tetromino {
            kind: random_kind,
            position: Point {x: 0, y: 0},
            shape: random_kind.shape(),
            current_rotation: Rotation::_0,
        }
    }

    pub fn tiles(&self) -> Vec<BoardTile> {
        let capacity = self.shape.iter().map(|x| x.iter().filter(|&&xx| xx).count()).sum::<usize>();
        let mut tiles = Vec::<BoardTile>::with_capacity(capacity);
        let mut y = self.position.y;
        for row in &self.shape {
            let mut x = self.position.x;
            for item in row {
                if *item {
                    tiles.push(BoardTile::new(x as u32, y as u32, self.kind.color()))
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