use std::cmp::Ordering;

use crate::game::{self, cell::Cell, game_action::GameAction, state::State};

const LINES_FULL_CHECKING_TIME: u128 = 10;
const DEFAULT_TETROMINO_FALLING_TIME: u128 = 80;
const DELTA_FALLING_TIME: u128 = 5;

const EARNED_POINTS_SINGLE: u32 = 10;
const EARNED_POINTS_DOUBLE: u32 = 30;
const EARNED_POINTS_TRIPLE: u32 = 50;
const EARNED_POINTS_TETRIS: u32 = 80;

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

        self.set_next_tetromino_to_current();
    }

    fn clear_grid_lines_full(&mut self) {
        let old_lines_counter = self.get_lines();

        for i in 0..game::GRID_HEIGHT {
            if self.is_grid_line_full(i) {
                self.clear_grid_line(i);
                self.shift_tetromino_cells_down(i);
            }
        }

        let new_lines_counter = self.get_lines();

        match new_lines_counter - old_lines_counter {
            1 => self.add_to_score(EARNED_POINTS_SINGLE * self.get_level()),
            2 => self.add_to_score(EARNED_POINTS_DOUBLE * self.get_level()),
            3 => self.add_to_score(EARNED_POINTS_TRIPLE * self.get_level()),
            4 => self.add_to_score(EARNED_POINTS_TETRIS * self.get_level()),
            _ => ()
        }

        if old_lines_counter / 10 < new_lines_counter / 10 {
            self.increment_level();
        }
    }

    fn swap_current_stored_tetrominos(&mut self) {
        if self.can_store() {

            let old_stored = self.get_stored_tetromino();
            self.set_stored_tetromino(Some(self.get_current_tetromino()));
            
            if let Some(tetromino) = old_stored {
                self.set_new_current_tetromino(tetromino);
                self.set_can_store_flag(false);
            } else {
                self.set_next_tetromino_to_current();
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