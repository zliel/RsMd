mod config;
mod html_generator;
mod io;
mod lexer;
mod parser;
mod types;
mod utils;

use clap::{Parser, command};
use std::error::Error;
use std::path::Path;
use std::sync::OnceLock;

use crate::config::{Config, init_config};
use crate::html_generator::{generate_html, generate_index};
use crate::io::{
    copy_css_to_output_dir, copy_favicon_to_output_dir, read_input_dir, write_default_css_file,
    write_html_to_file,
};
use crate::lexer::tokenize;
use crate::parser::{group_lines_to_blocks, parse_blocks};
use crate::types::Token;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Parser, Debug)]
#[command(
    author = "Zackary Liel",
    version = "1.0.0",
    about = "A Commonmark compliant markdown parser and static site generator.",
    override_usage = "rust_mark [OPTIONS] <INPUT_DIR>"
)]
struct Cli {
    #[arg(value_name = "INPUT_DIR")]
    input_dir: String,
    #[arg(short, long, default_value = "")]
    config: String,
    #[arg(short, long, default_value = "./output")]
    output_dir: String,
    #[arg(short, long, default_value = "false")]
    recursive: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let input_dir = &cli.input_dir;
    let config_path = &cli.config;
    let run_recursively = &cli.recursive;

    // Setup
    init_config(config_path)?;
    let file_contents = read_input_dir(input_dir, run_recursively)?;
    let mut file_names: Vec<String> = Vec::new();

    for (file_path, file_content) in file_contents {
        generate_static_site(&cli, &file_path, file_content)?;
        file_names.push(file_path);
    }

    let index_html = generate_index(&file_names);
    write_html_to_file(&index_html, &cli.output_dir, "index.html")?;

    let css_file = CONFIG.get().unwrap().html.css_file.clone();
    if css_file != "default" && !css_file.is_empty() {
        println!("Using custom CSS file: {}", css_file);
        copy_css_to_output_dir(&css_file, &cli.output_dir)?;
    } else {
        println!("Using default CSS file.");
        write_default_css_file(&cli.output_dir)?;
    }

    let favicon_path = CONFIG.get().unwrap().html.favicon_file.clone();
    if !favicon_path.is_empty() {
        println!("Copying favicon from: {}", favicon_path);
        copy_favicon_to_output_dir(&favicon_path, &cli.output_dir)?;
    } else {
        println!("No favicon specified in config.");
    }

    Ok(())
}

fn generate_static_site(
    cli: &Cli,
    file_path: &str,
    file_contents: String,
) -> Result<(), Box<dyn Error>> {
    // Tokenizing
    let mut tokenized_lines: Vec<Vec<Token>> = Vec::new();
    for line in file_contents.split('\n') {
        tokenized_lines.push(tokenize(line));
    }

    // Parsing
    let blocks = group_lines_to_blocks(tokenized_lines);
    let parsed_elements = parse_blocks(blocks);

    // HTML Generation
    let generated_html = generate_html(
        file_path,
        parsed_elements,
        &cli.output_dir,
        &cli.input_dir,
        file_path,
    );

    let html_relative_path = if file_path.ends_with(".md") {
        file_path.trim_end_matches(".md").to_string() + ".html"
    } else {
        file_path.to_string() + ".html"
    };

    let output_path = Path::new(&cli.output_dir).join(&html_relative_path);
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    write_html_to_file(&generated_html, &cli.output_dir, &html_relative_path)?;

    Ok(())
}
