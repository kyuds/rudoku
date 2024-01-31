use rudoku::board::Board;

const SUDOKU_TEXT: &'static str = 
r"  _____ _    _ _____   ____  _  ___    _ 
 / ____| |  | |  __ \ / __ \| |/ / |  | |
| (___ | |  | | |  | | |  | | ' /| |  | |
 \___ \| |  | | |  | | |  | |  < | |  | |
 ____) | |__| | |__| | |__| | . \| |__| |
|_____/ \____/|_____/ \____/|_|\_\\____/ ";

// Menu:
// - Arrow Keys
// - Verify (v)
// - Submit (s)
// - Reset  (r)
// - Quit   (q)

// pub struct GameState {
//     board: Board,
//     verify: bool,
//     position: (u8, u8),
// }

// pub trait Commands {
//     // TODO: refactor bad API...
//     fn move_pos(&mut self, axis: bool, step: bool) -> bool;
//     fn get_current_position(&self) -> (u8, u8);
//     fn toggle_verification(&mut self);
//     fn check_solution(&self) -> bool;
//     fn reset_board(&mut self);
// }

// pub trait ConsoleBoard {
//     fn lines(&self) -> Vec<String>;
// }

// impl ConsoleBoard for Board {

// }

pub fn tui_start(_board: Board) {
    
}
