use crate::game;
use game::cell::Cell;
use game::tetromino::Tetromino;
use rand::{self, Rng};

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
    can_hold: bool,
    next_tetrominos_queue: [Tetromino; game::NEXT_TETROMINOS_QUEUE_SIZE],
    score: u32,
    level: u32,
    lines: u32,
    clock: u128,
    rng: rand::rngs::ThreadRng
}

impl State {
    pub fn new() -> State {
        let mut rng = rand::rng();

        State {
            grid: [Cell::Empty; game::GRID_LENGTH],
            current_tetromino: CurrentTetromino {
                tetromino: Tetromino::J,
                position: GridCoords { i: 1, j: 4 },
                rotation: 0
            },
            hold: None,
            can_hold: true,
            next_tetrominos_queue: [
                Tetromino::from_index(rng.random()),
                Tetromino::from_index(rng.random()),
                Tetromino::from_index(rng.random())
            ],
            score: 0,
            level: 1,
            lines: 0,
            clock: 0,
            rng: rand::rng()
        }
    }

    pub fn get_grid_cell(&self, i: usize, j: usize ) -> Cell {
        self.grid[i * game::GRID_WIDTH + j]
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

    pub fn can_store(&self) -> bool {
        self.can_hold
    }

    pub fn get_in_next_tetromino_queue(&self, index: usize) -> Tetromino {
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

    pub fn is_grid_line_full(&self, i: usize) -> bool {
        for j in 0..game::GRID_WIDTH {
            if self.grid[i * game::GRID_WIDTH + j] == Cell::Empty {
                return false;
            }
        }

        true
    }
}

impl State {
    pub fn set_grid_cell(&mut self, i: usize, j: usize, cell: Cell) {
        self.grid[i * game::GRID_WIDTH + j] = cell
    }

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

    pub fn pop_tetromino_queue(&mut self) -> Tetromino {
        let tetromino = self.next_tetrominos_queue[0];
        self.next_tetrominos_queue[0] = self.next_tetrominos_queue[1];
        self.next_tetrominos_queue[1] = self.next_tetrominos_queue[2];
        self.next_tetrominos_queue[2] = self.get_random_tetromino();

        tetromino
    }

    pub fn set_new_current_tetromino(&mut self, new_tetromino: Tetromino) {
        self.current_tetromino = CurrentTetromino {
            tetromino: new_tetromino,
            position: GridCoords { i: 1, j: 4 },
            rotation: 0
        };
    }

    pub fn set_next_tetromino_to_current(&mut self) {
        self.current_tetromino = CurrentTetromino {
            tetromino: self.pop_tetromino_queue(),
            position: GridCoords { i: 1, j: 4 },
            rotation: 0
        };

        self.set_can_store_flag(true);
    }

    fn get_random_tetromino(&mut self) -> Tetromino {
        Tetromino::from_index(self.get_random_u32())
    }

    fn get_random_u32(&mut self) -> u32 {
        self.rng.random::<u32>()
    }

    pub fn set_stored_tetromino(&mut self, tetromino: Option<Tetromino>) {
        self.hold = tetromino;
    }

    pub fn set_can_store_flag(&mut self, value: bool) {
        self.can_hold = value
    }

    pub fn clear_grid_line(&mut self, i: usize) {
        for j in 0..game::GRID_WIDTH {
            self.set_grid_cell(i, j, Cell::Empty);
        }

        self.lines += 1
    }

    pub fn shift_tetromino_cells_down(&mut self, row_limit: usize) {
        if row_limit == 0 {
            panic!("Tried to shift tetromino cells down with row_limit = 0");
        }

        for j in 0..game::GRID_WIDTH {
            self.shift_tetromino_column_down(row_limit, j);
        }
    }

    fn shift_tetromino_column_down(&mut self, row_limit: usize, j: usize) {
        let mut i = row_limit - 1;
        let mut cell = self.get_grid_cell(i, j);

        while i > 0 {
            self.set_grid_cell(i + 1, j, cell);
            i -= 1;
            cell = self.get_grid_cell(i, j);
        }

        self.set_grid_cell(1, j, cell);
        self.set_grid_cell(0, j, Cell::Empty);
    }
}