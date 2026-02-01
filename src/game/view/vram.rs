use std::{fs, io, ptr};

use crate::game::state::State;
use crate::game::view::{self, View, cursor_positions};

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
    load_uint(view, state.get_level(), cursor_positions::LEVEL_COUNTER);
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
