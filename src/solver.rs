use crate::board::Board;

const WIDTH_SIZE: u8 = 9;
const CELL_SIZE: u8 = 3;

pub trait Solver {
    fn run(board: &mut Board) -> bool;
}

// Solvers:
pub struct BacktrackSolver;
// pub struct GeneticSolver;    ?
// pub struct ExactCoverSolver; ?
// pub struct ConstraintSolver; ?

impl Solver for BacktrackSolver {
    /// Use backtracking algorithm to solve the sudoku puzzle.
    /// Sudoku might be unsolvable --> hence return false
    fn run(board: &mut Board) -> bool {
        Self::internal(board, 0, 0)
    }
}

// internal helper functions to match API.
impl BacktrackSolver {
    fn internal(board: &mut Board, mut r: u8, mut c: u8) -> bool {
        if r + 1 == WIDTH_SIZE && c == WIDTH_SIZE {
            return true;
        }
        if c == WIDTH_SIZE {
            r += 1;
            c = 0;
        }
        if board.get(r, c) != 0 {
            return Self::internal(board, r, c + 1);
        }
        for cands in Self::get_possible_values(board, r, c) {
            board.set(r, c, cands).ok();
            if Self::internal(board, r, c + 1) {
                return true;
            }
            board.set(r, c, 0).ok();
        }
        false
    }

    fn get_possible_values(board: &Board, r: u8, c: u8) -> Vec<u8> {
        let mut vals = Vec::new();
        'l: for cands in 1..=WIDTH_SIZE {
            // check row and col concurrently
            for o in 0..WIDTH_SIZE {
                if board.get(r, o) == cands || board.get(o, c) == cands {
                    continue 'l;
                }
            }
            // check box
            let pr = (r / CELL_SIZE) * CELL_SIZE;
            let pc = (c / CELL_SIZE) * CELL_SIZE;
            for i in 0..CELL_SIZE {
                for j in 0..CELL_SIZE {
                    if board.get(pr + i, pc + j) == cands {
                        continue 'l;
                    }
                }
            }
            vals.push(cands);
        }
        vals
    }

    // solution counter to check if unique solution for sudoku exists.
    pub fn num_solution(board: &mut Board) -> u8 {
        Self::num_solution_internal(board, 0, 0)
    }

    fn num_solution_internal(board: &mut Board, mut r: u8, mut c: u8) -> u8 {
        let mut count = 0;
        if r + 1 == WIDTH_SIZE && c == WIDTH_SIZE {
            return 1;
        }
        if c == WIDTH_SIZE {
            r += 1;
            c = 0;
        }
        if board.get(r, c) != 0 {
            return Self::num_solution_internal(board, r, c + 1);
        }
        for cands in Self::get_possible_values(board, r, c) {
            board.set(r, c, cands).ok();
            count += Self::num_solution_internal(board, r, c + 1);
            board.set(r, c, 0).ok();
        }
        count
    }
}
