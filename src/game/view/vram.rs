use std::{fs, io, ptr};

use crate::game;
use crate::game::{cell::Cell, state::State};
use crate::game::view::{self, View, cursor_positions, tetromino_sprite::TetrominoSprite};

const TETROMINO_CELL_CHAR: u8 = b'H';

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
    clear_grid(view);
    load_uint(view, state.get_level(), cursor_positions::LEVEL_COUNTER);
    load_grid(state, view);
}

fn clear_grid(view: &mut View) {
    for index in cursor_positions::GRID_ORIGIN..cursor_positions::GRID_END {
        if view.vram[index] == TETROMINO_CELL_CHAR {
            view.vram[index] = b' ';
        }
    }
}

fn digit_to_utf8(digit: u8) -> u8 {
    let utf8_zero = b'0' as u8;
    utf8_zero + digit
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

fn load_grid(state: &State, view: &mut View) {
    for (index, cell) in state.grid.iter().enumerate() {
        let cell_grid_position = (
            index / game::GRID_WIDTH,
            index % game::GRID_WIDTH
        );

        load_tetromino_cell(view, cell, cell_grid_position);
    }
}

fn load_tetromino_cell(view: &mut View, cell: &Cell, cell_grid_position: (usize, usize)) {
    let cell_char = match cell {
        Cell::Full => b'H',
        Cell::Empty => b' ',
    };
    let cell_screen_position = cursor_positions::GRID_ORIGIN
        + cell_grid_position.0 * view::SCREEN_WIDTH
        + cell_grid_position.1 * view::CELL_WIDTH;

    view.vram[cell_screen_position] = cell_char;
    view.vram[cell_screen_position + 1] = cell_char;
}