use crate::game::{game_action::GameAction, state::State};

pub fn perform_action(state: &mut State, action: GameAction) {
    match action {
        GameAction::Left => state.decrement_level(),
        GameAction::Right => state.increment_level(),
        GameAction::Rotate => state.increment_rotation(),
        GameAction::Store => state.store_current_tetromino(),
        _ => ()
    }
}