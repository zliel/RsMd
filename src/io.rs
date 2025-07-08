//! This module provides functionality to related to reading/writing files.

use std::fs;
use std::{
    error::Error,
    fs::{File, ReadDir, create_dir_all, read_dir},
    io::{Read, Write},
};

pub fn read_input_dir(input_dir: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let entries: ReadDir = read_dir(input_dir)
        .map_err(|e| format!("Failed to read input directory '{}': {}", input_dir, e))?;

    // Collect the contents of all markdown files in the directory
    let mut file_contents: Vec<(String, String)> = Vec::new();
    for entry in entries {
        let entry = entry
            .map_err(|e| format!("Failed to read entry in directory '{}': {}", input_dir, e))?;
        let file_path = entry.path();
        let file_name = file_path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| {
                format!(
                    "Failed to get file name from path '{}'",
                    file_path.display()
                )
            })?
            .to_string();

        if file_path.extension().and_then(|s| s.to_str()) == Some("md") {
            let contents = read_file(file_path.to_str().unwrap())
                .map_err(|e| format!("Failed to read file '{}': {}", file_path.display(), e))?;
            file_contents.push((file_name, contents));
        }
    }

    Ok(file_contents)
}

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

/// Copies a CSS file to the specified output directory.
///
/// # Arguments
/// * `input_file_path` - The path of the CSS file to copy, taken from the config
/// * `output_dir` - The directory where the CSS file should be copied, taken from the CLI
///
/// # Returns
/// Returns a `Result` indicating success or failure. If successful, the CSS file has been copied to the
/// output directory.
pub fn copy_css_to_output_dir(input_file_path: &str, output_dir: &str) -> Result<(), String> {
    let file_name = input_file_path
        .rsplit('/')
        .next()
        .ok_or("Failed to extract filename from input path")?;

    let output_file_path = format!("{}/{}", output_dir, file_name);
    fs::copy(input_file_path, &output_file_path)
        .map_err(|e| format!("Failed to copy file: {}", e))?;

    Ok(())
}

/// Writes a default CSS file to the specified output directory.
pub fn write_default_css_file(output_dir: &str) -> Result<(), String> {
    let css_content = generate_default_css();
    let css_file_path = format!("{}/styles.css", output_dir);

    let mut file =
        File::create(&css_file_path).map_err(|e| format!("Failed to create CSS file: {}", e))?;

    file.write_all(css_content.as_bytes())
        .map_err(|e| format!("Failed to write to CSS file: {}", e))?;

    Ok(())
}

/// Generates a default CSS stylesheet as a string.
pub fn generate_default_css() -> String {
    r#"
    body {
        font-family: Arial, sans-serif;
        line-height: 1.6;
        margin: 0;
        padding: 20px;
    }
    h1, h2, h3 {
        color: #333;
    }
    p {
        margin: 0 0 10px;
    }
    a {
        color: #007bff;
        text-decoration: none;
    }
    a:hover {
        text-decoration: underline;
    }
    pre {
        background-color: #f8f9fa;
        padding: 10px;
        border-radius: 5px;
        overflow-x: auto;
    }
    code {
        font-family: monospace;
        background-color: #f8f9fa;
        padding: 2px 4px;
        border-radius: 3px;
    }
    blockquote {
        border-left: 4px solid #ccc;
        padding-left: 10px;
        color: #666;
        margin: 0 0 10px;
    }
    ul, ol {
        margin: 0 0 10px 20px;
    }
    li {
        margin: 0 0 5px;
    }
    hr {
        border: none;
        border-top: 1px solid #ccc;
        margin: 20px 0;
    }
    "#
    .to_string()
}
