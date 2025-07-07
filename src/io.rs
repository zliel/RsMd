//! This module provides functionality to related to reading/writing files.

use std::{error::Error, fs::File, io::Read};

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
