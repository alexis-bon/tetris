use crate::game::{game_action::GameAction, state::{CurrentTetromino, State}};

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

        self.get_current_tetromino_mutref().move_left();
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

        self.get_current_tetromino_mutref().move_right();
    }
}

pub fn perform_action(state: &mut State, action: GameAction) {
    match action {
        GameAction::Left => state.move_current_tetromino_left(),
        GameAction::Right => state.move_current_tetromino_right(),
        GameAction::Rotate => state.increment_rotation(),
        _ => ()
    }
}