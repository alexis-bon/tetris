use std::cmp::Ordering;

use crate::game::{game_action::GameAction, state::{State}};

const DEFAULT_TETROMINO_FALLING_TIME: u128 = 25;
const DELTA_FALLING_TIME: u128 = 5;

impl State {
    fn move_current_tetromino_left(&mut self) {
        let collisions = self
            .get_current_tetromino_ref()
            .get_left_collisions_cell_indexes();

        if collisions.contains(&None) {
            return
        }

        // TODO : test if some collisions contains a full cell,
        //        and if so stick the current tetromino

        self
            .get_current_tetromino_mutref()
            .move_left();
    }

    fn move_current_tetromino_right(&mut self) {
        let collisions = self
            .get_current_tetromino_ref()
            .get_right_collisions_cell_indexes();

        if collisions.contains(&None) {
            return
        }

        // TODO : test if some collisions contains a full cell,
        //        and if so stick the current tetromino

        self
            .get_current_tetromino_mutref()
            .move_right();
    }

    fn move_current_tetromino_down(&mut self) {
        let collisions = self
            .get_current_tetromino_ref()
            .get_down_collisions_cell_indexes();

        if collisions.contains(&None) {
            return
        }

        // TODO : test if some collisions contains a full cell,
        //        and if so stick the current tetromino

        self.get_current_tetromino_mutref()
            .move_down();
    }

    fn rotate_current_tetromino(&mut self) {
        self.increment_rotation();
        
        // Verify that the rotated tetromino doesn't clip in the grid border
        match self
                .get_current_tetromino_ref()
                .is_tetromino_clipping_horizontaly() {
            
            Ordering::Less => self.move_current_tetromino_right(),
            Ordering::Greater => self.move_current_tetromino_left(),
            Ordering::Equal => ()
        }

        if self
            .get_current_tetromino_ref()
            .is_tetromino_clipping_vertically() {

            self
                .get_current_tetromino_mutref()
                .move_up();
        }

    }
}

pub fn perform_action(state: &mut State, action: GameAction) {
    match action {
        GameAction::Left => state.move_current_tetromino_left(),
        GameAction::Right => state.move_current_tetromino_right(),
        GameAction::Rotate => state.rotate_current_tetromino(),
        _ => ()
    }
}

pub fn increment_clock_and_trigger_events(state: &mut State) {
    state.increment_clock();

    if state.get_clock() %
        (DEFAULT_TETROMINO_FALLING_TIME - state.get_level() as u128 * DELTA_FALLING_TIME) == 0 {

        state.move_current_tetromino_down();
    }
}