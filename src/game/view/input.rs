use std::time::Duration;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::game::game_action::GameAction;

pub fn read() -> Option<GameAction> {
    if let Ok(is_event) = event::poll(Duration::from_millis(0)) {
        if is_event {
            if let Ok(Event::Key(key_event)) = event::read() {
                if key_event.kind == KeyEventKind::Press {
                    
                    return match key_event.code {
                        KeyCode::Left => Some(GameAction::Left),
                        KeyCode::Right => Some(GameAction::Right),
                        KeyCode::Down => Some(GameAction::Down),
                        KeyCode::Char('r') => Some(GameAction::Rotate),
                        KeyCode::Char('s') => Some(GameAction::Store),
                        KeyCode::Char('p') => Some(GameAction::Pause),
                        KeyCode::Char('q') => Some(GameAction::Quit),
                        _ => None
                    }
                }
            }
        }

    }

    None
}