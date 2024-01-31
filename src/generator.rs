use crate::board::Board;
use crate::solver::BacktrackSolver;

const BOARD_SIZE: usize = 81;
const WIDTH_SIZE: u8 = 9;
const CELL_SIZE: u8 = 3;

pub struct Generator;

impl Generator {
    /// Create new board;
    pub fn create() -> Board {
        Board::empty()
    }
}
