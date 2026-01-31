use crate::game;
use game::cell_color::CellColor;
use game::tetromino::Tetromino;

pub struct State {
    grid: [Option<CellColor>; game::GRID_LENGTH],
    hold: Option<Tetromino>,
    next_tetrominos_queue: [Option<Tetromino>; game::NEXT_TETROMINOS_QUEUE_SIZE],
    score: i32,
    level: u32,
    lines: u32
}

impl State {
    pub fn new() -> State {
        State {
            grid: [None; game::GRID_LENGTH],
            hold: None,
            next_tetrominos_queue: [None; game::NEXT_TETROMINOS_QUEUE_SIZE],
            score: 0,
            level: 1,
            lines: 0
        }
    }

    pub fn get_level(self) -> u32 {
        self.level
    }

    pub fn increment_level(&mut self) {
        self.level += 1;
    }
}