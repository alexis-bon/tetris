use crate::game;
use game::cell::Cell;
use game::tetromino::Tetromino;

#[derive(Clone, Copy)]
pub struct GridCoords {
    pub i: usize,
    pub j: usize
}

impl GridCoords {
    pub fn equals(&self, other: &GridCoords) -> bool {
        self.i == other.i && self.j == other.j
    }

    pub fn to_grid_index(&self) -> usize {
        self.i * game::GRID_WIDTH + self.j
    }
}

pub struct CurrentTetromino {
    tetromino: Tetromino,
    position: GridCoords,
    rotation: usize
}

impl CurrentTetromino {
    pub fn get_tetromino(&self) -> Tetromino {
        self.tetromino
    }

    pub fn get_position(&self) -> GridCoords {
        self.position
    }

    pub fn get_rotation(&self) -> usize {
        self.rotation
    }
}

impl CurrentTetromino {
    /** Move the current tetromino position one cell to the left.
     *  Panics if current tetromino j coord is already 0
     */
    pub fn move_left(&mut self) {
        if self.position.j == 0 {
            panic!("Tried to move left current tetromino while it was already at column j = 0");
        }
        self.position.j -= 1
    }

    /** Move the current tetromino position one cell to the right.
     *  Panics if current tetromino j coord is already GRID_WIDTH - 1
     */
    pub fn move_right(&mut self) {
        if self.position.j + 1 == game::GRID_WIDTH {
            panic!("Tried to move right current tetromino while it was already at column j = GRID_WIDTH - 1");
        }
        self.position.j += 1
    }

    /** Move the current tetromino position one cell downwards.
     *  Panics if current tetromino i coord is already GRID_HEIGHT - 1
     */
    pub fn move_down(&mut self) {
        if self.position.i + 1 == game::GRID_HEIGHT {
            panic!("Tried to move down current tetromino while it was already at row i = GRID_HEIGHT - 1");
        }
        self.position.i += 1
    }

    /** Move the current tetromino position one cell upwards.
     *  Panics if current tetromino i coord is already 0
     */
    pub fn move_up(&mut self) {
        if self.position.i  == 0 {
            panic!("Tried to move up current tetromino while it was already at row i = 0");
        }
        self.position.i -= 1
    }
}

pub struct State {
    pub grid: [Cell; game::GRID_LENGTH],
    current_tetromino: CurrentTetromino,
    hold: Option<Tetromino>,
    next_tetrominos_queue: [Option<Tetromino>; game::NEXT_TETROMINOS_QUEUE_SIZE],
    score: u32,
    level: u32,
    lines: u32,
    clock: u128
}

impl State {
    pub fn new() -> State {
        State {
            grid: [Cell::Empty; game::GRID_LENGTH],
            current_tetromino: CurrentTetromino {
                tetromino: Tetromino::J,
                position: GridCoords { i: 1, j: 4 },
                rotation: 0
            },
            hold: None,
            // next_tetrominos_queue: [None; game::NEXT_TETROMINOS_QUEUE_SIZE],
            next_tetrominos_queue: [Some(Tetromino::J), Some(Tetromino::O), None],
            score: 1500,
            level: 3,
            lines: 17,
            clock: 0
        }
    }

    pub fn get_current_tetromino_ref(&self) -> &CurrentTetromino {
        &self.current_tetromino
    }

    pub fn get_current_tetromino_mutref(&mut self) -> &mut CurrentTetromino {
        &mut self.current_tetromino
    }

    pub fn get_current_tetromino(&self) -> Tetromino {
        self.current_tetromino.tetromino
    }

    pub fn get_current_tetromino_position(&self) -> GridCoords {
        self.current_tetromino.position
    }

    pub fn get_current_tetromino_rotation(&self) -> usize {
        self.current_tetromino.rotation
    }

    pub fn get_stored_tetromino(&self) -> Option<Tetromino> {
        self.hold
    }

    pub fn get_in_next_tetromino_queue(&self, index: usize) -> Option<Tetromino> {
        self.next_tetrominos_queue[index]
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }

    pub fn get_lines(&self) -> u32 {
        self.lines
    }

    pub fn get_clock(&self) -> u128 {
        self.clock
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

    pub fn increment_clock(&mut self) {
        self.clock += 1
    }

    pub fn set_stored_tetromino(&mut self, tetromino: Option<Tetromino>) {
        self.hold = tetromino;
    }
}