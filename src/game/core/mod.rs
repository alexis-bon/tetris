mod core;

use crate::game::{game_action::GameAction, state::State};

const LINES_FULL_CHECKING_TIME: u128 = 10;
const DEFAULT_TETROMINO_FALLING_TIME: u128 = 80;
const DELTA_FALLING_TIME: u128 = 5;

pub fn perform_action(state: &mut State, action: GameAction) {
    match action {
        GameAction::Left => state.move_current_tetromino_left(),
        GameAction::Right => state.move_current_tetromino_right(),
        GameAction::Down => state.move_current_tetromino_down(),
        GameAction::Rotate => state.rotate_current_tetromino(),
        GameAction::Store => state.swap_current_stored_tetrominos(),
        _ => ()
    }
}

pub fn increment_clock_and_trigger_events(state: &mut State) {
    state.increment_clock();

    if state.get_clock() %
        (DEFAULT_TETROMINO_FALLING_TIME - state.get_level() as u128 * DELTA_FALLING_TIME) == 0 {

        state.move_current_tetromino_down();
    }

    if state.get_clock() % LINES_FULL_CHECKING_TIME == 0 {
        state.clear_grid_lines_full();
    }
}