use std::{fs, io, ptr};

use crate::game;
use crate::game::state::{State, CurrentTetromino};
use crate::game::cell::Cell;
use crate::game::view::{self, View, cursor_positions, tetromino_sprite::TetrominoSprite};

const TETROMINO_CELL_CHAR: u8 = b'H';
const EMPTY_CELL_CHAR: u8 = b' ';

pub fn initialize(file_path: &str) -> io::Result<[u8; view::SCREEN_LENGTH]> {
    let mut file = fs::File::open(file_path)?;
    let mut content_string = String::new();

    io::Read::read_to_string(&mut file, &mut content_string)?;
    content_string = content_string.replace("\n", "\n\r");
    let content_bytes = content_string.as_bytes();

    let mut vram = [b' '; view::SCREEN_LENGTH];
    unsafe {
        ptr::copy(
            content_bytes.as_ptr(),
            vram.as_mut_ptr(),
            view::SCREEN_LENGTH
        );
    }

    Ok(vram)
}

pub fn load_state_data(state: &State, view: &mut View) {
    load_grid(state, view);
    load_current_tetromino_sprite(view, state.get_current_tetromino_ref());
    load_hold_section(view, state);
    load_next_section(view, state);
    load_uint(view, state.get_score(), cursor_positions::SCORE_COUNTER);
    load_uint(view, state.get_level(), cursor_positions::LEVEL_COUNTER);
    load_uint(view, state.get_lines(), cursor_positions::LINES_COUNTER);
}

fn load_grid(state: &State, view: &mut View) {
    for (index, cell) in state.grid.iter().enumerate() {
        let cell_grid_position = (
            index / game::GRID_WIDTH,
            index % game::GRID_WIDTH
        );

        load_tetromino_cell_grid(view, cell, cell_grid_position);
    }
}

fn load_tetromino_cell_grid(view: &mut View, cell: &Cell, cell_grid_position: (usize, usize)) {
    let cell_char = match cell {
        Cell::Full => TETROMINO_CELL_CHAR,
        Cell::Empty => EMPTY_CELL_CHAR,
    };
    let cell_screen_position = cursor_positions::GRID_ORIGIN
        + cell_grid_position.0 * view::SCREEN_WIDTH
        + cell_grid_position.1 * view::CELL_WIDTH;

    view.vram[cell_screen_position] = cell_char;
    view.vram[cell_screen_position + 1] = cell_char;
}

fn load_current_tetromino_sprite(view: &mut View, current_tetromino: &CurrentTetromino) {
    let sprite = TetrominoSprite::of_current_tetromino(&current_tetromino);

    load_tetromino_sprite(view, sprite);
}

fn load_hold_section(view: &mut View, state: &State) {
    clear_section(
        view,
        cursor_positions::HOLD_GRID_ORIGIN,
        view::HOLD_SECTION_HEIGHT
    );

    if let Some(tetromino) = state.get_stored_tetromino() {
        let tetromino_sprite = TetrominoSprite::display_sprite(
            tetromino,
            cursor_positions::HOLD_GRID_CENTER
        );
        load_tetromino_sprite(
            view,
            tetromino_sprite
        );
    }
}

fn load_next_section(view: &mut View, state: &State) {
    clear_section(
        view,
        cursor_positions::NEXT_GRID_ORIGIN,
        view::NEXT_SECTION_HEIGHT
    );

    load_next_section_part(view, state, 0, cursor_positions::NEXT_GRID_CENTER0);
    load_next_section_part(view, state, 1, cursor_positions::NEXT_GRID_CENTER1);
    load_next_section_part(view, state, 2, cursor_positions::NEXT_GRID_CENTER2);
}

fn load_next_section_part(view: &mut View, state: &State, queue_index: usize, screen_center: usize) {
    if let Some(tetromino) = state.get_in_next_tetromino_queue(queue_index) {
        let tetromino_sprite = TetrominoSprite::display_sprite(
            tetromino,
            screen_center
        );
        load_tetromino_sprite(
            view,
            tetromino_sprite
        );
    }
}

// fn load_tetromino_sprite(view: &mut View, tetromino: Option<Tetromino>, center_screen_position: usize) {
//     if let Some(tetromino) = tetromino {
//         let sprite = TetrominoSprite::display_sprite(
//             tetromino,
//             center_screen_position
//         );

//         load_tetromino_cell(view, sprite.cells_grid_position.0);
//         load_tetromino_cell(view, sprite.cells_grid_position.1);
//         load_tetromino_cell(view, sprite.cells_grid_position.2);
//         load_tetromino_cell(view, sprite.cells_grid_position.3);
//     }
// }

fn load_tetromino_sprite(view: &mut View, sprite: TetrominoSprite) {
    if let Some(cell0_screen_position) = sprite.cells_screen_position.0 {
        load_tetromino_cell(view, cell0_screen_position);
    }
    if let Some(cell1_screen_position) = sprite.cells_screen_position.1 {
        load_tetromino_cell(view, cell1_screen_position);
    }
    if let Some(cell2_screen_position) = sprite.cells_screen_position.2 {
        load_tetromino_cell(view, cell2_screen_position);
    }
    if let Some(cell3_screen_position) = sprite.cells_screen_position.3 {
        load_tetromino_cell(view, cell3_screen_position);
    }
}

fn load_tetromino_cell(view: &mut View, screen_position: usize) {
    view.vram[screen_position] = TETROMINO_CELL_CHAR;
    view.vram[screen_position + 1] = TETROMINO_CELL_CHAR;
}

fn clear_section(view: &mut View, origin: usize, height: usize) {
    for k in 0..height {
        let line_origin = origin + k * view::SCREEN_WIDTH;
        clear_section_line(view, line_origin);
    }
}

fn clear_section_line(view: &mut View, line_origin: usize) {
    for index in line_origin..(line_origin + view::SECTION_WIDTH) {
        if view.vram[index] == TETROMINO_CELL_CHAR {
            view.vram[index] = EMPTY_CELL_CHAR;
        }
    }
}

fn load_uint(view: &mut View, n: u32, position: usize) {
    let digit0 = digit_to_utf8(( n          % 10) as u8);
    let digit1 = digit_to_utf8(((n / 10   ) % 10) as u8);
    let digit2 = digit_to_utf8(((n / 100  ) % 10) as u8);
    let digit3 = digit_to_utf8(((n / 1000 ) % 10) as u8);
    let digit4 = digit_to_utf8(((n / 10000) % 10) as u8);

    view.vram[position    ] = digit0;
    view.vram[position - 1] = digit1;
    view.vram[position - 2] = digit2;
    view.vram[position - 3] = digit3;
    view.vram[position - 4] = digit4;
}

fn digit_to_utf8(digit: u8) -> u8 {
    let utf8_zero = b'0' as u8;
    utf8_zero + digit
}