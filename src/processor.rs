//! Main processing pipeline for ascfix.

use crate::cli::Args;
use crate::discovery::FileDiscovery;
use crate::io;
use crate::output::{FileResult, ProcessingResults, ProcessingStats, StatsOutput};
use anyhow::Result;
use std::path::Path;

/// Exit code for when check mode detects differences.
pub const CHECK_FAILED_EXIT_CODE: i32 = 1;

/// Exit code for success.
pub const SUCCESS_EXIT_CODE: i32 = 0;

/// Main processor for handling file transformations.
pub struct Processor {
    args: Args,
    config: crate::config::Config,
}

impl Processor {
    /// Create a new processor with the given arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration file cannot be loaded or parsed.
    pub fn new(args: Args) -> Result<Self> {
        let config = crate::config::Config::load_from_cwd()
            .map_err(|e| anyhow::anyhow!("Failed to load config: {e}"))?;
        Ok(Self { args, config })
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
        // Determine if we should repair fences (--all implies --fences)
        let repair_fences = self.args.fences || self.args.all;
        // Determine the mode (--all implies --mode=diagram)
        let mode = if self.args.all {
            &crate::cli::Mode::Diagram
        } else {
            &self.args.mode
        };
        let processed = crate::modes::process_by_mode(mode, &content, repair_fences, &self.config);
        Ok(processed)
    }

    /// Process all files specified in arguments.
    ///
    /// Uses `FileDiscovery` to find files matching configured extensions.
    /// Collects errors and continues processing, reporting all at the end.
    /// In check mode, returns `CHECK_FAILED_EXIT_CODE` if any file needs fixing.
    /// In other modes, writes files and returns appropriate exit code.
    ///
    /// # Errors
    ///
    /// Returns an error if file discovery fails or if there are fatal I/O errors.
    pub fn process_all(&self) -> Result<i32> {
        // Create file discovery with configured extensions and gitignore setting
        let discovery = FileDiscovery::new(self.args.ext.clone(), !self.args.no_gitignore);

        // Discover files matching the criteria
        let file_paths = discovery.discover(&self.args.paths)?;

        if file_paths.is_empty() {
            if self.args.verbose {
                crate::output::log_warning("No files matching extensions found");
            } else {
                eprintln!("No files matching extensions found");
            }
            return Ok(SUCCESS_EXIT_CODE);
        }

        if self.args.verbose {
            crate::output::log_verbose(&format!("Found {} files to process", file_paths.len()));
        }

        let mut stats = ProcessingStats::new();
        let mut file_results: Vec<FileResult> = Vec::new();
        let mut any_needs_fixing = false;

        for file_path in file_paths {
            let result = self.process_single_file_enhanced(&file_path, &mut any_needs_fixing, &mut stats);
            file_results.push(result);
        }

        // Handle different output modes
        if self.args.json {
            // JSON output mode
            let results = ProcessingResults {
                files: file_results,
                stats: StatsOutput::from(&stats),
            };
            println!("{}", serde_json::to_string_pretty(&results)?);
        } else if self.args.list_files {
            // List files mode - output only files that need fixing
            for result in &file_results {
                if result.is_modified() {
                    println!("{}", result.file_path());
                }
            }
        }

        // Print summary if requested
        if self.args.summary && !self.args.json {
            stats.print_summary();
        }

        // Return appropriate exit code
        if stats.error_files > 0 || (self.args.check && any_needs_fixing) {
            Ok(CHECK_FAILED_EXIT_CODE)
        } else {
            Ok(SUCCESS_EXIT_CODE)
        }
    }

    /// Process a single file and return detailed result.
    ///
    /// Enhanced version that tracks statistics and supports all output modes.
    fn process_single_file_enhanced(
        &self,
        file_path: &Path,
        any_needs_fixing: &mut bool,
        stats: &mut ProcessingStats,
    ) -> FileResult {
        let file_str = file_path.display().to_string();

        // Check file size if max_size is set
        if let Some(max_size) = self.args.max_size {
            if let Ok(metadata) = file_path.metadata() {
                let file_size = metadata.len();
                if file_size > max_size {
                    if self.args.verbose {
                        crate::output::log_warning(&format!(
                            "Skipping {} (exceeds maximum size: {} bytes, max: {} bytes)",
                            file_path.display(),
                            file_size,
                            max_size
                        ));
                    }
                    stats.record_skipped();
                    return FileResult::Skipped {
                        file: file_str,
                        reason: format!("File size ({file_size} bytes) exceeds maximum ({max_size} bytes)"),
                    };
                }
            }
        }

        // Read file content
        let content = match io::read_markdown(file_path) {
            Ok(c) => c,
            Err(e) => {
                if self.args.verbose {
                    crate::output::log_error(&format!("Failed to read {}: {}", file_path.display(), e));
                }
                stats.record_error();
                return FileResult::Error {
                    file: file_str,
                    error: e.to_string(),
                };
            }
        };

        // Process the content
        let repair_fences = self.args.fences || self.args.all;
        let mode = if self.args.all {
            &crate::cli::Mode::Diagram
        } else {
            &self.args.mode
        };
        let processed = crate::modes::process_by_mode(mode, &content, repair_fences, &self.config);

        // Check if file needs fixing
        if crate::modes::content_needs_fixing(&content, &processed) {
            *any_needs_fixing = true;
            stats.record_modified();

            if self.args.verbose {
                crate::output::log_success(&format!("Modified: {}", file_path.display()));
            }

            // Handle different output modes
            if self.args.diff {
                crate::output::print_diff(file_path, &content, &processed);
            } else if self.args.check {
                // In check mode, just report without writing
                if !self.args.list_files && !self.args.json {
                    eprintln!("File needs fixing: {}", file_path.display());
                }
            } else if self.args.in_place {
                // Write the file
                if let Err(e) = io::write_markdown(file_path, &processed) {
                    stats.record_error();
                    return FileResult::Error {
                        file: file_str,
                        error: format!("Failed to write: {e}"),
                    };
                }
            } else if !self.args.json && !self.args.list_files {
                // Output to stdout
                println!("{processed}");
            }

            FileResult::Modified {
                file: file_str,
                transformations: None,
            }
        } else {
            stats.record_unchanged();

            if self.args.verbose {
                crate::output::log_verbose(&format!("Unchanged: {}", file_path.display()));
            }

            if !self.args.check && !self.args.in_place && !self.args.json && !self.args.list_files {
                // File doesn't need fixing and we're in normal output mode
                println!("{processed}");
            }

            FileResult::Unchanged {
                file: file_str,
            }
        }
    }

    /// Process a single file and report if it needs fixing.
    ///
    /// Returns Ok(()) if processing succeeded, Err if there was a fatal error.
    /// Updates `any_needs_fixing` if the file needs to be fixed.
    /// Skips files that exceed `max_size` without error.
    #[allow(dead_code)] // Reason: Kept for backward compatibility
    fn process_single_file(&self, file_path: &Path, any_needs_fixing: &mut bool) -> Result<()> {
        // Check file size if max_size is set
        if let Some(max_size) = self.args.max_size {
            let file_size = file_path.metadata()?.len();
            if file_size > max_size {
                eprintln!(
                    "Skipping file (exceeds maximum size: {} bytes, max: {} bytes): {}",
                    file_size,
                    max_size,
                    file_path.display()
                );
                return Ok(());
            }
        }

        let content = io::read_markdown(file_path)?;
        // Determine if we should repair fences (--all implies --fences)
        let repair_fences = self.args.fences || self.args.all;
        // Determine the mode (--all implies --mode=diagram)
        let mode = if self.args.all {
            &crate::cli::Mode::Diagram
        } else {
            &self.args.mode
        };
        let processed = crate::modes::process_by_mode(mode, &content, repair_fences, &self.config);

        // Check if file needs fixing
        if crate::modes::content_needs_fixing(&content, &processed) {
            *any_needs_fixing = true;

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
        let processor = Processor::new(args).unwrap();
        assert_eq!(processor.args.paths.len(), 1);
    }

    #[test]
    fn test_max_size_enforcement() -> Result<()> {
        use clap::Parser;
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.md");
        let content = "# Test\n\nSmall file";
        fs::write(&file_path, content)?;

        // Create processor with max_size: 5 bytes (smaller than file)
        let args = Args::try_parse_from(vec![
            "ascfix",
            file_path.to_str().unwrap(),
            "--max-size",
            "5",
        ])
        .expect("Failed to parse args");
        let processor = Processor::new(args)?;

        // Process should skip the file due to size limit
        let exit_code = processor.process_all()?;
        assert_eq!(exit_code, 0);
        Ok(())
    }

    #[test]
    fn test_max_size_allows_valid_file() -> Result<()> {
        use clap::Parser;
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.md");
        let content = "# Test";
        fs::write(&file_path, content)?;

        // Create processor with max_size: 1000 bytes (larger than file)
        let args = Args::try_parse_from(vec![
            "ascfix",
            file_path.to_str().unwrap(),
            "--max-size",
            "1000",
        ])
        .expect("Failed to parse args");
        let processor = Processor::new(args)?;

        // Process should handle the file normally
        let exit_code = processor.process_all()?;
        assert_eq!(exit_code, 0);
        Ok(())
    }
}
