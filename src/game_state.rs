use crate::{board_tile::BoardTile, 
    button_state::ButtonState, 
    gravity::Gravity, 
    rotation::{Rotation, RotationDirection},
    tetromino::{Tetromino, TetrominoKind}, 
    wall_kicks};
use ggez::graphics::{self, Color};
use rusttype::Point;


pub struct GameState {
    board: [BoardTile; crate::NUMBER_OF_TILES],
    pub left_button_state: ButtonState,
    pub right_button_state: ButtonState,
    pub rotate_clockwise_button_state: ButtonState,
    pub rotate_counterclockwise_button_state: ButtonState,
    pub hard_drop_button_state: ButtonState,
    pub soft_drop_button_state: ButtonState,
    pub hold_button_state: ButtonState,
    tetromino: Tetromino,
    ghost: Option<Tetromino>,
    vertical_gravity: f32,
    horizontal_gravity: f32,
    game_over: bool
}

impl GameState {

    const HORIZONTAL_GRAVITY_FACTOR: f32 = 0.25f32;

    pub fn new() -> Self {
        let mut board = [BoardTile::empty(); crate::NUMBER_OF_TILES];
        for i in 0..crate::NUMBER_OF_TILES {
            let y = i / crate::BOARD_WIDTH;
            let x = i - y * crate::BOARD_WIDTH;
            board[i].x = x as u32;
            board[i].y = y as u32;
        }
        let tetromino = Tetromino::new(TetrominoKind::T);
        let ghost = Some(tetromino.to_ghost());
        Self {
            board,
            tetromino,
            ghost,
            vertical_gravity: 0f32,
            horizontal_gravity: 0f32,
            left_button_state: ButtonState::new(),
            right_button_state: ButtonState::new(),
            rotate_clockwise_button_state: ButtonState::new(),
            rotate_counterclockwise_button_state: ButtonState::new(),
            hard_drop_button_state: ButtonState::new(),
            soft_drop_button_state: ButtonState::new(),
            hold_button_state: ButtonState::new(),
            game_over: false
        }
    }

    fn move_tetromino_to_board(&mut self) {
        let shape = &self.tetromino.shape;
        let mut y = self.tetromino.position.y;
        for row in shape {
            let mut x = self.tetromino.position.x;
            for item in row {
                if *item {
                    self.board[crate::BOARD_WIDTH * y as usize + x as usize].color = self.tetromino.color;
                }
                x += 1;
            }
            y += 1;
        }
    }

    fn new_tetromino(&mut self) {
        self.tetromino = Tetromino::random();
        self.ghost = Some(self.tetromino.to_ghost());
        self.vertical_gravity = 0f32;
    }

    fn remove_full_rows(board: &mut [BoardTile]) {
        for row_number in 1..crate::BOARD_HEIGHT {
            let start_index = row_number * crate::BOARD_WIDTH;
            let end_index = start_index + crate::BOARD_WIDTH;
            let is_full = board[start_index..end_index].iter().all(|x| x.color != Color::BLACK);
            if is_full {
                for i in (0..start_index).rev() {
                    board[i + crate::BOARD_WIDTH].color = board[i].color;
                }
                for i in 0..crate::BOARD_WIDTH {
                    board[i].color = Color::BLACK;
                }
            }

        }
    }

    pub fn hold(&self) -> bool {
        self.hold_button_state.is_pressed()
    }

    fn handle_vertical(&mut self) {
        if self.hard_drop_button_state.should_handle_once() {
            self.vertical_gravity = Gravity::HardDrop.value();
            self.hard_drop_button_state.handled_once();
        } else if self.soft_drop_button_state.is_pressed() {
            self.vertical_gravity += Gravity::SoftDrop.value();
        } else {
            self.vertical_gravity += Gravity::Normal.value();
        }
    }

    fn handle_horizontal(&mut self) {
        if self.left_button_state.should_handle_once() {
            self.horizontal_gravity = -1f32;
            self.left_button_state.handled_once();
        } else if self.left_button_state.is_long_pressed() {
            self.horizontal_gravity -= GameState::HORIZONTAL_GRAVITY_FACTOR;
        }
        if self.right_button_state.should_handle_once() {
            self.horizontal_gravity = 1f32;
            self.right_button_state.handled_once();
        } else if self.right_button_state.is_long_pressed() {
            self.horizontal_gravity += GameState::HORIZONTAL_GRAVITY_FACTOR;
        }
    }

    fn handle_rotation(&mut self) {
        if self.tetromino.kind == TetrominoKind::O {
            return;
        }
        if self.rotate_clockwise_button_state.should_handle_once() {
            self.try_rotate(RotationDirection::Clockwise);
            self.rotate_clockwise_button_state.handled_once();
        }
        if self.rotate_counterclockwise_button_state.should_handle_once() {
            self.try_rotate(RotationDirection::CounterClockwise);
            self.rotate_counterclockwise_button_state.handled_once();
        }
    }

    fn try_rotate(&mut self, direction: RotationDirection) {
        let mut clone = self.tetromino.clone();
        let wall_kicks = Self::get_wall_kick_vectors(clone.kind, clone.current_rotation, direction);
        clone.rotate(direction);
        for kick in wall_kicks {
            if Self::can_move(&clone, &self.board, kick) {
                clone.position.x += kick.x;
                clone.position.y += kick.y;
                self.tetromino = clone;
                break;
            }   
        }
    }
    
    fn get_wall_kick_vectors(tetromino: TetrominoKind, start: Rotation, direction: RotationDirection) -> [Point<i32>; 5] {
        let table = match tetromino {
            TetrominoKind::O => panic!("'O' tetromino does not support rotation and does not have any wall kick vector"),
            TetrominoKind::I => wall_kicks::I_WALL_KICKS,
            _ => wall_kicks::WALL_KICKS
        };

        let index: usize;
        if direction == RotationDirection::Clockwise {
            index = 2 * start as usize
        } else {
            let finish  = start.next(RotationDirection::CounterClockwise);
            index = 2 * finish as usize + 1
        }

        table[index]
    }

    pub fn update_game(&mut self) {
        if self.game_over {
            return;
        }
        self.handle_rotation();
        self.handle_vertical();
        self.handle_horizontal();
        let round_finish = self.move_tetromino();
        if round_finish {
            self.move_tetromino_to_board();
            Self::remove_full_rows(&mut self.board);
        }

        if self.board[0..crate::BOARD_WIDTH].iter().any(|x| x.color != Color::BLACK) {
            self.game_over = true;
            return;
        }
        
        if round_finish {
            self.new_tetromino();
        }

        let mut ghost = self.tetromino.to_ghost();
        while Self::can_move(&ghost, &self.board, Point {x: 0, y: 1}) {
            ghost.position.y += 1;
        }
        self.ghost = Some(ghost)
    }

    pub fn draw_game(&self, canvas: &mut graphics::Canvas) {
        if self.game_over {
            Self::draw_text("GAME", 0.45f32, canvas);
            Self::draw_text("OVER", 0.55f32, canvas);
            return;
        }

        for seg in self.board {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(seg.into())
                    .color(seg.color)
            );
        }

        if self.ghost.is_some() {
            let tiles = self.ghost.as_ref().unwrap().tiles();
            for tile in tiles{
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(tile.into())
                        .color(tile.color),
                );
            }
        }

        let tetromino_tiles = self.tetromino.tiles();
        for tile in tetromino_tiles{
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(tile.into())
                    .color(tile.color),
            );
        }
    }

    fn draw_text(text: &str, height_percentage: f32, canvas: &mut graphics::Canvas) {
        let mut text = graphics::Text::new(text);
        text.set_layout(graphics::TextLayout::center());
        text.set_scale(32f32);
        let width = (crate::BOARD_WIDTH * crate::TILE_SIZE) as f32 / 2f32;
        let height = (crate::BOARD_HEIGHT * crate::TILE_SIZE) as f32 * height_percentage;
        let game_draw_param = graphics::DrawParam::from([width, height]).color(Color::WHITE);
        canvas.draw(&text, game_draw_param);
    }

    fn move_tetromino(&mut self) -> bool {
        self.move_horizontally();
        self.move_vertically()
    }

    fn move_horizontally(&mut self) {
        if self.horizontal_gravity >= 1f32 {
            while self.horizontal_gravity >= 1f32 {
                if !Self::can_move(&self.tetromino, &self.board, Point {x: 1, y: 0})  {
                    self.horizontal_gravity = 0f32;
                    return;
                }
                self.tetromino.position.x += 1;
                self.horizontal_gravity -= 1f32;
            }
            self.horizontal_gravity = 0f32;
        } else if self.horizontal_gravity <= -1f32 {
            while self.horizontal_gravity <= -1f32 {
                if !Self::can_move(&self.tetromino, &self.board, Point {x: -1, y: 0})  {
                    self.horizontal_gravity = 0f32;
                    return;
                }
                self.tetromino.position.x -= 1;
                self.horizontal_gravity += 1f32;
            }
            self.horizontal_gravity = 0f32;
        }
    }

    fn move_vertically(&mut self) -> bool {
        if self.vertical_gravity >= 1f32 {
            //move tetromino down
            while self.vertical_gravity >= 1f32 {
                if !Self::can_move(&self.tetromino, &self.board, Point {x: 0, y: 1}) {
                    self.vertical_gravity = 0f32;
                    return true;
                }
                self.tetromino.position.y += 1;
                self.vertical_gravity -= 1f32;
            }
            self.vertical_gravity = 0f32; // reset gravity to avoid errors related to the cumulation of fractional parts.
        }
        false
    }

    fn can_move(tetromino: &Tetromino, board: &[BoardTile], offset_vector: Point<i32> ) -> bool {
        let mut y = tetromino.position.y as i32 +tetromino.shape.len() as i32 + offset_vector.y - 1;
        
        for row in tetromino.shape.iter().rev() {
            let mut x = tetromino.position.x as i32 + offset_vector.x;
            for tile in row {
                if *tile {
                    if x < 0 || x >= crate::BOARD_WIDTH.try_into().unwrap() || y < 0 || y >= crate::BOARD_HEIGHT.try_into().unwrap() {
                        return false;
                    }
                    if board[(y as usize) * crate::BOARD_WIDTH + (x as usize)].color != Color::BLACK {
                        return false;
                    }
                }
                x += 1;
            }
            y -= 1;
        }

        true
    }


}