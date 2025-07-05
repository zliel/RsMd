//! This module handles the configuration I/O for the application.
use serde::Deserialize;

use crate::CONFIG;

/// Represents the global configuration for the application.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub lexer: LexerConfig,
}

/// Manages all configuration for tokenization
#[derive(Debug, Deserialize)]
pub struct LexerConfig {
    pub tab_size: usize,
}

impl Config {
    /// Creates a new `Config` instance from the specified file path
    ///
    /// # Arguments
    /// * `file_path` - The path to the configuration file.
    ///
    /// # Returns
    /// Returns a `Result` containing the `Config` instance if successful
    pub fn from_file(file_path: &str) -> Result<Self, String> {
        let contents = std::fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        let config: Config =
            toml::from_str(&contents).map_err(|e| format!("Failed to parse config file: {}", e))?;

        Ok(config)
    }
}

pub fn init_config(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    CONFIG.get_or_init(|| {
        Config::from_file(config_path).unwrap_or_else(|err| {
            eprintln!("Error loading config: {}", err);
            std::process::exit(1);
        })
    });
    Ok(())
}
