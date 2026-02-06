//! Main processing pipeline for ascfix.

use crate::cli::Args;
use crate::io;
use anyhow::Result;
use std::path::Path;

/// Main processor for handling file transformations.
pub struct Processor {
    args: Args,
}

impl Processor {
    /// Create a new processor with the given arguments.
    pub const fn new(args: Args) -> Self {
        Self { args }
    }

    /// Process a single file.
    ///
    /// # Errors
    ///
    /// Returns an error if file reading/writing fails.
    #[allow(clippy::unused_self)] // Reason: Keep as method for API consistency
    pub fn process_file(&self, path: &Path) -> Result<String> {
        io::read_markdown(path)
    }

    /// Process all files specified in arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if any file processing fails.
    pub fn process_all(&self) -> Result<()> {
        for file_path in &self.args.files {
            let content = self.process_file(file_path)?;

            if self.args.in_place {
                io::write_markdown(file_path, &content)?;
            } else {
                println!("{content}");
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_round_trip_single_file() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.md");
        let original_content = "# Test Document\n\nSome content here.\n";
        fs::write(&file_path, original_content)?;

        // Read the file
        let read_content = io::read_markdown(&file_path)?;

        // Write it back
        io::write_markdown(&file_path, &read_content)?;

        // Verify it's identical
        let final_content = fs::read_to_string(&file_path)?;
        assert_eq!(final_content, original_content);
        Ok(())
    }

    #[test]
    fn test_processor_creates_instance() {
        use clap::Parser;
        let args = Args::try_parse_from(["ascfix", "test.md"]).expect("Failed to parse args");
        let processor = Processor::new(args);
        assert_eq!(processor.args.files.len(), 1);
    }
}
