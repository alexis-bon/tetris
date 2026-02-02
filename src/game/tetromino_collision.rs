use crate::game;
use crate::game::state::{CurrentTetromino, GridCoords};

impl CurrentTetromino {
    pub fn get_left_collisions_cell_indexes(&self) -> Vec<Option<usize>> {
        self.get_collisions_cell_indexes((0, -1))
    }

    pub fn get_right_collisions_cell_indexes(&self) -> Vec<Option<usize>> {
        self.get_collisions_cell_indexes((0, 1))
    }

    pub fn get_down_collisions_cell_indexes(&self) -> Vec<Option<usize>> {
        self.get_collisions_cell_indexes((0, 1))
    }

    fn get_collisions_cell_indexes(&self, direction: (i32, i32)) -> Vec<Option<usize>> {
        let cells_coords = self.get_cells_coords();
        let mut collisions = Vec::new();
        
        let possible_neighboor0 = GridCoords {
            i: ((cells_coords.0.i) as i32 + direction.0) as usize,
            j: ((cells_coords.0.j) as i32 + direction.1) as usize
        };
        self.add_to_collisions_if_valid(&mut collisions, possible_neighboor0);

        let possible_neighboor1 = GridCoords {
            i: ((cells_coords.1.i) as i32 + direction.0) as usize,
            j: ((cells_coords.1.j) as i32 + direction.1) as usize
        };
        self.add_to_collisions_if_valid(&mut collisions, possible_neighboor1);

        let possible_neighboor2 = GridCoords {
            i: ((cells_coords.2.i) as i32 + direction.0) as usize,
            j: ((cells_coords.2.j) as i32 + direction.1) as usize
        };
        self.add_to_collisions_if_valid(&mut collisions, possible_neighboor2);

        let possible_neighboor3 = GridCoords {
            i: ((cells_coords.3.i) as i32 + direction.0) as usize,
            j: ((cells_coords.3.j) as i32 + direction.1) as usize
        };
        self.add_to_collisions_if_valid(&mut collisions, possible_neighboor3);

        collisions
    }

    fn add_to_collisions_if_valid(
        &self,
        collisions: &mut Vec<Option<usize>>,
        possible_neighboor: GridCoords
    ) {
        if self.is_cell_part_of_me(&possible_neighboor) {
            return
        }

        if possible_neighboor.i >= game::GRID_HEIGHT
            || possible_neighboor.j >= game::GRID_WIDTH {
            
            collisions.push(None)
        } else {
            collisions.push(Some(possible_neighboor.to_grid_index()));
        }
    }

}