use crate::game;
use game::cell::Cell;
use game::tetromino::Tetromino;

pub struct CurrentTetromino {
    tetromino: Tetromino,
    position: (usize, usize),
    rotation: usize
}

pub struct State {
    pub grid: [Cell; game::GRID_LENGTH],
    current_tetromino: CurrentTetromino,
    hold: Option<Tetromino>,
    next_tetrominos_queue: [Option<Tetromino>; game::NEXT_TETROMINOS_QUEUE_SIZE],
    score: i32,
    level: u32,
    lines: u32
}

impl State {
    pub fn new() -> State {
        State {
            grid: [Cell::Empty; game::GRID_LENGTH],
            current_tetromino: CurrentTetromino {
                tetromino: Tetromino::J,
                position: (1, 4),
                rotation: 0
            },
            hold: None,
            next_tetrominos_queue: [None; game::NEXT_TETROMINOS_QUEUE_SIZE],
            score: 0,
            level: 1,
            lines: 0
        }
    }

    pub fn get_current_tetromino(&self) -> Tetromino {
        self.current_tetromino.tetromino
    }

    pub fn get_current_tetromino_position(&self) -> (usize, usize) {
        self.current_tetromino.position
    }

    pub fn get_current_tetromino_rotation(&self) -> usize {
        self.current_tetromino.rotation
    }

    pub fn get_hold_tetromino(&self) -> Option<Tetromino> {
        self.hold
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }
}

impl State {
    pub fn increment_level(&mut self) {
        self.level += 1;
    }

    pub fn decrement_level(&mut self) {
        let l = self.level;
        self.level = if l > 0 {l - 1} else {l};
    }

    pub fn increment_rotation(&mut self) {
        self.current_tetromino.rotation = (self.current_tetromino.rotation + 1) % 4;
    }

    pub fn store_current_tetromino(&mut self) {
        // temporary function, to be completed
        self.hold = Some(self.get_current_tetromino());
    }
}