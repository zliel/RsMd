mod config;
mod io;
mod lexer;
mod parser;
mod types;
mod utils;

use io::read_file;
use std::error::Error;
use std::sync::OnceLock;
use std::{env::args, process};

use crate::config::Config;
use crate::lexer::tokenize;
use crate::parser::{group_lines_to_blocks, parse_blocks};
use crate::types::Token;

static CONFIG: OnceLock<Config> = OnceLock::new();

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("Error: Missing file path argument.");
        eprintln!("Usage: cargo run <file_path>");
        process::exit(1);
    }

    let file_path = &args[1];
    let file_contents = read_file(file_path)?;

    // Tokenizing
    let mut tokenized_lines: Vec<Vec<Token>> = Vec::new();
    for line in file_contents.split('\n') {
        tokenized_lines.push(tokenize(line));
    }

    // Parsing
    let blocks = group_lines_to_blocks(tokenized_lines);

    let parsed_elements = parse_blocks(blocks);
    parsed_elements
        .iter()
        .for_each(|block| println!("{:?}", block));

    Ok(())
}
