//! Output formatting for different modes (summary, JSON, diff, etc.)

#![allow(clippy::missing_const_for_fn)] // Reason: Methods modify self, clippy suggestion is incorrect

use colored::Colorize;
use serde::{Deserialize, Serialize};
use similar::{ChangeTag, TextDiff};
use std::path::Path;

/// Statistics about processing results
#[derive(Debug, Clone, Default)]
#[allow(clippy::struct_field_names)] // Reason: Field names are clear and descriptive
pub struct ProcessingStats {
    pub total_files: usize,
    pub modified_files: usize,
    pub unchanged_files: usize,
    pub error_files: usize,
    pub skipped_files: usize,
}

impl ProcessingStats {
    /// Create a new stats tracker
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a file that was modified
    pub fn record_modified(&mut self) {
        self.total_files += 1;
        self.modified_files += 1;
    }

    /// Record a file that was unchanged
    pub fn record_unchanged(&mut self) {
        self.total_files += 1;
        self.unchanged_files += 1;
    }

    /// Record a file that had an error
    pub fn record_error(&mut self) {
        self.total_files += 1;
        self.error_files += 1;
    }

    /// Record a file that was skipped
    pub fn record_skipped(&mut self) {
        self.skipped_files += 1;
    }

    /// Print summary to stderr
    pub fn print_summary(&self) {
        eprintln!("\n{}", "Summary:".bold());
        eprintln!("  Total files processed: {}", self.total_files);

        if self.modified_files > 0 {
            eprintln!("  {}: {}", "Modified".green(), self.modified_files);
        }

        if self.unchanged_files > 0 {
            eprintln!("  {}: {}", "Unchanged".blue(), self.unchanged_files);
        }

        if self.error_files > 0 {
            eprintln!("  {}: {}", "Errors".red(), self.error_files);
        }

        if self.skipped_files > 0 {
            eprintln!("  {}: {}", "Skipped".yellow(), self.skipped_files);
        }
    }
}

/// Result of processing a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum FileResult {
    #[serde(rename = "modified")]
    Modified {
        file: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        transformations: Option<Vec<String>>,
    },
    #[serde(rename = "unchanged")]
    Unchanged {
        file: String,
    },
    #[serde(rename = "error")]
    Error {
        file: String,
        error: String,
    },
    #[serde(rename = "skipped")]
    Skipped {
        file: String,
        reason: String,
    },
}

impl FileResult {
    /// Get the file path from this result
    #[must_use]
    pub fn file_path(&self) -> &str {
        match self {
            Self::Modified { file, .. }
            | Self::Unchanged { file }
            | Self::Error { file, .. }
            | Self::Skipped { file, .. } => file,
        }
    }

    /// Check if this result represents a modified file
    #[must_use]
    pub const fn is_modified(&self) -> bool {
        matches!(self, Self::Modified { .. })
    }

    /// Check if this result represents an error
    #[must_use]
    #[allow(dead_code)] // Reason: Part of public API for programmatic use
    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error { .. })
    }
}

/// Container for all file results
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingResults {
    pub files: Vec<FileResult>,
    pub stats: StatsOutput,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsOutput {
    pub total: usize,
    pub modified: usize,
    pub unchanged: usize,
    pub errors: usize,
    pub skipped: usize,
}

impl From<&ProcessingStats> for StatsOutput {
    fn from(stats: &ProcessingStats) -> Self {
        Self {
            total: stats.total_files,
            modified: stats.modified_files,
            unchanged: stats.unchanged_files,
            errors: stats.error_files,
            skipped: stats.skipped_files,
        }
    }
}

/// Print a unified diff between original and processed content
pub fn print_diff(path: &Path, original: &str, processed: &str) {
    let diff = TextDiff::from_lines(original, processed);

    println!("{} {}", "diff".bold(), path.display().to_string().yellow());

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-".red(),
            ChangeTag::Insert => "+".green(),
            ChangeTag::Equal => " ".normal(),
        };
        print!("{sign}{change}");
    }
}

/// Print verbose log message
pub fn log_verbose(message: &str) {
    eprintln!("{} {}", "[verbose]".blue().bold(), message);
}

/// Print error message
pub fn log_error(message: &str) {
    eprintln!("{} {}", "[error]".red().bold(), message);
}

/// Print success message
pub fn log_success(message: &str) {
    eprintln!("{} {}", "[success]".green().bold(), message);
}

/// Print warning message
pub fn log_warning(message: &str) {
    eprintln!("{} {}", "[warning]".yellow().bold(), message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_tracking() {
        let mut stats = ProcessingStats::new();
        assert_eq!(stats.total_files, 0);

        stats.record_modified();
        assert_eq!(stats.total_files, 1);
        assert_eq!(stats.modified_files, 1);

        stats.record_unchanged();
        assert_eq!(stats.total_files, 2);
        assert_eq!(stats.unchanged_files, 1);

        stats.record_error();
        assert_eq!(stats.total_files, 3);
        assert_eq!(stats.error_files, 1);
    }

    #[test]
    fn test_file_result_accessors() {
        let result = FileResult::Modified {
            file: "test.md".to_string(),
            transformations: None,
        };

        assert_eq!(result.file_path(), "test.md");
        assert!(result.is_modified());
        assert!(!result.is_error());
    }

    #[test]
    fn test_json_serialization() {
        let result = FileResult::Modified {
            file: "test.md".to_string(),
            transformations: Some(vec!["box_expansion".to_string()]),
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("modified"));
        assert!(json.contains("test.md"));
    }
}
