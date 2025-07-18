//! This module handles the configuration I/O for the application.
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::CONFIG;
use crate::io::{does_config_exist, get_config_path, write_default_config};

/// Represents the global configuration for the application.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub lexer: LexerConfig,
    pub html: HtmlConfig,
}

/// Manages all configuration for tokenization
#[derive(Debug, Deserialize, Serialize)]
pub struct LexerConfig {
    pub tab_size: usize,
}

/// Manages all configuration for HTML generation
#[derive(Debug, Deserialize, Serialize)]
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
    /// * `file_path` - The path to the configuration file. If no file path is provided as a CLI
    ///   arg, it will check for a config file in the default config directory.
    ///
    /// # Returns
    /// Returns a `Result` containing the `Config` instance if successful
    pub fn from_file(file_path: &str) -> Result<Self, String> {
        // If the user provided a config file, try to load the config from it
        if !file_path.is_empty() {
            info!("Loading config from file: {}", file_path);
            let contents = std::fs::read_to_string(file_path)
                .map_err(|e| format!("Failed to read config file: {}", e))?;

            let config: Config = toml::from_str(&contents)
                .map_err(|e| format!("Failed to parse config file: {}", e))?;

            return Ok(config);
        }

        // If the user did not provide a config file, check if a config file exists in the config
        // directory
        if does_config_exist()? {
            let config_path =
                get_config_path().map_err(|e| format!("Failed to get config path: {}", e))?;

            let contents = std::fs::read_to_string(&config_path)
                .map_err(|e| format!("Failed to read config file: {}", e))?;

            let config: Config = toml::from_str(&contents)
                .map_err(|e| format!("Failed to parse config file: {}", e))?;

            Ok(config)
        } else {
            // Write the default config if it does not exist
            let default_config = Config {
                lexer: LexerConfig { tab_size: 4 },
                html: HtmlConfig {
                    css_file: default_css(),
                    favicon_file: String::new(),
                },
            };

            write_default_config(&default_config)
                .map_err(|e| format!("Failed to write default config: {}", e))?;

            Ok(default_config)
        }
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
            error!("Failed to load config: {}", err);
            std::process::exit(1);
        })
    });
    Ok(())
}
