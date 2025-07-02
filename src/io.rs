//! This module provides functionality to related to reading/writing files.

use std::{fs::File, io::Read};

/// Reads the contents of a file into a String.
///
/// # Arguments
/// * `file_path` - The path of the file to read.
///
/// # Returns
/// Returns the contents of the file as a String instance.
pub fn read_file(file_path: &str) -> String {
    let mut md_file: File =
        File::open(file_path).unwrap_or_else(|_| panic!("Couldn't open file: \"{file_path}\""));

    let mut contents = String::new();
    md_file
        .read_to_string(&mut contents)
        .expect("Couldn't read file into string");

    contents
}
