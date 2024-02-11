use ggez::graphics::Color;
use ggez::{
    event, graphics,
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};
use rusttype::Point;
use std::io::{stdout, Result};
use std::thread;
use std::time::{Duration, Instant};

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
enum Tetromino {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
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
            Gravity::SoftDrop => todo!(),
            Gravity::HardDrop => todo!(),
        }
    }
}

impl Tetromino {
    fn color(&self) -> Color {
        match self {
            Tetromino::I => Color::CYAN,
            Tetromino::O => Color::YELLOW,
            Tetromino::T => Color::from_rgb(0xa0, 0x20, 0xf0), //Purple,
            Tetromino::S => Color::GREEN,
            Tetromino::Z => Color::RED,
            Tetromino::J => Color::BLUE,
            Tetromino::L => Color::from_rgb(0xff, 0xa5, 0x00), //Orange,
        }
    }

    fn shape(&self) -> Vec<Vec<bool>> {
        match self {
            Tetromino::I => vec![vec![true, true, true, true]],
            Tetromino::O => vec![vec![true, true], vec![true, true]],
            Tetromino::T => vec![vec![false, true, false], vec![true, true, true]],
            Tetromino::S => vec![vec![false, true, true], vec![true, true, false]],
            Tetromino::Z => vec![vec![true, true, false], vec![false, true, true]],
            Tetromino::J => vec![vec![true, false, false], vec![true, true, true]],
            Tetromino::L => vec![vec![false, false, true], vec![true, true, true]],
        }
    }

    fn width(&self) -> usize {
        match self {
            Tetromino::I => 4,
            Tetromino::O => 2,
            Tetromino::S => 3,
            Tetromino::Z => 3,
            Tetromino::T => 3,
            Tetromino::J => 3,
            Tetromino::L => 3,
        }
    }

    fn tiles(&self, start_x: usize, start_y: usize) -> Vec<BoardTile> {
        let shape = self.shape();
        let capacity = shape.iter().map(|x| x.iter().filter(|&&xx| xx).count()).sum::<usize>();
        let mut tiles = Vec::<BoardTile>::with_capacity(capacity);
        let mut x = start_x;
        let mut y = start_y;
        for row in shape {
            x = start_x;
            for item in row {
                if item {
                    tiles.push(BoardTile::new(x, y, self.color()))
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
    current_tetromino: Tetromino,
    tetromino_position: Point<usize>,
    tetromino_finished: bool,
    vertical_gravity: f32,
    horizontal_gravity: f32,
}

struct ButtonState {
    pressed_duration: Option<Instant>,
    handled: bool,
    key_down_was_noticed: bool
}

impl ButtonState {

    const HOLD_DURATION_MILLIS: u64 = 2000; //todo change this value to make button hold more natural

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
            current_tetromino: Tetromino::I,
            tetromino_position: Point { x: 0, y: 0 },
            tetromino_finished: false,
            vertical_gravity: 0f32,
            horizontal_gravity: 0f32,
            left_button_state: ButtonState::new(),
            right_button_state: ButtonState::new()
        }
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
        self.move_tetromino();
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
        let tetromino_tiles = self.current_tetromino.tiles(self.tetromino_position.x, self.tetromino_position.y);
        for tile in tetromino_tiles{
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(tile.into())
                    .color(tile.color),
            );
        }
    }

    fn move_tetromino(&mut self) {
        if self.vertical_gravity >= 1f32 {
            //move tetromino down
            while self.vertical_gravity >= 1f32 {
                //todo check collision
                self.tetromino_position.y += 1;
                self.vertical_gravity -= 1f32;
            }
            self.vertical_gravity = 0f32; // reset gravity to avoid errors related to the cumulation of fractional parts.
        }

        if self.horizontal_gravity >= 1f32 {
            while self.horizontal_gravity >= 1f32 {
                //todo check collision
                self.tetromino_position.x += 1;
                self.horizontal_gravity -= 1f32;
            }
            self.horizontal_gravity = 0f32;
        } else if self.horizontal_gravity <= -1f32 {
            while self.horizontal_gravity <= -1f32 {
                //todo check collision
                self.tetromino_position.x -= 1;
                self.horizontal_gravity += 1f32;
            }
            self.horizontal_gravity = 0f32;
        }
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
