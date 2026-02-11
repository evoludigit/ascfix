//! Configuration file support for ascfix
//!
//! Supports TOML configuration files for customizing behavior.

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Default configuration values
pub const DEFAULT_MAX_LINE_LENGTH: usize = 120;
pub const DEFAULT_BOX_PADDING: usize = 1;

/// Configuration for diagram formatting
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormattingConfig {
    /// Maximum line length for wrapping
    pub max_line_length: usize,
    /// Padding inside boxes
    pub box_padding: usize,
    /// Whether to preserve Unicode characters
    pub preserve_unicode: bool,
    /// Whether to validate diagrams
    pub validate_diagrams: bool,
}

impl Default for FormattingConfig {
    fn default() -> Self {
        Self {
            max_line_length: DEFAULT_MAX_LINE_LENGTH,
            box_padding: DEFAULT_BOX_PADDING,
            preserve_unicode: true,
            validate_diagrams: false,
        }
    }
}

/// Main configuration structure
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Config {
    /// Formatting options
    pub formatting: FormattingConfig,
    /// Whether to enable flowchart support
    pub enable_flowcharts: bool,
    /// Whether to enable sequence diagrams
    pub enable_sequence_diagrams: bool,
}

impl Config {
    /// Load configuration from a TOML file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or if the TOML is invalid.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Load configuration from current directory or parents
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue accessing the filesystem or parsing config files.
    pub fn load_from_cwd() -> Result<Self, Box<dyn std::error::Error>> {
        let mut current = std::env::current_dir()?;

        loop {
            let config_path = current.join(".ascfix.toml");
            if config_path.exists() {
                return Self::from_file(config_path);
            }

            if !current.pop() {
                break;
            }
        }

        // Return default if no config found
        Ok(Self::default())
    }

    /// Save configuration to a TOML file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written or if serialization fails.
    #[allow(dead_code)] // Reserved for future configuration editing features
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let toml = toml::to_string_pretty(self)?;
        std::fs::write(path, toml)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.formatting.max_line_length, DEFAULT_MAX_LINE_LENGTH);
        assert_eq!(config.formatting.box_padding, DEFAULT_BOX_PADDING);
        assert!(config.formatting.preserve_unicode);
        assert!(!config.enable_flowcharts);
    }

    #[test]
    fn test_load_config_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let config_content = r"enable_flowcharts = true
enable_sequence_diagrams = false

[formatting]
max_line_length = 100
box_padding = 2
preserve_unicode = false
validate_diagrams = true
";

        temp_file.write_all(config_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let config = Config::from_file(temp_file.path()).unwrap();

        assert_eq!(config.formatting.max_line_length, 100);
        assert_eq!(config.formatting.box_padding, 2);
        assert!(!config.formatting.preserve_unicode);
        assert!(config.formatting.validate_diagrams);
        assert!(config.enable_flowcharts);
        assert!(!config.enable_sequence_diagrams);
    }
}
