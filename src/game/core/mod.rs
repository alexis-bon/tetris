use crate::game::{game_action::GameAction, state::State};

impl State {
    pub fn increment_level(&mut self) {
        self.set_level(self.get_level() + 1);
    }

    pub fn decrement_level(&mut self) {
        let l = self.get_level();
        self.set_level(if l > 0 {l - 1} else {l});
    }
}

pub fn perform_action(state: &mut State, action: GameAction) {
    match action {
        GameAction::Left => state.decrement_level(),
        GameAction::Right => state.increment_level(),
        _ => ()
    }
}