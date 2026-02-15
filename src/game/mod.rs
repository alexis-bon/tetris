
mod core;
mod view;

const  GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 20;
const GRID_LENGTH: usize = GRID_HEIGHT * GRID_WIDTH;

const NEXT_TETROMINOS_QUEUE_SIZE: usize = 3;

const SLEEP_TIME_BETWEEN_FRAMES_MILLIS: u64 = 10;

mod tetromino;
mod tetromino_collision;
mod cell;
mod game_action;

mod state;

use std::io;
use std::time::Duration;

use crate::game::game_action::GameAction;

pub fn start_game() -> Result<(), String> {
    let mut state = state::State::new();

    let mut view_struct = match view::initialize_view() {
        Ok (view_struct) => view_struct,
        Err(e) => return Err(e.to_string()),
    };

    loop {

        if let Some(next_action) = view::input::read() {
            match next_action {
                GameAction::Quit => break,
                GameAction::Pause => state.flip_paused_flag(),
                _ => {
                    if !state.is_game_paused() {
                        core::perform_action(&mut state, next_action);
                    }
                }
            }
        }

        if !state.is_game_paused() {
            core::increment_clock_and_trigger_events(&mut state);
        }

        match view::display_state(&state, &mut view_struct) {
            Ok(_) => (),
            io::Result::Err(e) => return Err(e.to_string())
        }

        std::thread::sleep(Duration::from_millis(SLEEP_TIME_BETWEEN_FRAMES_MILLIS));
    }
    match view::close_view(&mut view_struct) {
        Ok(_) => (),
        io::Result::Err(e) => return Err(e.to_string())
    };

    Ok(())
}