use std::cmp::Ordering;

use crate::game::{game_action::GameAction, state::{State}};

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