//! This module provides functionality to related to reading/writing files.

use std::{
    error::Error,
    fs::{File, create_dir_all},
    io::{Read, Write},
};

/// Reads the contents of a file into a String.
///
/// # Arguments
/// * `file_path` - The path of the file to read.
///
/// # Returns
/// Returns a `Result` containing the file contents as a string on success,
/// or an error message on failure.
pub fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut md_file: File =
        File::open(file_path).map_err(|e| format!("Failed to open file '{}': {}", file_path, e))?;

    let mut contents = String::new();
    md_file
        .read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    Ok(contents)
}

/// Writes the provided HTML string to a file in the specified output directory.
///
/// # Arguments
/// * `html` - The HTML content to write to the file.
/// * `output_dir` - The directory where the HTML file should be saved.
/// * `input_filename` - The name of the input markdown file (used to derive the output filename).
///
/// # Returns
/// Returns a `Result` indicating success or failure.
pub fn write_html_to_file(
    html: &str,
    output_dir: &str,
    input_filename: &str,
) -> Result<(), Box<dyn Error>> {
    println!("Writing output to directory: {}", output_dir);

    // Ensure the output directory exists
    create_dir_all(output_dir)
        .map_err(|e| format!("Failed to create directory '{}': {}", output_dir, e))?;

    // Get only the filename without the extension and without the path
    let input_filename = input_filename
        .rsplit('/')
        .next()
        .ok_or("Failed to extract filename from input path")?
        .trim_end_matches(".md");

    let output_file_path = format!("{}/{}.html", output_dir, input_filename);
    let mut output_file = File::create(&output_file_path)
        .map_err(|e| format!("Failed to create file '{}': {}", output_file_path, e))?;

    output_file
        .write_all(html.as_bytes())
        .map_err(|e| format!("Failed to write to file '{}': {}", output_file_path, e))?;

    println!("HTML written to: {}", output_file_path);
    Ok(())
}
