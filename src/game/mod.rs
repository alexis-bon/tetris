use std::time;

mod core;
mod view;

const  GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 20;
const GRID_LENGTH: usize = GRID_HEIGHT * GRID_WIDTH;

const NEXT_TETROMINOS_QUEUE_SIZE: usize = 3;

mod tetromino;
mod cell;

mod state;

pub fn start_game() -> Result<(), String> {
    let mut state = state::State::new();

    let mut vram = match view::initialize_vram("data/screen.txt") {
        Ok (vram) => vram,
        Err(e) => return Err(e.to_string()),
    };
    
    let mut i = 68;
    loop {
        vram[1] = if vram[1] < b'Z' {vram[1] + 1} else {b'A'};
        vram[i] = vram[1];

        i = if i < 134 {i + 1} else {68};

        let vram_str = match String::from_utf8(vram.to_vec()) {
            Ok(vram_str) => vram_str,
            Err(e) => return Err(e.to_string()),
        };
        print!("{esc}c", esc = 27 as char);
        print!("{}", vram_str);

        std::thread::sleep(time::Duration::from_millis(40));
    }

    Ok(())
}