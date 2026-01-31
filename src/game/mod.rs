mod core;
mod view;

const  GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 20;
const GRID_LENGTH: usize = GRID_HEIGHT * GRID_WIDTH;

const NEXT_TETROMINOS_QUEUE_SIZE: usize = 3;

mod tetromino;
mod cell_color;

mod state;

pub fn start_game() {
    let mut state = state::State::new();
    state.increment_level();
    println!("Level {}", state.get_level());
}