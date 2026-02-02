use std::{cell, vec};

use crate::game::{self, state::{CurrentTetromino, GridCoords}};

#[derive(Copy, Clone)]
pub enum Tetromino {
    I, O, T, J, L, S, Z
}

const SHAPES_J: [[(i32, i32); 3]; 4] = [
    [(-1,  0), ( 1, -1), ( 1,  0)],
    [( 0, -1), ( 0,  1), ( 1,  1)],
    [(-1,  0), (-1,  1), ( 1,  0)],
    [(-1, -1), ( 0, -1), ( 0,  1)]
];

const SHAPE_O: [(i32, i32); 3] =
    [(-1,  0), (-1,  1), ( 0,  1)];

impl Tetromino {
    /** Returns the shape used in sections HOLD and NEXT on the screen */
    pub fn get_display_shape(&self) -> [(i32, i32); 3] {
        match self {
            Tetromino::J => SHAPES_J[3],
            _ => SHAPE_O
        }
    }
}

impl CurrentTetromino {
    pub fn get_cells_coords(&self) -> (GridCoords, GridCoords, GridCoords, GridCoords) {
        let shape = self.get_shape();
        let center = self.get_position();
        let center_i32_i = center.i as i32;
        let center_i32_j = center.j as i32;

        let coords_cell0 = GridCoords {
            i: (center_i32_i + shape[0].0) as usize,
            j: (center_i32_j + shape[0].1) as usize
        };

        let coords_cell1 = GridCoords {
            i: (center_i32_i + shape[1].0) as usize,
            j: (center_i32_j + shape[1].1) as usize
        };

        let coords_cell2 = GridCoords {
            i: (center_i32_i + shape[2].0) as usize,
            j: (center_i32_j + shape[2].1) as usize
        };

        (center, coords_cell0, coords_cell1, coords_cell2)
    }

    pub fn is_cell_part_of_me(&self, cell_coords: &GridCoords) -> bool {
        let cells_coords = self.get_cells_coords();

        cells_coords.0.equals(cell_coords) ||
        cells_coords.1.equals(cell_coords) ||
        cells_coords.2.equals(cell_coords) ||
        cells_coords.3.equals(cell_coords)
    }

    pub fn get_shape(&self) -> [(i32, i32); 3] {
        match self.get_tetromino() {
            Tetromino::J => SHAPES_J[self.get_rotation()],
            _ => SHAPE_O
        }
    }
}