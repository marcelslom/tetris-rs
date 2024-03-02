mod game_state;
use game_state::GameState;

mod board_tile;
mod tetromino;
mod vertical_action;
mod button_state;
mod gravity;
mod rotation;
mod wall_kicks;

use vertical_action::VerticalAction;

use ggez::{
    event, graphics,
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};


const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;
const NUMBER_OF_TILES: usize = BOARD_WIDTH * BOARD_HEIGHT;
const TILE_SIZE: u16 = 20;

const SCREEN_SIZE: (f32, f32) = (
    BOARD_WIDTH as f32 * TILE_SIZE as f32,
    BOARD_HEIGHT as f32 * TILE_SIZE as f32,
);

const DESIRED_FPS: u32 = 60;


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

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> std::prelude::v1::Result<(), ggez::GameError> {
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

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> std::prelude::v1::Result<(), ggez::GameError> {
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
