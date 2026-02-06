//! Main processing pipeline for ascfix.

use crate::cli::Args;
use crate::io;
use anyhow::Result;
use std::path::Path;

/// Exit code for when check mode detects differences.
pub const CHECK_FAILED_EXIT_CODE: i32 = 1;

/// Exit code for success.
pub const SUCCESS_EXIT_CODE: i32 = 0;

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
    /// Routes to the appropriate processor based on the configured mode.
    ///
    /// # Errors
    ///
    /// Returns an error if file reading/writing fails.
    #[allow(dead_code)] // Reason: Part of public API for library usage
    pub fn process_file(&self, path: &Path) -> Result<String> {
        let content = io::read_markdown(path)?;
        let processed = crate::modes::process_by_mode(&self.args.mode, &content);
        Ok(processed)
    }

    /// Process all files specified in arguments.
    ///
    /// In check mode, returns `CHECK_FAILED_EXIT_CODE` if any file needs fixing.
    /// In other modes, writes files and returns `SUCCESS_EXIT_CODE`.
    ///
    /// # Errors
    ///
    /// Returns an error if any file processing fails.
    pub fn process_all(&self) -> Result<i32> {
        let mut any_needs_fixing = false;

        for file_path in &self.args.files {
            let content = io::read_markdown(file_path)?;
            let processed = crate::modes::process_by_mode(&self.args.mode, &content);

            // Check if file needs fixing
            if crate::modes::content_needs_fixing(&content, &processed) {
                any_needs_fixing = true;

                if self.args.check {
                    // In check mode, just report without writing
                    eprintln!("File needs fixing: {}", file_path.display());
                } else {
                    // Normal mode: write the file
                    if self.args.in_place {
                        io::write_markdown(file_path, &processed)?;
                    } else {
                        println!("{processed}");
                    }
                }
            } else if !self.args.check && !self.args.in_place {
                // File doesn't need fixing and we're in normal output mode
                println!("{processed}");
            }
        }

        // Return appropriate exit code
        if self.args.check && any_needs_fixing {
            Ok(CHECK_FAILED_EXIT_CODE)
        } else {
            Ok(SUCCESS_EXIT_CODE)
        }
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
