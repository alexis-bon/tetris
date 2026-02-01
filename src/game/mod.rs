
mod core;
mod view;

const  GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 20;
const GRID_LENGTH: usize = GRID_HEIGHT * GRID_WIDTH;

const NEXT_TETROMINOS_QUEUE_SIZE: usize = 3;

mod tetromino;
mod cell;

mod state;

use std::io;
use std::time::Duration;


pub fn start_game() -> Result<(), String> {
    let mut state = state::State::new();

    let mut view_struct = match view::initialize_view("data/screen.txt") {
        Ok (view_struct) => view_struct,
        Err(e) => return Err(e.to_string()),
    };

    loop {
        match view::display_state(&state, &mut view_struct) {
            Ok(_) => (),
            io::Result::Err(e) => return Err(e.to_string())
        }

        state.increment_level();

        std::thread::sleep(Duration::from_millis(40));
    }
    match view::close_view(&mut view_struct) {
        Ok(_) => (),
        io::Result::Err(e) => return Err(e.to_string())
    };

    Ok(())
}