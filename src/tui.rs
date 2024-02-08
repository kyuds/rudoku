use rudoku::{
    board::Board,
    solver::{ BTSolver, Solver },
};
use std::{
    io::{ self, Write, Stdout },
    error::Error,
};
use crossterm::{
    terminal, cursor, queue,
    QueueableCommand,
    event::{ self, Event },
    terminal::{
        enable_raw_mode, 
        disable_raw_mode,
    },
    style::{
        Print, 
        SetForegroundColor, 
        SetBackgroundColor, 
        ResetColor, 
        Color, 
        Attribute,
    },
};
use std::{thread, time};

// DEVELOPMENT IN PROGRESS

pub fn start(_board: Board) -> Result<(), Box<dyn Error>> {
    // enable_raw_mode()?;
    // disable_raw_mode()?;
    println!("Not implemented yet!");
    Ok(())
}

// TODO: create palette struct + IMPL for white and dark term. 
// struct Palette {
//     outer_boarder: str,
//     inner_boarder: str,
//     highlight: str,
//     text: str,
//     invariant: str,
// }

struct GameState {
    // palette: Palette, // TODO
    board: Board,
    position: (u8, u8),
    message: String,
}

trait Commands {
    fn move_cursor(&mut self, cmd: char);
    fn check_solution(&self) -> bool;
    fn reset_board(&mut self);
    fn change_number(&mut self, r: u8, c: u8, num: u8);
}

trait Console {
    fn display(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn Error>>;
}

// impl Commands for GameState {

// }

impl Console for GameState {
    fn display(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
        stdout.queue(cursor::SavePosition)?
              .queue(SetForegroundColor(Color::Blue))?
              .queue(Print(String::from("+---+---+---+---+---+---+---+---+---+\n")))?
              .queue(cursor::MoveToNextLine(1))?;
        for r in 0..9 {
            stdout.queue(SetForegroundColor(Color::Blue))?
                  .queue(Print(String::from("|")))?;
            for c in 0..9 {
                stdout.queue(SetForegroundColor(Color::Blue))?
                      .queue(Print(self.format_cell(r, c)))?
                      .queue(self.get_border_color(c))?
                      .queue(Print(String::from("|")))?;
            }
            stdout.queue(Print(String::from("\n")))?
                  .queue(cursor::MoveToNextLine(1))?
                  .queue(SetForegroundColor(Color::Blue))?
                  .queue(Print(String::from("+")))?;
            for c in 0..9 {
                stdout.queue(self.get_border_color(r))?
                      .queue(Print(String::from("---")))?
                      .queue({
                        if (r == 2 || r == 5) && c != 8 && r != 8 {
                            SetForegroundColor(Color::Green)
                        } else if r == 8 {
                            SetForegroundColor(Color::Blue)
                        } else {
                            self.get_border_color(c)
                        }
                      })?
                      .queue(Print(String::from("+")))?;
            }
            stdout.queue(Print(String::from("\n")))?
                  .queue(cursor::MoveToNextLine(1))?;
        }
        stdout.flush()?;
        stdout.queue(cursor::RestorePosition)?;
        stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown))?;
        Ok(())
    }
}

impl GameState {
    fn new(board: Board) -> Self {
        GameState {
            // TODO: need palette here. 
            board,
            position: (0, 0),
            message: String::from(""),
        }
    }

    fn format_cell(&self, r: u8, c: u8) -> String {
        let num = self.board.get(r, c);
        format!(" {} ", {
            if num == 0 {
                String::from(" ")
            } else {
                num.to_string()
            }
        })
    }

    fn get_border_color(&self, num: u8) -> SetForegroundColor {
        if num == 2 || num == 5 {
            return SetForegroundColor(Color::Green);
        }
        SetForegroundColor(Color::Blue)
    }
}
