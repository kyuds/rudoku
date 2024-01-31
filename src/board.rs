use std::io::{ self, Write };
use std::collections::HashSet;
use std::error::Error;
use std::fs::{ self, OpenOptions};

const BOARD_SIZE: usize = 81;
const WIDTH_SIZE: u8 = 9;

/// Enum to determine positions that violate sudoku properties.
pub enum BoardInvalid {
    Square(u8),
    Row(u8),
    Col(u8),
}

pub struct Board {
    // 9x9 board translates to a length 81 array
    board: [u8; BOARD_SIZE],
    // There are fixed numbers in a sudoku puzzle.
    // Those fixed numbers should be invariant.
    // tuple: (pos, val)
    invariant: Vec<(u8, u8)>,
}

impl Board {
    pub fn new(board: [u8; BOARD_SIZE], invariant: Vec<(u8, u8)>) -> Board {
        Board { board, invariant }
    }

    pub fn empty() -> Board {
        Board {
            board: [0; BOARD_SIZE],
            invariant: Vec::new(),
        }
    }

    pub fn from_file(path: &str) -> Result<Board, Box<dyn Error>> {
        let mut board = Self::empty();
        let contents = fs::read_to_string(path)?;
        let mut curr_pos: u8 = 0;

        for line in contents.lines() {
            for ch in line.split_whitespace() {
                if curr_pos == 81 {
                    return Err(Box::new(io::Error::new(
                        io::ErrorKind::InvalidData, 
                        "Input should be 9x9 grid. Only 81 numbers."
                    )));
                }
                let num: u8 = match ch.parse() {
                    Ok(n) => {
                        if n < 10 { n }
                        else {
                            return Err(Box::new(io::Error::new(
                                io::ErrorKind::InvalidData, 
                                "Numbers should be in [0,10)."
                            )));
                        }
                    },
                    Err(e) => { return Err(Box::new(e)); },
                };
                if num != 0 {
                    board.invariant.push((curr_pos, num));
                    board.board[usize::from(curr_pos)] = num;
                }
                curr_pos += 1;
            }
        }
        if curr_pos != 81 {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData, 
                "Input is less than 81 numbers."
            )));
        }
        Ok(board)
    }

    pub fn to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new().write(true)
                                         .create_new(true)
                                         .open(path)?;
        let mut stringify = String::new();
        for r in 0..9 {
            for c in 0..9 {
                stringify.push_str(&format!("{} ", self.get(r, c)));
            }
            stringify.push_str("\n");
        }
        file.write(stringify.as_bytes())?;
        Ok(())
    }

    /// Check if the sudoku board is solved completely. No empty spaces.
    pub fn verify(&self) -> bool {
        // 0~8: row, 9~17: col, 18~26: grid
        let mut sets: [HashSet<u8>; 27] = Default::default();

        // check (for some reason) sudoku board is modified.
        if !self.check_invariants() {
            return false;
        }

        for r in 0..9 {
            for c in 0..9 {
                let num = self.get(r, c);
                
                if num == 0 {
                    return false;
                }
                let g_idx = usize::from((r / 3) * 3 + c / 3 + 18);
                let r_idx = usize::from(r);
                let c_idx = usize::from(c + 9);

                if sets[r_idx].contains(&num) || 
                   sets[c_idx].contains(&num) || 
                   sets[g_idx].contains(&num) {
                    return false;
                }
                sets[r_idx].insert(num);
                sets[c_idx].insert(num);
                sets[g_idx].insert(num);
            }
        }
        true
    }

    /// Check if the sudoku board is solved.
    /// If not solved, return a vector of "coordinates" with designated "type" of invalid coordinates.
    /// Note that having an empty position doesn't matter, but having a wrong position does.
    /// This feature is intended to be used in the interactive GUI.
    pub fn verbose_verify(&self) -> (bool, Vec<BoardInvalid>) {
        let mut sets: [(HashSet<u8>, usize); 27] = Default::default();

        for r in 0..9 {
            for c in 0..9 {
                let num = self.get(r, c);
            
                let g_idx = usize::from((r / 3) * 3 + c / 3 + 18);
                let r_idx = usize::from(r);
                let c_idx = usize::from(c + 9);

                if num == 0 {
                    sets[r_idx].1 += 1;
                    sets[c_idx].1 += 1;
                    sets[g_idx].1 += 1;
                } else {
                    sets[r_idx].0.insert(num);
                    sets[c_idx].0.insert(num);
                    sets[g_idx].0.insert(num);
                }
            }
        }

        let mut v = Vec::new();
        for i in 0..27 {
            let idx = usize::from(i);
            if sets[idx].0.len() + sets[idx].1 < 9 {
                if i < 9 {
                    v.push(BoardInvalid::Row(i));
                } else if i < 18 {
                    v.push(BoardInvalid::Col(i - 9));
                } else {
                    v.push(BoardInvalid::Square(i - 18));
                }
            }
        }

        (v.is_empty(), v)
    }

    /// Check if board maintains its original fixed values. This is a sanity check.
    pub fn check_invariants(&self) -> bool {
        self.invariant
            .iter()
            .all(|&t| self.board[usize::from(t.0)] == t.1)
    }

    /// Getter method for respective coordinate. Panics if coordinate is invalid.
    /// Will panic on out of bounds error. 
    pub fn get(&self, r: u8, c: u8) -> u8 {
        self.board[usize::from(linearize(r, c))]
    }

    /// Setter method for respective coordinate. Panics if coordinate is invalid. Returns an error
    /// if coordinate is set by the "problem" board and is supposed to be invariant.
    /// Will panic on out of bounds error. 
    pub fn set(&mut self, r: u8, c: u8, val: u8) -> Result<(), &'static str> {
        let coor = linearize(r, c);
        let const_pos = self.invariant
                            .iter()
                            .any(|&t| t.0 == coor);
        if const_pos && self.board[usize::from(coor)] != val {
            return Err("Coordinate shouldn't be changed.");
        }
        self.board[usize::from(coor)] = val;
        Ok(())
    }

    /// Restore said board to the original problem state.
    pub fn reset(&mut self) {
        self.board = [0; BOARD_SIZE];
        for p in &self.invariant {
            self.board[usize::from(p.0)] = p.1;
        }
    }

    /// Print board
    pub fn print(&self) {
        for r in 0..9 {
            for c in 0..9 {
                print!("{} ", self.board[usize::from(linearize(r, c))]);
            }
            println!("");
        }
    }
}

// Helper functions

fn linearize(r: u8, c: u8) -> u8 {
    if r > WIDTH_SIZE || c > WIDTH_SIZE {
        panic!("Coordinates ({r}, {c}) is invalid in 9x9 grid.");
    }
    r * 9 + c
}

// Test Module
#[cfg(test)]
mod tests {
    #[test]
    fn test_file_io() {

    }

    #[test]
    fn test_get_set() {

    }

    #[test]
    fn test_verify() {

    }

    #[test]
    fn test_invariants() {

    }
}
