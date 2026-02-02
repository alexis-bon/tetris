use crate::game::state::GridCoords;
use crate::game::{state::CurrentTetromino, tetromino::Tetromino};
use crate::game::view::{self, cursor_positions};

pub struct TetrominoSprite {
    pub tetromino: Tetromino,
    pub cells_screen_position: (Option<usize>, Option<usize>, Option<usize>, Option<usize>)
}

const GRID_ORIGIN_I32 : i32 = cursor_positions::GRID_ORIGIN as i32;
const SCREEN_WIDTH_I32: i32 = view::SCREEN_WIDTH as i32;
const CELL_WIDTH_I32  : i32 = view::CELL_WIDTH as i32;

impl TetrominoSprite {
    pub fn display_sprite(tetromino: Tetromino, center_screen_position: usize)
        -> TetrominoSprite {
        
        TetrominoSprite {
            tetromino,
            cells_screen_position: Self::get_screen_cells_from_shape(
                    center_screen_position,
                    tetromino.get_display_shape()
                )
        }
    }

    pub fn of_current_tetromino(
        current_tetromino: &CurrentTetromino
    ) -> TetrominoSprite {

        TetrominoSprite {
            tetromino: current_tetromino.get_tetromino(),
            cells_screen_position: Self::get_screen_cells_from_shape(
                current_tetromino.get_position().to_screen_index(),
                current_tetromino.get_shape()
            )
        }
    }

    //  fn get_grid_cells_from_shape(grid_center: (usize, usize), shape: [(i32, i32); 3])
    //     -> (usize, usize, usize, usize)
    // {
    //     let grid_center_i32 = (grid_center.0 as i32, grid_center.1 as i32);
    //     let center_i32 = GRID_ORIGIN_I32
    //         + grid_center_i32.0 * SCREEN_WIDTH_I32
    //         + grid_center_i32.1 * CELL_WIDTH_I32;

    //     (
    //         (center_i32) as usize,
    //         (center_i32 + shape[0].0 * SCREEN_WIDTH_I32 + shape[0].1 * CELL_WIDTH_I32) as usize,
    //         (center_i32 + shape[1].0 * SCREEN_WIDTH_I32 + shape[1].1 * CELL_WIDTH_I32) as usize,
    //         (center_i32 + shape[2].0 * SCREEN_WIDTH_I32 + shape[2].1 * CELL_WIDTH_I32) as usize,
    //     )
    // }

    fn get_screen_cells_from_shape(screen_center: usize, shape: [(i32, i32); 3])
        -> (Option<usize>, Option<usize>, Option<usize>, Option<usize>) {

        let screen_center_i32 = screen_center as i32;

        let cell0_screen_index_i32 =
            screen_center_i32 + shape[0].0 * SCREEN_WIDTH_I32 + shape[0].1 * CELL_WIDTH_I32;
        let cell1_screen_index_i32 =
            screen_center_i32 + shape[1].0 * SCREEN_WIDTH_I32 + shape[1].1 * CELL_WIDTH_I32;
        let cell2_screen_index_i32 =
            screen_center_i32 + shape[2].0 * SCREEN_WIDTH_I32 + shape[2].1 * CELL_WIDTH_I32;

        (
            Some(screen_center),
            if cell0_screen_index_i32 >= 0 {Some(cell0_screen_index_i32 as usize)} else {None},
            if cell1_screen_index_i32 >= 0 {Some(cell1_screen_index_i32 as usize)} else {None},
            if cell2_screen_index_i32 >= 0 {Some(cell2_screen_index_i32 as usize)} else {None}
        )
    }
}

impl GridCoords {
    pub fn to_screen_index(&self) -> usize {
        cursor_positions::GRID_ORIGIN
        + self.i * view::SCREEN_WIDTH
        + self.j * view::CELL_WIDTH
    }
}