//! This module handles the configuration I/O for the application.
use toml::Table;

/// Represents the global configuration for the application.
#[derive(Debug)]
pub struct Config {
    pub lexer: LexerConfig,
}

#[derive(Debug)]
pub struct LexerConfig {
    pub tab_size: usize,
}

impl Config {
    /// Creates a new `Config` instance with the specified tab size.
    ///
    /// # Arguments
    /// * `file_path` - The path to the configuration file.
    ///
    /// # Returns
    /// Returns a `Result` containing the `Config` instance if successful
    pub fn from_file(file_path: &str) -> Result<Self, String> {
        let contents = std::fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        let config: Table =
            toml::from_str(&contents).map_err(|e| format!("Failed to parse config file: {}", e))?;

        let lexer_table = config
            .get("lexer")
            .and_then(|val| val.as_table())
            .ok_or("Missing `[lexer]` section heading in config")?;

        let tab_size: usize = lexer_table
            .get("tab_size")
            .and_then(|val| val.as_integer())
            .map(|val| val as usize)
            .ok_or("Missing or invalid 'tab_size' in config")?;

        Ok(Config {
            lexer: LexerConfig { tab_size },
        })
    }
}
