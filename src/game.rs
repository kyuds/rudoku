// use rudoku::board::{ Board, BoardInvalid };

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
