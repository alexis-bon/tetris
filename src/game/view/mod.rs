use std::{fs, io::{self, Write}, ptr};
use crossterm::{
    terminal,
    cursor,
    style::Print,
    queue,
    execute
};

use crate::game::state::State;

const SCREEN_WIDTH : usize = 69;
const SCREEN_HEIGHT: usize = 22;
const SCREEN_LENGTH: usize = SCREEN_WIDTH * SCREEN_HEIGHT - 2;

pub struct View {
    vram: [u8; SCREEN_LENGTH],
    stdout: io::Stdout,
    
}

pub fn initialize_view(file_path: &str) -> io::Result<View> {
    terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, cursor::Hide)?;

    Ok(
        View {
            vram: initialize_vram(file_path)?,
            stdout,
    })
}

pub fn display_state(state: &State, view: &mut View) -> io::Result<()> {
    let vram_str = match String::from_utf8(view.vram.to_vec()) {
        Ok(vram_str) => vram_str,
        Err(e) => panic!("UTF8 error in display_state : {}", e.to_string()),
    };

    view.vram[1] = if view.vram[1] < b'Z' {view.vram[1] + 1} else {b'A'};

    queue!(view.stdout, terminal::Clear(terminal::ClearType::All))?;
    queue!(view.stdout, terminal::Clear(terminal::ClearType::Purge))?;
    queue!(view.stdout, cursor::MoveTo(0, 0))?;
    queue!(view.stdout, Print(vram_str))?;
    // queue!(view.stdout, Print("Tetris le jeu\n"))?;
    // queue!(view.stdout, Print("0 2 4 6 8 0 2 4 6 8\n"))?;
    view.stdout.flush()?;

    Ok(())
}

pub fn close_view(view: &mut View) -> io::Result<()> {
    execute!(view.stdout, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn initialize_vram(file_path: &str) -> io::Result<[u8; SCREEN_LENGTH]> {
    let mut file = fs::File::open(file_path)?;
    let mut content_string = String::new();

    io::Read::read_to_string(&mut file, &mut content_string)?;
    content_string = content_string.replace("\n", "\n\r");
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
