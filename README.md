# tetris-rs

## About The Project

This is simple implementation of Tetris written in Rust.

![screenshot]


## Getting Started

To get a local copy up and running follow these steps.


### Prerequisites

* Rust > downloadable at https://rustup.rs/


### Installation

Clone the repo
   ```sh
   git clone https://github.com/marcelslom/tetris-rs.git
   ```
Go to the project directory
   ```sh
   cd tetris-rs
   ```
Build the project
   ```sh
   cargo build --release
   ```
Run the project
   ```sh
   cargo run --release
   ```


## Usage

The game starts as soon as the project is launched.


### Controls

<kbd>←</kbd> Move left

<kbd>→</kbd> Move right

<kbd>↑</kbd> Rotate clockwise

<kbd>Numpad 0</kbd> Rotate counterclockwise

<kbd>↓</kbd> Soft drop

<kbd>Space</kbd> Hard drop

<kbd>C</kbd> Hold

<kbd>Esc</kbd> Close


## Game adjustment

There are a few constants defined in the project, that can be adjusted.

In `main.rs` file:
   - `BOARD_WIDTH` - width of game board (measured in number of tiles);
   - `BOARD_HEIGHT` - height of game board (measured in number of tiles);
   - `TILE_SIZE` - width (and height, as tile is square) of one tile (measured in pixels);
   - `DESIRED_FPS` - tells the game how many times per second the game logic should be recalculated.

In `button_state.rs` file:
   - `HOLD_DURATION_MILLIS` - specifies the time that must elapse after a button is pressed down in order for the press to be regarded as long press.

In `game_state.rs` file:
   - `HORIZONTAL_GRAVITY_FACTOR` - value of horizontal gravity that is used to calculate horizontal tetromino speed when 'Move left' or 'Move right' button is hold (see <a href="#gravity">Gravity</a> for more info).


### Gravity

_Gravity_ can be thought of as the mechanism that makes the tetromino move down the board. Its unit is G and its value is how many tiles the tetromino would move down per frame. E.g. 1G = 1 tile per frame, 0.1G = 0.1 tile per frame => the tetromino would move down one row every 10 frames. Basically, _gravity_ is responsible for the speed at which the tetromino falls.

In `gravity.rs` file there are three values defined:
   - `Normal` - gravity value used when there is no player's interaction;
   - `SoftDrop` - gravity value for Soft Drop;
   - `HardDrop` - gravity value for Hard Drop, causes the tetromino to fall down the board almost immediately (should be changed when changing `BOARD_HEIGHT` value).

Please head to <a href="https://tetris.fandom.com/wiki/Drop#Gravity">Tetris Wiki</a> for more info.


## Tech stack

This game is written in <a href="https://www.rust-lang.org/">Rust</a> with <a href="https://ggez.rs/">ggez</a> game library.


[screenshot]: images/screenshot.png