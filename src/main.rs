mod game;

use std::error::Error;
// use rudoku::board::Board;
// use rudoku::solver::backtrack_solver;
// use rudoku::generator::generate;
use clap::{ Parser, Subcommand, Args };
// use crate::game;

// rudoku {cli | gui} --{(crte|slv)|(crte|opn)} PATHS...

fn main() {
    let args = Arguments::parse();

    let handler: Result<(), Box<dyn Error>> = match args.app {
        App::Cli { opt, paths } => {
            if opt.solve && paths.len() == 2 {
                solve(&paths)
            } else if opt.create && paths.len() == 1 {
                create(&paths[0])
            } else {
                println!("Incorrect number of file paths provided.");
                Ok(())
            }
        },
        App::Gui { opt, path } => {
            if opt.create {
                Ok(())
            } else if opt.open && path.is_some() {
                Ok(())
            } else {
                println!("Need to provide a file path to open.");
                Ok(())
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
    Gui {
        #[command(flatten)]
        opt: GuiOptions,
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
struct GuiOptions {
    #[arg(short, long)]
    create: bool,
    #[arg(short, long)]
    open: bool,
}

fn solve(paths: &Vec<String>) -> Result<(), Box<dyn Error>> {
    println!("Solving {} and outputting to {}", paths[0], paths[1]);
    Ok(())
}

fn create(path: &String) -> Result<(), Box<dyn Error>> {
    println!("Creating into file {}", path);
    Ok(())
}
