mod game;

// use rudoku::board::Board;
use clap::{Parser, Subcommand, Args};
// use crate::game;

// rudoku {cli | gui} --{(crte|slv)|(crte|opn)} PATHS...

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);
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

