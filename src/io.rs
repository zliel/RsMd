//! This module provides functionality to related to reading/writing files.

use std::{fs::File, io::Read};

/// Reads the contents of a file into a String.
///
/// # Arguments
/// * `file_path` - The path of the file to read.
///
/// # Returns
/// Returns Ok(String) if successful, or an Err(String) with an error message if it fails.
pub fn read_file(file_path: &str) -> Result<String, String> {
    let mut md_file: File =
        File::open(file_path).map_err(|e| format!("Failed to open file '{}': {}", file_path, e))?;

    let mut contents = String::new();
    md_file
        .read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    Ok(contents)
}
