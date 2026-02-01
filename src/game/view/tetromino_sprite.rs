use crate::game::tetromino::Tetromino;
use crate::game::view::{self, cursor_positions};

pub struct TetrominoSprite {
    pub tetromino: Tetromino,
    pub cells_grid_position: (usize, usize, usize, usize)
}

const GRID_ORIGIN_I32 : i32 = cursor_positions::GRID_ORIGIN as i32;
const SCREEN_WIDTH_I32: i32 = view::SCREEN_WIDTH as i32;
const CELL_WIDTH_I32  : i32 = view::CELL_WIDTH as i32;

const SHAPES_J: [[(i32, i32); 3]; 4] = [
    [(-1,  0), ( 1, -1), ( 1,  0)],
    [( 0, -1), ( 0,  1), ( 1,  1)],
    [(-1,  0), (-1,  1), ( 1,  0)],
    [(-1, -1), ( 0, -1), ( 0,  1)]
];

const SHAPE_O: [(i32, i32); 3] =
    [( 0,  1),  ( 1,  0), ( 1,  1)];

impl TetrominoSprite {
    // pub fn new(tetromino: Tetromino, position: usize, rotation: usize) -> TetrominoSprite {
    //     TetrominoSprite {
    //         tetromino,
    //         cells: TetrominoSprite::get_cells_from_shape(
    //             position,
    //             TetrominoSprite::get_shape(tetromino, rotation)
    //         )
    //     }
    // }

    pub fn current_tetromino(
        tetromino: Tetromino,
        grid_position: (usize, usize),
        rotation: usize
    ) -> TetrominoSprite {

        TetrominoSprite {
            tetromino,
            cells_grid_position: Self::get_cells_from_shape(
                grid_position,
                Self::get_shape(tetromino, rotation)
            )
        }
    }

    fn get_shape(tetromino: Tetromino, rotation: usize) -> [(i32, i32); 3] {
        match tetromino {
            Tetromino::J => SHAPES_J[rotation],
            _ => SHAPE_O
        }
    }

    fn get_cells_from_shape(grid_center: (usize, usize), shape: [(i32, i32); 3])
        -> (usize, usize, usize, usize)
    {
        let grid_center_i32 = (grid_center.0 as i32, grid_center.1 as i32);
        let center_i32 = GRID_ORIGIN_I32
            + grid_center_i32.0 * SCREEN_WIDTH_I32
            + grid_center_i32.1 * CELL_WIDTH_I32;

        (
            (center_i32) as usize,
            (center_i32 + shape[0].0 * SCREEN_WIDTH_I32 + shape[0].1 * CELL_WIDTH_I32) as usize,
            (center_i32 + shape[1].0 * SCREEN_WIDTH_I32 + shape[1].1 * CELL_WIDTH_I32) as usize,
            (center_i32 + shape[2].0 * SCREEN_WIDTH_I32 + shape[2].1 * CELL_WIDTH_I32) as usize,
        )
    }
}