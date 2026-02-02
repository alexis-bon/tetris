use std::cmp::Ordering;

use crate::game::{self, cell::Cell, game_action::GameAction, state::State};

const LINES_FULL_CHECKING_TIME: u128 = 10;
const DEFAULT_TETROMINO_FALLING_TIME: u128 = 80;
const DELTA_FALLING_TIME: u128 = 5;

impl State {
    fn move_current_tetromino_left(&mut self) {
        let collisions = self
            .get_current_tetromino_ref()
            .get_left_collisions_cell_indexes();

        if  collisions.contains(&None) ||
            self.is_collision_with_other_tetromino(collisions) {

            return
        }

        self.get_current_tetromino_mutref()
        .move_left();
    }

    fn move_current_tetromino_right(&mut self) {
        let collisions = self
            .get_current_tetromino_ref()
            .get_right_collisions_cell_indexes();

        if collisions.contains(&None)  ||
            self.is_collision_with_other_tetromino(collisions) {

            return
        }

        self.get_current_tetromino_mutref()
        .move_right();
    }

    fn move_current_tetromino_down(&mut self) {
        let collisions = self
            .get_current_tetromino_ref()
            .get_down_collisions_cell_indexes();

        if collisions.contains(&None) {
            self.stick_current_tetromino();
            return;
        }

        if self.is_collision_with_other_tetromino(collisions) {
            self.stick_current_tetromino();
        } else {
            self.get_current_tetromino_mutref()
            .move_down();
        }
    }

    fn is_collision_with_other_tetromino(&mut self, collisions: Vec<Option<usize>>) -> bool {
        for collision in collisions {
            if let Some(collision_index) = collision {
                if self.grid[collision_index] == Cell::Full {
                    return true;
                }
            }
        }

        false
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

    fn stick_current_tetromino(&mut self) {
        let old_tetromino_ref = self.get_current_tetromino_ref();
        let cells_coords = old_tetromino_ref.get_cells_coords();

        self.grid[cells_coords.0.to_grid_index()] = Cell::Full;
        self.grid[cells_coords.1.to_grid_index()] = Cell::Full;
        self.grid[cells_coords.2.to_grid_index()] = Cell::Full;
        self.grid[cells_coords.3.to_grid_index()] = Cell::Full;

        self.set_new_current_tetromino();
    }

    fn clear_grid_lines_full(&mut self) {
    for i in 0..game::GRID_HEIGHT {
        if self.is_grid_line_full(i) {
            self.clear_grid_line(i);
            self.shift_tetromino_cells_down(i);
        }
    }
}
}

pub fn perform_action(state: &mut State, action: GameAction) {
    match action {
        GameAction::Left => state.move_current_tetromino_left(),
        GameAction::Right => state.move_current_tetromino_right(),
        GameAction::Down => state.move_current_tetromino_down(),
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

    if state.get_clock() % LINES_FULL_CHECKING_TIME == 0 {
        state.clear_grid_lines_full();
    }
}