pub mod input;

mod cursor_positions;
mod vram;

use std::io::{self, Write};
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
            vram: vram::initialize(file_path)?,
            stdout,
    })
}

pub fn display_state(state: &State, view: &mut View) -> io::Result<()> {
    let vram_str = match String::from_utf8(view.vram.to_vec()) {
        Ok(vram_str) => vram_str,
        Err(e) => panic!("UTF8 error in display_state : {}", e.to_string()),
    };

    vram::load_state_data(state, view);

    queue!(view.stdout, terminal::Clear(terminal::ClearType::All))?;
    queue!(view.stdout, terminal::Clear(terminal::ClearType::Purge))?;
    queue!(view.stdout, cursor::MoveTo(0, 0))?;
    queue!(view.stdout, Print(vram_str))?;
    view.stdout.flush()?;

    Ok(())
}

pub fn close_view(view: &mut View) -> io::Result<()> {
    execute!(view.stdout, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}