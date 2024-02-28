use ggez::graphics::Color;
use ggez::{
    event, graphics,
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};
use rusttype::Point;
use std::default;
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

static WALL_KICKS : [[Point<i32>; 5]; 8] = [
    [Point {x: 0, y: 0}, Point {x: -1, y: 0}, Point {x: -1, y: -1}, Point {x: 0, y: 2}, Point {x: -1, y: 2} ],
    [Point {x: 0, y: 0}, Point {x: 1, y: 0}, Point {x: 1, y: 1}, Point {x: 0, y: -2}, Point {x: 1, y: -2} ],

    [Point {x: 0, y: 0}, Point {x: 1, y: 0}, Point {x: 1, y: 1}, Point {x: 0, y: -2}, Point {x: 1, y: -2} ],
    [Point {x: 0, y: 0}, Point {x: -1, y: 0}, Point {x: -1, y: -1}, Point {x: 0, y: 2}, Point {x: -1, y: 2} ],

    [Point {x: 0, y: 0}, Point {x: 1, y: 0}, Point {x: 1, y: -1}, Point {x: 0, y: 2}, Point {x: 1, y: 2} ],
    [Point {x: 0, y: 0}, Point {x: -1, y: 0}, Point {x: -1, y: 1}, Point {x: 0, y: -2}, Point {x: -1, y: -2} ],

    [Point {x: 0, y: 0}, Point {x: -1, y: 0}, Point {x: -1, y: 1}, Point {x: 0, y: -2}, Point {x: -1, y: -2} ],
    [Point {x: 0, y: 0}, Point {x: 1, y: 0}, Point {x: 1, y: -1}, Point {x: 0, y: 2}, Point {x: 1, y: 2} ],
];

static I_WALL_KICKS : [[Point<i32>; 5]; 8] = [
    [Point {x: 0, y: 0}, Point {x: -2, y: 0}, Point {x: 1, y: 0}, Point {x: -2, y: 1}, Point {x: 1, y: -2} ],
    [Point {x: 0, y: 0}, Point {x: 2, y: 0}, Point {x: -1, y: 0}, Point {x: 2, y: -1}, Point {x: -1, y: 2} ],

    [Point {x: 0, y: 0}, Point {x: -1, y: 0}, Point {x: 2, y: 0}, Point {x: -1, y: -2}, Point {x: 2, y: 1} ],
    [Point {x: 0, y: 0}, Point {x: 1, y: 0}, Point {x: -2, y: 0}, Point {x: 1, y: 2}, Point {x: -2, y: -1} ],

    [Point {x: 0, y: 0}, Point {x: 2, y: 0}, Point {x: -1, y: 0}, Point {x: 2, y: -1}, Point {x: -1, y: 2} ],
    [Point {x: 0, y: 0}, Point {x: -2, y: 0}, Point {x: 1, y: 0}, Point {x: -2, y: 1}, Point {x: 1, y: -2} ],

    [Point {x: 0, y: 0}, Point {x: 1, y: 0}, Point {x: -2, y: 0}, Point {x: 1, y: 2}, Point {x: -2, y: -1} ],
    [Point {x: 0, y: 0}, Point {x: -1, y: 0}, Point {x: 2, y: 0}, Point {x: -1, y: -2}, Point {x: 2, y: 1} ],
];


#[derive(Clone, Copy, PartialEq)]
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

#[derive(Copy, Clone)]
enum Rotation {
    _0 = 0,
    R = 1,
    _2 = 2,
    L = 3
}

impl Rotation {

    fn next(&self, direction: RotationDirection) -> Self {
        match direction {
            RotationDirection::Clockwise => Self::next_clockwise(*self),
            RotationDirection::CounterClockwise => Self::next_counter_clockwise(*self),
        }
    }

    fn next_clockwise(rotation: Rotation) -> Self {
        match rotation {
            Rotation::_0 => Rotation::R,
            Rotation::R => Rotation::_2,
            Rotation::_2 => Rotation::L,
            Rotation::L => Rotation::_0,
        }
    }

    fn next_counter_clockwise(rotation: Rotation) -> Self {
        match rotation {
            Rotation::_0 => Rotation::L,
            Rotation::R => Rotation::_0,
            Rotation::_2 => Rotation::R,
            Rotation::L => Rotation::_2,
        }
    }
    
}

#[derive(Copy, Clone, PartialEq)]
enum RotationDirection {
    Clockwise, CounterClockwise
}

#[derive(Clone)]
struct Tetromino {
    kind: TetrominoKind, 
    position: Point<i32>,
    shape: Vec<Vec<bool>>,
    current_rotation: Rotation

}

impl Tetromino {
    fn new(kind: TetrominoKind) -> Self {
        Tetromino {
            kind,
            position: Point {x: 0, y: 0},
            shape: kind.shape(),
            current_rotation: Rotation::_0,
        }
    }

    fn random() -> Self {
        let random_kind: TetrominoKind = rand::random();
        Tetromino {
            kind: random_kind,
            position: Point {x: 0, y: 0},
            shape: random_kind.shape(),
            current_rotation: Rotation::_0,
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
                    tiles.push(BoardTile::new(x as u32, y as u32, self.kind.color()))
                }
                x += 1;
            }
            y += 1;
        }
        tiles
    }

     fn rotate(&mut self, direction: RotationDirection) {
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

#[derive(Copy, Clone)]
struct BoardTile {
    x: u32,
    y: u32, 
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

    fn new(x: u32, y: u32, color: Color) -> Self {
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
    rotate_clockwise_button_state: ButtonState,
    rotate_counterclockwise_button_state: ButtonState,
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

    fn should_handle_once(&self) -> bool {
        self.key_down_was_noticed && ! self.handled
    }

    fn handled_once(&mut self) {
        self.handled = true;
    }

    fn is_short_pressed(&self) -> bool {
        match self.pressed_duration {
            Some(duration) => duration + Duration::from_millis(Self::HOLD_DURATION_MILLIS) > Instant::now(),
            None => false
        }
    }

    fn is_long_pressed(&self) -> bool {
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
            board[i].x = x as u32;
            board[i].y = y as u32;
        }
        Self {
            board,
            current_vertical_action: VerticalAction::None,
            tetromino: Tetromino::new(TetrominoKind::T),
            vertical_gravity: 0f32,
            horizontal_gravity: 0f32,
            left_button_state: ButtonState::new(),
            right_button_state: ButtonState::new(),
            rotate_clockwise_button_state: ButtonState::new(),
            rotate_counterclockwise_button_state: ButtonState::new(),
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
                    self.board[BOARD_WIDTH * y as usize + x as usize].color = self.tetromino.kind.color();
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
            TetrominoKind::I => I_WALL_KICKS,
            _ => WALL_KICKS
        };

        let mut index: usize = 0;
        if direction == RotationDirection::Clockwise {
            index = 2 * start as usize;
        } else {
            let finish  = start.next(RotationDirection::CounterClockwise);
            index = 2 * finish as usize + 1;
        }

        table[index]
    }

    fn update_game(&mut self) {
        self.handle_rotation();
        self.handle_vertical();
        self.handle_horizontal();
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
                    if x < 0 || x >= BOARD_WIDTH.try_into().unwrap() || y < 0 || y >= BOARD_HEIGHT.try_into().unwrap() {
                        return false;
                    }
                    if board[(y as usize) * BOARD_WIDTH + (x as usize)].color != Color::BLACK {
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
            KeyCode::Up => self.rotate_clockwise_button_state.key_down(),
            KeyCode::Numpad0 => self.rotate_counterclockwise_button_state.key_down(),
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
            KeyCode::Up => self.rotate_clockwise_button_state.key_up(),
            KeyCode::Numpad0 => self.rotate_counterclockwise_button_state.key_up(),
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
