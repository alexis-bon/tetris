use crate::game::state::{CurrentTetromino, GridCoords};

#[derive(Copy, Clone)]
pub enum Tetromino {
    I, O, T, J, L, S, Z
}

const NB_TETROMINOS: u32 = 7;

const SHAPES_I: [[(i32, i32); 3]; 2] = [
    [( 0, -1), ( 0,  1), ( 0,  2)],
    [(-1,  0), ( 1,  0), ( 2,  0)]
];

const SHAPE_O: [(i32, i32); 3] =
    [(-1,  0), (-1,  1), ( 0,  1)];

const SHAPES_T: [[(i32, i32); 3]; 4] = [
    [( 0, -1), ( 0,  1), ( 1,  0)],
    [(-1,  0), ( 0,  1), ( 1,  0)],
    [(-1,  0), ( 0, -1), ( 0,  1)],
    [(-1 , 0), ( 0, -1), ( 1,  0)]
];

const SHAPES_J: [[(i32, i32); 3]; 4] = [
    [(-1,  0), ( 1, -1), ( 1,  0)],
    [( 0, -1), ( 0,  1), ( 1,  1)],
    [(-1,  0), (-1,  1), ( 1,  0)],
    [(-1, -1), ( 0, -1), ( 0,  1)]
];

const SHAPES_L: [[(i32, i32); 3]; 4] = [
    [(-1, -1), (-1,  0), ( 1,  0)],
    [(-1,  1), ( 0, -1), ( 0,  1)],
    [(-1,  0), ( 1,  0), ( 1,  1)],
    [( 0, -1), ( 0,  1), ( 1,  0)]
];

const SHAPES_S: [[(i32, i32); 3]; 2] = [
    [(-1,  0), (-1,  1), ( 0, -1)],
    [(-1,  0), ( 1,  0), ( 1,  1)]
];

const SHAPES_Z: [[(i32, i32); 3]; 2] = [
    [(-1, -1), (-1,  0), ( 0,  1)],
    [(-1,  1), ( 0,  1), ( 1,  0)]
];

impl Tetromino {
    /** Returns the shape used in sections HOLD and NEXT on the screen */
    pub fn get_display_shape(&self) -> [(i32, i32); 3] {
        match self {
            Tetromino::I => SHAPES_I[0],
            Tetromino::O => SHAPE_O,
            Tetromino::T => SHAPES_T[2],
            Tetromino::J => SHAPES_J[3],
            Tetromino::L => SHAPES_L[1],
            Tetromino::S => SHAPES_S[0],
            Tetromino::Z => SHAPES_Z[0]
        }
    }

    pub fn from_index(index: u32) -> Tetromino {
        match index % NB_TETROMINOS {
            0 => Tetromino::I,
            1 => Tetromino::O,
            2 => Tetromino::T,
            3 => Tetromino::J,
            4 => Tetromino::L,
            5 => Tetromino::S,
            6 => Tetromino::Z,
            _ => panic!("Maths are not good")
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

    /** Returns true if given cell is one of this tetromino */
    pub fn is_cell_part_of_me(&self, cell_coords: &GridCoords) -> bool {
        let cells_coords = self.get_cells_coords();

        cells_coords.0.equals(cell_coords) ||
        cells_coords.1.equals(cell_coords) ||
        cells_coords.2.equals(cell_coords) ||
        cells_coords.3.equals(cell_coords)
    }

    pub fn get_shape(&self) -> [(i32, i32); 3] {
        match self.get_tetromino() {
            Tetromino::I => SHAPES_I[self.get_rotation() % 2],
            Tetromino::O => SHAPE_O,
            Tetromino::T => SHAPES_T[self.get_rotation()],
            Tetromino::J => SHAPES_J[self.get_rotation()],
            Tetromino::L => SHAPES_L[self.get_rotation()],
            Tetromino::S => SHAPES_S[self.get_rotation() % 2],
            Tetromino::Z => SHAPES_Z[self.get_rotation() % 2]
        }
    }
}