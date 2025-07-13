//! This module provides functionality to related to reading/writing files.

use std::fs;
use std::path::PathBuf;
use std::{
    error::Error,
    fs::{File, ReadDir, create_dir_all, read_dir},
    io::{Read, Write},
    path::Path,
};

use dirs::config_dir;

use crate::config::Config;
use crate::html_generator::generate_default_css;

/// Reads all markdown files from the specified input directory and returns their contents.
///
/// # Arguments
/// * `input_dir` - The directory containing markdown files.
///
/// # Returns
/// Returns a `Result` containing a vector of tuples, where each tuple contains the file name
/// and its contents as a string.
pub fn read_input_dir(
    input_dir: &str,
    run_recursively: &bool,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    if *run_recursively {
        // If recursive, visit all subdirectories
        let mut file_contents: Vec<(String, String)> = Vec::new();
        let input_dir = Path::new(input_dir);
        visit_dir(Path::new(input_dir), input_dir, &mut file_contents)?;

        Ok(file_contents)
    } else {
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
}

fn visit_dir(
    dir: &Path,
    base: &Path,
    file_contents: &mut Vec<(String, String)>,
) -> Result<(), Box<dyn Error>> {
    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            visit_dir(&path, base, file_contents)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let rel_path = path
                .strip_prefix(base)
                .map_err(|e| format!("Failed to strip base path: {}", e))?
                .to_string_lossy()
                .to_string();
            let contents = read_file(path.to_str().unwrap())
                .map_err(|e| format!("Failed to read file '{}': {}", path.display(), e))?;

            file_contents.push((rel_path, contents));
        }
    }

    Ok(())
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
    input_filepath: &str,
) -> Result<(), Box<dyn Error>> {
    println!("Writing output to directory: {}", output_dir);
    let output_dir = Path::new(output_dir).join(input_filepath);

    if let Some(parent) = output_dir.parent() {
        create_dir_all(parent).map_err(|e| {
            format!(
                "Failed to create output directory '{}': {}",
                parent.display(),
                e
            )
        })?;
    }

    let mut output_file = File::create(&output_dir).map_err(|e| {
        format!(
            "Failed to create output file '{}': {}",
            output_dir.display(),
            e
        )
    })?;

    output_file.write_all(html.as_bytes()).map_err(|e| {
        format!(
            "Failed to write to output file '{}': {}",
            output_dir.display(),
            e
        )
    })?;

    println!("HTML written to: {}", output_dir.display());
    Ok(())
}

/// Copies a file from the input path to the specified output directory, optionally creating a
/// subdirectory.
///
/// # Arguments
/// * `input_file_path` - The path of the file to copy.
/// * `output_dir` - The directory where the file should be copied.
/// * `subdir` - An optional subdirectory within the output directory.
/// * `base_dir` - An optional base directory to resolve relative paths.
///
/// # Returns
/// Returns a `Result` indicating success or failure. If successful, the file is copied to the
/// output directory.
pub fn copy_file_to_output_dir(
    input_file_path: &str,
    output_dir: &str,
    subdir: Option<&str>,
    base_dir: Option<&str>,
) -> Result<(), String> {
    use std::path::PathBuf;

    let abs_input_path = if let Some(base) = base_dir {
        let input_path = Path::new(input_file_path);
        if input_path.is_absolute() {
            input_path.to_path_buf()
        } else {
            Path::new(base).join(input_file_path)
        }
    } else {
        PathBuf::from(input_file_path)
    };

    let file_name = abs_input_path
        .file_name()
        .ok_or("Failed to extract filename from input path")?;

    let mut output_file_path = PathBuf::from(output_dir);
    if let Some(sub) = subdir {
        output_file_path.push(sub);
        create_dir_all(&output_file_path)
            .map_err(|e| format!("Failed to create subdirectory '{}': {}", sub, e))?;
    } else {
        create_dir_all(&output_file_path)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }
    output_file_path.push(file_name);

    fs::copy(&abs_input_path, &output_file_path)
        .map_err(|e| format!("Failed to copy file: {}", e))?;

    Ok(())
}

/// Copies a favicon file to the specified output directory.
pub fn copy_favicon_to_output_dir(input_file_path: &str, output_dir: &str) -> Result<(), String> {
    copy_file_to_output_dir(input_file_path, output_dir, Some("media"), None)
}

/// Copies an image file to the specified output directory.
pub fn copy_image_to_output_dir(
    input_file_path: &str,
    output_dir: &str,
    md_dir: &str,
) -> Result<(), String> {
    copy_file_to_output_dir(input_file_path, output_dir, Some("media"), Some(md_dir))
}

/// Copies a CSS file to the specified output directory.
pub fn copy_css_to_output_dir(input_file_path: &str, output_dir: &str) -> Result<(), String> {
    copy_file_to_output_dir(input_file_path, output_dir, None, None)
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

pub fn get_config_path() -> Result<PathBuf, String> {
    let mut config_path = config_dir().unwrap_or_else(|| PathBuf::from("."));

    config_path.push("rustmark");
    create_dir_all(&config_path).map_err(|e| {
        format!(
            "Failed to create config directory '{}': {}",
            config_path.display(),
            e
        )
    })?;
    config_path.push("config.toml");

    Ok(config_path)
}

pub fn does_config_exist() -> Result<bool, String> {
    let config_path = get_config_path()?;

    let config_exists = fs::exists(&config_path).map_err(|e| {
        format!(
            "Failed to check if config file exists at '{}': {}",
            config_path.display(),
            e
        )
    })?;

    Ok(config_exists)
}

pub fn write_default_config(default_config: &Config) -> Result<(), String> {
    let config_path = get_config_path()?;

    if does_config_exist()? {
        return Ok(());
    }

    println!(
        "Config file does not exist, creating default config at: {}",
        config_path.display()
    );

    let mut file = File::create(&config_path).map_err(|e| {
        format!(
            "Failed to create config file at '{}': {}",
            config_path.display(),
            e
        )
    })?;

    let default_config_content = toml::to_string_pretty(&default_config)
        .map_err(|e| format!("Failed to serialize default config: {}", e))?;

    file.write_all(default_config_content.as_bytes())
        .map_err(|e| format!("Failed to write to config file: {}", e))?;

    println!("Default config file created at: {}", config_path.display());

    Ok(())
}
