use crate::board::Board;

const WIDTH_SIZE: u8 = 9;

pub trait Solver {
    fn run(board: &mut Board) -> bool;
}

// Solvers:
pub struct BTSolver;
// pub struct GASolver;
// pub struct ECSolver;
// pub struct CSPSolver;

impl Solver for BTSolver {
    /// Use backtracking algorithm to solve the sudoku puzzle.
    /// Sudoku might be unsolvable --> hence return false
    fn run(board: &mut Board) -> bool {
        Self::internal(board, 0, 0)
    }
}

// internal helper functions to match API.
impl BTSolver {
    fn internal(board: &mut Board, mut r: u8, mut c: u8) -> bool {
        if r + 1 == WIDTH_SIZE && c == WIDTH_SIZE {
            return true;
        } else if c == WIDTH_SIZE {
            r += 1;
            c = 0;
        }
        if board.get(r, c) != 0 {
            return Self::internal(board, r, c + 1);
        }
        for cands in board.get_possible_values(r, c) {
            board.set(r, c, cands).ok();
            if Self::internal(board, r, c + 1) {
                return true;
            }
            board.set(r, c, 0).ok();
        }
        false
    }

    // solution counter to check if unique solution for sudoku exists.
    pub fn num_solution(board: &mut Board) -> u8 {
        Self::num_solution_internal(board, 0, 0)
    }

    fn num_solution_internal(board: &mut Board, mut r: u8, mut c: u8) -> u8 {
        let mut count = 0;
        if r + 1 == WIDTH_SIZE && c == WIDTH_SIZE {
            return 1;
        } else if c == WIDTH_SIZE {
            r += 1;
            c = 0;
        }
        if board.get(r, c) != 0 {
            return Self::num_solution_internal(board, r, c + 1);
        }
        for cands in board.get_possible_values(r, c) {
            board.set(r, c, cands).ok();
            count += Self::num_solution_internal(board, r, c + 1);
            board.set(r, c, 0).ok();
        }
        count
    }
}
