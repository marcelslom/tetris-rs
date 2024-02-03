use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal, Frame},
    widgets::Paragraph,
};
use rusttype::Point;
use std::io::{stdout, Result};

const BOARD_WIDTH:usize = 12;
const BOARD_HEIGHT:usize = 20;

#[derive(Clone, Copy)]
enum Color {
    Black, White, Yellow, Blue, Purple, Red, Green
}

enum Tetromino {
    I, O, T, S, Z, J, L
}

struct AppState {
    should_close: bool
}

struct GameState {
    board: [[Color; BOARD_WIDTH]; BOARD_HEIGHT],
    current_tetromino: Tetromino,
    tetromino_position: Point<usize>,
    tetromino_set_to_board: bool
}

fn main() -> Result<()> {
    startup()?;
    let result = game_loop();
    shutdown()?;
    result
}

fn game_loop() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    terminal.clear()?;

    let mut app_state = AppState {should_close: false };
    let mut game_state = GameState { board: [[Color::Black; BOARD_WIDTH]; BOARD_HEIGHT], current_tetromino: Tetromino::I, tetromino_position: Point { x: BOARD_WIDTH / 2, y: 0 }, tetromino_set_to_board: false };

    loop {

        game_logic()?;
        terminal.draw(|frame| draw(&mut game_state, frame))?;
        if app_state.should_close {
            break;
        }
    }

    Ok(())
}

fn game_logic() -> Result<()> {
   /* if event::poll(std::time::Duration::from_millis(250)).unwrap() {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press{
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('j') => counter -= 1,
                    KeyCode::Char('k') => counter += 1,
                    _ => {}
                }
            }
        }
    } */
    Ok(())
}

fn draw(game: &mut GameState, frame: &mut Frame) {

}

fn startup() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    Ok(())
}

fn shutdown() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
