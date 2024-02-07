use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::Constraint,
    prelude::{CrosstermBackend, Frame, Terminal},
    style::{Color, Style},
    text::Text,
    widgets::{Cell, Row, Table},
};
use rusttype::Point;
use std::{any::Any, io::{stdout, Result}};
use std::time::{Duration, Instant};
use std::thread;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;
const NUMBER_OF_TILES: usize = BOARD_WIDTH * BOARD_HEIGHT;
const TILE_SIZE: u16 = 2;
const EVENT_POLL_DURATION_MS: u64 = 1;
const FRAME_DURATION: Duration = Duration::from_micros(16776);

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
enum Action {
    None,
    SoftDrop,
    HardDrop,
    Hold,
    Left,
    Right,
    RotateClockwise,
    RotateCounterClockwise,
    CloseGame,
}

#[derive(Clone, Copy)]
enum Gravity {
    None,
    Normal,
    SoftDrop,
    HardDrop
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
            Tetromino::I => Color::Cyan,
            Tetromino::O => Color::Yellow,
            Tetromino::T => Color::Magenta, //Color::Purple,
            Tetromino::S => Color::Green,
            Tetromino::Z => Color::Red,
            Tetromino::J => Color::Blue,
            Tetromino::L => Color::LightMagenta, //Color::Orange,
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
}

struct AppContext {
    should_close: bool,
}

struct GameState {
    board: [Color; NUMBER_OF_TILES],
    current_tetromino: Tetromino,
    tetromino_position: Point<usize>,
    tetromino_finished: bool,
    cumulated_gravity: f32
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

    let mut app_context = AppContext {
        should_close: false,
    };
    let mut game_state = GameState {
        board: [Color::Black; NUMBER_OF_TILES],
        current_tetromino: Tetromino::I,
        tetromino_position: Point { x: 0, y: 0 },
        tetromino_finished: false,
        cumulated_gravity: 0f32
    };

    loop {
        let frame_start = Instant::now();
        let input = read_input()?;
        app_logic(&mut app_context, input);
        if app_context.should_close {
            break;
        }
        game_logic(&mut game_state, input);
        terminal.draw(|frame| draw(&mut game_state, frame))?;
        let elapsed = frame_start.elapsed();
        if elapsed < FRAME_DURATION {
            let time_to_wait = FRAME_DURATION - elapsed;
            thread::sleep(time_to_wait);
        }
    }

    Ok(())
}

fn read_input() -> Result<Action> {
    if event::poll(std::time::Duration::from_millis(EVENT_POLL_DURATION_MS))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                return match key.code {
                    KeyCode::Char('q') => Ok(Action::CloseGame),
                    KeyCode::Up => Ok(Action::RotateClockwise),
                    KeyCode::Char('0')=> Ok(Action::RotateCounterClockwise),
                    KeyCode::Down => Ok(Action::SoftDrop),
                    KeyCode::Char(' ') => Ok(Action::HardDrop),
                    KeyCode::Char('c') => Ok(Action::Hold),
                    KeyCode::Left => Ok(Action::Left),
                    KeyCode::Right => Ok(Action::Right),
                    _ => Ok(Action::None),
                };
            }
        }
    }
    Ok(Action::None)
}

fn app_logic(context: &mut AppContext, input: Action) {
    if matches!(input, Action::CloseGame) {
        context.should_close = true;
    }
}

fn game_logic(game: &mut GameState, input: Action) {
    match input {
        Action::None => game.cumulated_gravity += Gravity::Normal.value(),
        Action::SoftDrop => game.cumulated_gravity = Gravity::SoftDrop.value(),
        Action::HardDrop => game.cumulated_gravity = Gravity::HardDrop.value(),
        Action::Hold => return,
        Action::Left => {
            game.cumulated_gravity += Gravity::Normal.value();
            if game.tetromino_position.x > 0 {
                game.tetromino_position.x -= 1;
            }
        },
        Action::Right => {
            game.cumulated_gravity += Gravity::Normal.value();
            if game.tetromino_position.x < BOARD_WIDTH - game.current_tetromino.width() {
                game.tetromino_position.x += 1;
            }
        },
        Action::RotateClockwise => todo!(),
        Action::RotateCounterClockwise => todo!(),
        Action::CloseGame => {},
    }

    if game.cumulated_gravity > 1f32 { //move tetromino down
        while game.cumulated_gravity > 1f32 {
            //todo check collision
            game.tetromino_position.y += 1;
            game.cumulated_gravity -= 1f32;
        }
        game.cumulated_gravity = 0f32; // reset gravity to avoid errors related to the cumulation of fractional parts.
    }

}

//this is TUI for test purposes, probably will be reimplemented later
fn draw(game: &mut GameState, frame: &mut Frame) {
    let mut table_colors = game.board.clone();
    if !game.tetromino_finished {
        let mut y_offset = game.tetromino_position.y * BOARD_WIDTH;
        for line in game.current_tetromino.shape() {
            let mut x_offset = game.tetromino_position.x;
            for value in line {
                if value {
                    table_colors[y_offset + x_offset] = game.current_tetromino.color();
                }
                x_offset += 1;
            }
            y_offset += BOARD_WIDTH;
        }
    }

    let widths = vec![Constraint::Length(TILE_SIZE); BOARD_WIDTH];
    let rows = table_colors.chunks(BOARD_WIDTH).map(|row| {
        Row::new(
            row.iter()
                .map(|cell| Cell::from(Text::from("")).style(Style::default().bg(*cell))),
        )
        .height(TILE_SIZE)
    });
    let table = Table::new(rows, widths).column_spacing(0);

    frame.render_widget(table, frame.size());
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
