use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::{Constraint, Offset}, prelude::{CrosstermBackend, Frame, Stylize, Terminal}, style::{Color, Style}, text::Text, widgets::{Cell, Paragraph, Row, Table}
};
use rusttype::Point;
use std::io::{stdout, Result};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;
const NUMBER_OF_TILES: usize = BOARD_WIDTH * BOARD_HEIGHT;
const TILE_SIZE: u16 = 2;

/*#[derive(Clone, Copy)]
enum Color {
    Black, White, Yellow, Blue, Purple, Red, Green, Orange, Cyan
}*/

enum Tetromino {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
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
}

struct AppContext {
    should_close: bool,
}

struct GameState {
    board: [Color; NUMBER_OF_TILES],
    current_tetromino: Tetromino,
    tetromino_position: Point<usize>,
    tetromino_set_to_board: bool,
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
        tetromino_position: Point {
            x: 0,
            y: 0,
        },
        tetromino_set_to_board: false,
    };

    loop {
        game_logic(&mut game_state, &mut app_context)?;
        terminal.draw(|frame| draw(&mut game_state, frame))?;
        if app_context.should_close {
            break;
        }
    }

    Ok(())
}

fn game_logic(game: &mut GameState, app_context: &mut AppContext) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250)).unwrap() {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press{
                match key.code {
                    KeyCode::Char('q') => app_context.should_close = true,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

//this is TUI for test purposes, probably will be reimplemented later
fn draw(game: &mut GameState, frame: &mut Frame) {
    let mut table_colors = game.board.clone();
    if !game.tetromino_set_to_board {
        let mut y_offset = game.tetromino_position.y * BOARD_WIDTH;
        for line in game.current_tetromino.shape() {
            let mut x_offset =  game.tetromino_position.x;
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
        Row::new(row.iter().map(|cell| {
            Cell::from(Text::from("")).style(Style::default().bg(*cell))
        }))
        .height(TILE_SIZE)
    });
    let table = Table::new(rows, widths).column_spacing(0);
    frame.render_widget(table, frame.size())
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
