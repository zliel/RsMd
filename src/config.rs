//! This module handles the configuration I/O for the application.
use log::{error, info};

use log::{error, info, warn};
use serde::{Deserialize, Serialize};

use crate::CONFIG;
use crate::io::{does_config_exist, get_config_path, write_default_config};

/// Represents the global configuration for the application.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    #[serde(default)]
    pub lexer: LexerConfig,
    #[serde(default)]
    pub html: HtmlConfig,
}

/// Manages all configuration for tokenization
#[derive(Debug, Deserialize, Serialize)]
pub struct LexerConfig {
    #[serde(default = "default_tab_size")]
    pub tab_size: usize,
}

impl Default for LexerConfig {
    fn default() -> Self {
        LexerConfig { tab_size: 4 }
    }
}

fn default_tab_size() -> usize {
    4
}

/// Manages all configuration for HTML generation
#[derive(Debug, Deserialize, Serialize)]
pub struct HtmlConfig {
    #[serde(default = "default_css")]
    pub css_file: String,
    #[serde(default)]
    pub favicon_file: String,
    #[serde(default)]
    pub use_prism: bool,
    #[serde(default = "default_prism_theme")]
    pub prism_theme: String,
    #[serde(default = "sanitize_by_default")]
    pub sanitize_html: bool,
}

impl Default for HtmlConfig {
    fn default() -> Self {
        HtmlConfig {
            css_file: default_css(),
            favicon_file: "".to_string(),
            use_prism: false,
            prism_theme: default_prism_theme(),
            sanitize_html: sanitize_by_default(),
        }
    }
}

/// Sets the default PrismJS theme to "vsc-dark-plus" in `config.toml`
fn default_prism_theme() -> String {
    "vsc-dark-plus".to_string()
}

/// Sets `sanitize_html` to true by default in `config.toml`
fn sanitize_by_default() -> bool {
    true
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

            validate_config(file_path, contents, &config)?;

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

            validate_config(&config_path.to_string_lossy(), contents, &config)?;

            Ok(config)
        } else {
            warn!(
                "No config file found, writing default config to: {}",
                config_path.to_string_lossy()
            );
            let default_config = Config::default();

            write_default_config(&default_config)
                .map_err(|e| format!("Failed to write default config: {}", e))?;

            Ok(default_config)
        }
    }
}

/// Validates the configuration by checking if the original config file matches the filled config
///
/// If the original config is missing fields, it updates the file with any missing fields
fn validate_config(file_path: &str, contents: String, config: &Config) -> Result<(), String> {
    let original: toml::Value =
        toml::from_str(&contents).map_err(|e| format!("Failed to parse config file: {}", e))?;

    let filled: toml::Value = toml::Value::try_from(config)
        .map_err(|e| format!("Failed to convert config to TOML: {}", e))?;

    if original != filled {
        warn!("Config is missing fields, writing updated config to: {file_path}");
        let serialized = toml::to_string_pretty(config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        std::fs::write(file_path, serialized)
            .map_err(|e| format!("Failed to write config file: {}", e))?;
    }

    Ok(())
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
