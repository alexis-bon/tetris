use std::fs;
use std::io;
use std::io::Read;
use std::ptr;

use crate::game::state::State;

const SCREEN_WIDTH : usize = 68;
const SCREEN_HEIGHT: usize = 22;
const SCREEN_LENGTH: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

pub fn initialize_vram(file_path: &str) -> std::io::Result<[u8; SCREEN_LENGTH]> {
    let mut file = fs::File::open(file_path)?;
    let mut content_string = String::new();
    file.read_to_string(&mut content_string)?;
    let content_bytes = content_string.as_bytes();

    let mut vram = [b' '; SCREEN_LENGTH];
    unsafe {
        ptr::copy(
            content_bytes.as_ptr(),
            vram.as_mut_ptr(),
            SCREEN_LENGTH
        );
    }

    Ok(vram)
}

pub fn display_state(state: &State) {
    let screen = String::from("");
}
