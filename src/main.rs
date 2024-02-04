mod tui;

use rudoku::{
    board::Board,
    solver::{ Solver, BTSolver },
    generator::Generator,
};
use std::{ error::Error, io };
use clap::{ Parser, Subcommand, Args };

type ErrCheck = Result<(), Box<dyn Error>>;

fn main() {
    let args = Arguments::parse();
    let _error_checker: ErrCheck = match args.app {
        App::Cli { opt, paths } => {
            if opt.solve && paths.len() == 2 {
                solve(&paths)
            } else if opt.create && paths.len() == 1 {
                create(&paths[0])
            } else {
                Err(Box::new(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Incorrect number of file paths provided."
                )))
            }
        },
        App::Tui { opt, path } => {
            if opt.create {
                tui_create()
            } else if opt.open && path.is_some() {
                tui_open(&path.unwrap())
            } else {
                Err(Box::new(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Need to provide a file path to open."
                )))
            }
        },
    };

    // match errors. 
}

// Argument Parser Logic

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[command(subcommand)]
    app: App,
}

#[derive(Subcommand, Debug)]
enum App {
    Cli {
        #[command(flatten)]
        opt: CliOptions,
        paths: Vec<String>,
    },
    Tui {
        #[command(flatten)]
        opt: TuiOptions,
        path: Option<String>,
    },
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct CliOptions {
    #[arg(short, long)]
    create: bool,
    #[arg(short, long)]
    solve: bool,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct TuiOptions {
    #[arg(short, long)]
    create: bool,
    #[arg(short, long)]
    open: bool,
}

// helper functions for different commands.
fn solve(paths: &Vec<String>) -> ErrCheck {
    let mut board = Board::from_file(&paths[0])?;
    let solved = BTSolver::run(&mut board);
    if solved {
        println!("Successfully solved {}\nSaving results to {}", paths[0], paths[1]);
        board.to_file(&paths[1])
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidData,
            "Sudoku board could not be solved."
        )))
    }
}

fn create(path: &String) -> ErrCheck {
    println!("Creating new game in '{}'", path);
    let mut gen = Generator::new(Generator::arbitrary_seed());
    gen.create().to_file(&path)    
}

fn tui_create() -> ErrCheck {
    let mut gen = Generator::new(Generator::arbitrary_seed());
    let board = gen.create();
    tui::start(board)
}

fn tui_open(path: &String) -> ErrCheck {
    let board = Board::from_file(&path)?;
    tui::start(board)
}
