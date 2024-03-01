use ggez::graphics::{self, Color}
;

#[derive(Copy, Clone)]
pub struct BoardTile {
    pub x: u32,
    pub y: u32, 
    pub color: Color
}

impl BoardTile {

    pub fn empty() -> Self {
        Self {
            x: 0,
            y: 0,
            color: Color::BLACK,
        }
    }

    pub fn new(x: u32, y: u32, color: Color) -> Self {
        Self {
            x,
            y,
            color
        }
    }
}

impl From<BoardTile> for graphics::Rect {
    fn from(item: BoardTile) -> Self {
        graphics::Rect::new_i32(
            item.x as i32 * crate::TILE_SIZE as i32,
            item.y as i32 * crate::TILE_SIZE as i32,
            crate::TILE_SIZE as i32,
            crate::TILE_SIZE as i32,
        )
    }
}