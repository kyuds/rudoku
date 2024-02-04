use crate::{
    board::Board,
    solver::BTSolver,
};
use rand::{
    prelude::*,
    seq::SliceRandom,
};
use rand_chacha::ChaCha8Rng;
use chrono::offset::Local;

const WIDTH_SIZE: u8 = 9;
const PUNCTURE_MAX_ATTEMPTS: u8 = 15;

pub struct Generator {
    rng: ChaCha8Rng,
}

// Separated generator constructor and board
// creater so that the generator can be reused
// multiple times.
impl Generator {
    /// Constructor for reuse
    pub fn new(seed: u64) -> Self {
        Generator {
            rng: ChaCha8Rng::seed_from_u64(seed)
        }
    }

    /// Sudoku board generator based on randomized,
    /// optimized backtracking.
    pub fn create(&mut self) -> Board {
        let mut board = Board::empty();
        self.complete_fill(&mut board);
        self.puncture(&mut board);
        board
    }

    fn complete_fill(&mut self, board: &mut Board) {
        // follows algorithm of inserting 3 diagonal "squares"
        // and then performing backtracking so that the number
        // of tiles to backtrack is 54 instead of 81.
        self.fill_diagonal_box(board, 0, 0);
        self.fill_diagonal_box(board, 3, 3);
        self.fill_diagonal_box(board, 6, 6);
        assert!(self.fill_remaining(board, 0, 0));
    }

    fn fill_diagonal_box(&mut self, board: &mut Board, r: u8, c: u8) {
        let mut vals = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        vals.shuffle(&mut self.rng);
        for x in 0..3 {
            for y in 0..3 {
                let lin = usize::from(x * 3 + y);
                let _ = board.set(r + x, c + y, vals[lin]);
            }
        }
    }

    fn fill_remaining(&mut self, board: &mut Board, mut r: u8, mut c: u8) -> bool {
        if r + 1 == WIDTH_SIZE && c == WIDTH_SIZE {
            return true;
        } else if c == WIDTH_SIZE {
            r += 1;
            c = 0;
        }
        if board.get(r, c) != 0 {
            return self.fill_remaining(board, r, c + 1);
        }
        let mut possibles = board.get_possible_values(r, c);
        possibles.shuffle(&mut self.rng);
        for cands in possibles {
            board.set(r, c, cands).ok();
            if self.fill_remaining(board, r, c + 1) {
                return true;
            }
            board.set(r, c, 0).ok();
        }
        false
    }

    fn puncture(&mut self, board: &mut Board) {
        'l: loop {
            for _ in 0..PUNCTURE_MAX_ATTEMPTS {
                let idx = self.rng.gen::<u8>() % 81;
                let r = idx / 9;
                let c = idx % 9;
                let tmp = board.get(r, c);
                if tmp == 0 {
                    continue;
                }
                let _ = board.set(r, c, 0);
                if BTSolver::num_solution(board) == 1 {
                    continue 'l;
                }
                let _ = board.set(r, c, tmp);
            }
            break;
        }
    }

    /// Creates a seed based on local time stamp. Custom seeds are
    /// also possible, should just be an unsigned 64-bit integer.
    pub fn arbitrary_seed() -> u64 {
        u64::from_ne_bytes(Local::now().timestamp().to_ne_bytes())
    }
}
