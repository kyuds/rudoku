// use clap::Parser;

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     // Command to run
//     #[arg]
// }

fn main() {
    println!("This is the command line interface!");
}

/*
cargo run --bin cli -- solve samples/sample1.txt answer1.txt
cargo run --bin cli -- solve samples/sample1.txt
cargo run --bin cli -- create problem1.txt
cargo run --bin cli -- solvable samples/sample1.txt
cargo run --bin cli -- ga_solve samples/sample1.txt answer1.txt
cargo run --bin cli -- ga_solve samples/sample1.txt
*/
