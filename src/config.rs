//! This module handles the configuration I/O for the application.
use serde::Deserialize;

use crate::CONFIG;

/// Represents the global configuration for the application.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub lexer: LexerConfig,
    pub html: HtmlConfig,
}

/// Manages all configuration for tokenization
#[derive(Debug, Deserialize)]
pub struct LexerConfig {
    pub tab_size: usize,
}

/// Manages all configuration for HTML generation
#[derive(Debug, Deserialize)]
pub struct HtmlConfig {
    #[serde(default = "default_css")]
    pub css_file: String,
    pub favicon_file: String,
}

/// Sets the default CSS file to "default" in the case that the `css_file` field is omitted
fn default_css() -> String {
    "default".to_string()
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

/// Initializes the global configuration from the specified file path
///
/// # Arguments
/// * `config_path` - The path to the configuration file.
///
/// # Returns
/// Returns a `Result` indicating success or failure. If successful, a global `CONFIG` has been
/// initialized.
pub fn init_config(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    CONFIG.get_or_init(|| {
        Config::from_file(config_path).unwrap_or_else(|err| {
            eprintln!("Error loading config: {}", err);
            std::process::exit(1);
        })
    });
    Ok(())
}
