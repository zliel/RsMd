mod io;
mod lexer;
mod parser;
mod types;

use io::read_file;
use std::env::args;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("Error: Missing file path argument.");
        eprintln!("Usage: cargo run <file_path>");
        std::process::exit(1);
    }

    let file_path = &args[1];
}
