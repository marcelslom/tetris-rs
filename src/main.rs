use ggez::graphics::Color;
use ggez::{
    event, graphics,
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};
use rusttype::Point;
use std::time::{Duration, Instant};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;
const NUMBER_OF_TILES: usize = BOARD_WIDTH * BOARD_HEIGHT;
const TILE_SIZE: u16 = 20;

const SCREEN_SIZE: (f32, f32) = (
    BOARD_WIDTH as f32 * TILE_SIZE as f32,
    BOARD_HEIGHT as f32 * TILE_SIZE as f32,
);

const DESIRED_FPS: u32 = 60;

#[derive(Clone, Copy)]
enum TetrominoKind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
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

#[derive(Clone, Copy)]
enum VerticalAction {
    None,
    SoftDrop,
    HardDrop,
    Hold
}

#[derive(Clone, Copy)]
enum RotationAction {
    None,
    RotateClockwise,
    RotateCounterClockwise
}

#[derive(Clone, Copy)]
enum Gravity {
    None,
    Normal,
    SoftDrop,
    HardDrop,
}

impl Gravity {
    fn value(&self) -> f32 {
        match self {
            Gravity::None => 0f32,
            Gravity::Normal => 1f32 / 64f32,
            Gravity::SoftDrop => 1f32,
            Gravity::HardDrop => 20f32,
        }
    }
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
            TetrominoKind::I => vec![vec![true, true, true, true]],
            TetrominoKind::O => vec![vec![true, true], vec![true, true]],
            TetrominoKind::T => vec![vec![false, true, false], vec![true, true, true]],
            TetrominoKind::S => vec![vec![false, true, true], vec![true, true, false]],
            TetrominoKind::Z => vec![vec![true, true, false], vec![false, true, true]],
            TetrominoKind::J => vec![vec![true, false, false], vec![true, true, true]],
            TetrominoKind::L => vec![vec![false, false, true], vec![true, true, true]],
        }
    }

    fn width(&self) -> usize {
        match self {
            TetrominoKind::I => 4,
            TetrominoKind::O => 2,
            TetrominoKind::S => 3,
            TetrominoKind::Z => 3,
            TetrominoKind::T => 3,
            TetrominoKind::J => 3,
            TetrominoKind::L => 3,
        }
    }

    fn height(&self) -> usize {
        match self {
            TetrominoKind::I => 1,
            TetrominoKind::O => 2,
            TetrominoKind::S => 2,
            TetrominoKind::Z => 2,
            TetrominoKind::T => 2,
            TetrominoKind::J => 2,
            TetrominoKind::L => 2,
        }
    }
}

struct Tetromino {
    kind: TetrominoKind, 
    position: Point<usize>,
    shape: Vec<Vec<bool>>
}

impl Tetromino {
    fn new(kind: TetrominoKind) -> Self {
        Tetromino {
            kind,
            position: Point {x: 0, y: 0},
            shape: kind.shape()
        }
    }

    fn random() -> Self {
        let random_kind: TetrominoKind = rand::random();
        Tetromino {
            kind: random_kind,
            position: Point {x: 0, y: 0},
            shape: random_kind.shape()
        }
    }

    fn tiles(&self) -> Vec<BoardTile> {
        let capacity = self.shape.iter().map(|x| x.iter().filter(|&&xx| xx).count()).sum::<usize>();
        let mut tiles = Vec::<BoardTile>::with_capacity(capacity);
        let mut x = self.position.x;
        let mut y = self.position.y;
        for row in &self.shape {
            x = self.position.x;
            for item in row {
                if *item {
                    tiles.push(BoardTile::new(x, y, self.kind.color()))
                }
                x += 1;
            }
            y += 1;
        }
        tiles
    }
}

#[derive(Copy, Clone)]
struct BoardTile {
    x: usize,
    y: usize, 
    color: Color
}

impl BoardTile {

    fn empty() -> Self {
        Self {
            x: 0,
            y: 0,
            color: Color::BLACK,
        }
    }

    fn new(x: usize, y: usize, color: Color) -> Self {
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
            item.x as i32 * TILE_SIZE as i32,
            item.y as i32 * TILE_SIZE as i32,
            TILE_SIZE as i32,
            TILE_SIZE as i32,
        )
    }
}

struct GameState {
    board: [BoardTile; NUMBER_OF_TILES],
    current_vertical_action: VerticalAction,
    left_button_state: ButtonState,
    right_button_state: ButtonState,
    current_rotation_action: RotationAction,
    tetromino: Tetromino,
    vertical_gravity: f32,
    horizontal_gravity: f32,
}

struct ButtonState {
    pressed_duration: Option<Instant>,
    handled: bool,
    key_down_was_noticed: bool
}

impl ButtonState {

    const HOLD_DURATION_MILLIS: u64 = 500; //todo change this value to make button hold more natural

    fn new() -> Self {
        Self {
            pressed_duration: None,
            handled: false,
            key_down_was_noticed: false
        }
    }

    fn key_down(&mut self) {
        if self.key_down_was_noticed {
            return;
        }
        self.pressed_duration = Some(Instant::now());
        self.key_down_was_noticed = true;
    }

    fn key_up(&mut self) {
        self.pressed_duration = None;
        self.handled = false;
        self.key_down_was_noticed = false;
    }

    fn is_pressed(&self) -> bool {
        match self.pressed_duration {
            Some(duration) => duration + Duration::from_millis(Self::HOLD_DURATION_MILLIS) > Instant::now(),
            None => false
        }
    }

    fn is_hold(&self) -> bool {
        match self.pressed_duration {
            Some(duration) => duration + Duration::from_millis(Self::HOLD_DURATION_MILLIS) <= Instant::now(),
            None => false
        }
    }

}

impl GameState {

    const HORIZONTAL_GRAVITY_FACTOR: f32 = 0.25f32;

    fn new() -> Self {
        let mut board = [BoardTile::empty(); NUMBER_OF_TILES];
        for i in 0..NUMBER_OF_TILES {
            let y = i / BOARD_WIDTH;
            let x = i - y * BOARD_WIDTH;
            board[i].x = x;
            board[i].y = y;
        }
        Self {
            board,
            current_vertical_action: VerticalAction::None,
            current_rotation_action: RotationAction::None,
            tetromino: Tetromino::new(TetrominoKind::I),
            vertical_gravity: 0f32,
            horizontal_gravity: 0f32,
            left_button_state: ButtonState::new(),
            right_button_state: ButtonState::new()
        }
    }

    fn move_tetromino_to_board(&mut self) {
        let shape = &self.tetromino.shape;
        let mut x = self.tetromino.position.x;
        let mut y = self.tetromino.position.y;
        for row in shape {
            x = self.tetromino.position.x;
            for item in row {
                if *item {
                    self.board[BOARD_WIDTH * y + x].color = self.tetromino.kind.color();
                }
                x += 1;
            }
            y += 1;
        }
    }

    fn new_tetromino(&mut self) {
        self.tetromino = Tetromino::random();
        self.vertical_gravity = 0f32;
    }

    fn remove_full_rows(board: &mut [BoardTile]) {
        for row_number in 1..BOARD_HEIGHT {
            let start_index = row_number * BOARD_WIDTH;
            let end_index = start_index + BOARD_WIDTH;
            let is_full = board[start_index..end_index].iter().all(|x| x.color != Color::BLACK);
            if is_full {
                for i in (0..start_index).rev() {
                    board[i + BOARD_WIDTH].color = board[i].color;
                }
                for i in 0..BOARD_WIDTH {
                    board[i].color = Color::BLACK;
                }
            }

        }
    }

    fn finish_round(&mut self) {
        self.move_tetromino_to_board();
        Self::remove_full_rows(&mut self.board);
        self.new_tetromino();
    }

    fn hold(&self) -> bool {
        matches!(self.current_vertical_action, VerticalAction::Hold)
    }

    fn handle_vertical(&mut self) {
        self.vertical_gravity = match self.current_vertical_action {
            VerticalAction::None => self.vertical_gravity + Gravity::Normal.value(),
            VerticalAction::SoftDrop => Gravity::SoftDrop.value(),
            VerticalAction::HardDrop => Gravity::HardDrop.value(),
            _ => todo!(),
        };
    }

    fn handle_horizontal(&mut self) {
        if self.left_button_state.is_pressed() && !self.left_button_state.handled {
            self.horizontal_gravity = -1f32;
            self.left_button_state.handled = true;
        } else if self.left_button_state.is_hold() {
            self.horizontal_gravity -= GameState::HORIZONTAL_GRAVITY_FACTOR;
        }
        if self.right_button_state.is_pressed() && !self.right_button_state.handled {
            self.horizontal_gravity = 1f32;
            self.right_button_state.handled = true;
        } else if self.right_button_state.is_hold() {
            self.horizontal_gravity += GameState::HORIZONTAL_GRAVITY_FACTOR;
        }
    }

    fn handle_rotation(&self) {
        todo!();
        self.current_rotation_action = RotationAction::None;
    }

    fn update_game(&mut self) {
        self.handle_vertical();
        self.handle_horizontal();
       // self.handle_rotation();
        let vertical_collision = self.move_tetromino();
        if vertical_collision {
            self.finish_round();
        }
    }

    fn draw_game(&self, canvas: &mut graphics::Canvas) {
        for seg in self.board {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(seg.into())
                    .color(seg.color),
            );
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

    fn move_tetromino(&mut self) -> bool {
        self.move_horizontally();
        self.move_vertically()
    }

    fn move_horizontally(&mut self) {
        if self.horizontal_gravity >= 1f32 {
            while self.horizontal_gravity >= 1f32 {
                if self.horizontal_collision_right() {
                    self.horizontal_gravity = 0f32;
                    return;
                }
                self.tetromino.position.x += 1;
                self.horizontal_gravity -= 1f32;
            }
            self.horizontal_gravity = 0f32;
        } else if self.horizontal_gravity <= -1f32 {
            while self.horizontal_gravity <= -1f32 {
                if self.horizontal_collision_left() {
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
                if self.vertical_collision() {
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

    fn horizontal_collision_left(&self) -> bool {
        if self.tetromino.position.x <= 0 {
            return true;
        }

        let mut counter = 0;
        for row in &self.tetromino.shape {
            let offset = row.iter().position(|x| *x);
            if offset.is_none() {
                continue;
            }
            let offset = offset.unwrap();
            if self.board[BOARD_WIDTH * (self.tetromino.position.y + counter) + self.tetromino.position.x + offset - 1].color != Color::BLACK {
                return true;
            }
            counter += 1;
        }

        false
    }

    fn horizontal_collision_right(&self) -> bool {
        if self.tetromino.position.x + self.tetromino.kind.width() >= BOARD_WIDTH {
            return true;
        }

        let mut counter = 0;
        for row in &self.tetromino.shape {
            let offset = row.iter().rposition(|x| *x);
            if offset.is_none() {
                continue;
            }
            let offset = offset.unwrap();
            if self.board[BOARD_WIDTH * (self.tetromino.position.y + counter) + self.tetromino.position.x + self.tetromino.kind.width() - offset + 1].color != Color::BLACK {
                return true;
            }
            counter += 1;
        }

        false
    }

    fn vertical_collision(&self) -> bool {
        if self.tetromino.position.y + self.tetromino.kind.height() >= BOARD_HEIGHT {
            return true;
        }

        for position in 0..self.tetromino.kind.width() {
            let offset = self.tetromino.shape.iter().rposition(|x| x[position]);
            if offset.is_none() {
                continue;
            }
            let row_number = self.tetromino.position.y + offset.unwrap() + 1;
            let column_number = self.tetromino.position.x + position;
            if self.board[BOARD_WIDTH * row_number + column_number].color != Color::BLACK {
                return true;
            }
        }

        false
    }


}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> std::prelude::v1::Result<(), ggez::GameError> {
        while ctx.time.check_update_time(DESIRED_FPS) {
            if self.hold() {
                continue;
            }
            self.update_game();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> std::prelude::v1::Result<(), ggez::GameError> {
            let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));
    
            self.draw_game(&mut canvas);
    
            canvas.finish(ctx)?;
            ggez::timer::yield_now();
            Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, repeated: bool) -> std::prelude::v1::Result<(), ggez::GameError> {
        let keycode = input.keycode.unwrap();
        match keycode {
            KeyCode::Escape => ctx.request_quit(),
            KeyCode::Up => self.current_rotation_action = RotationAction::RotateClockwise,
            KeyCode::Numpad0 => self.current_rotation_action = RotationAction::RotateCounterClockwise,
            KeyCode::Down => self.current_vertical_action = VerticalAction::SoftDrop,
            KeyCode::Space => self.current_vertical_action = VerticalAction::HardDrop,
            KeyCode::C => self.current_vertical_action = VerticalAction::Hold,
            KeyCode::Left => self.left_button_state.key_down(),
            KeyCode::Right => self.right_button_state.key_down(),
            _ => {}
        }

        Ok(())
    }

    fn key_up_event(&mut self, ctx: &mut Context, input: KeyInput) -> std::prelude::v1::Result<(), ggez::GameError> {
        let keycode = input.keycode.unwrap();
        match keycode {
            KeyCode::Down => self.current_vertical_action = VerticalAction::None,
            KeyCode::Space => self.current_vertical_action = VerticalAction::None,
            KeyCode::C => self.current_vertical_action = VerticalAction::None,
            KeyCode::Left => self.left_button_state.key_up(),
            KeyCode::Right => self.right_button_state.key_up(),
            _ => {}
        }

        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, events_loop) = ggez::ContextBuilder::new("tetris", "author")
        .window_setup(ggez::conf::WindowSetup::default().title("Tetris!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = GameState::new();
    event::run(ctx, events_loop, state)
}
