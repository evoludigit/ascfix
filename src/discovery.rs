//! File discovery and filtering for ascfix.
//!
//! This module provides functionality to discover and filter files based on:
//! - File extensions
//! - Directory traversal
//! - `.gitignore` respecting (optional)

use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

/// Parse a comma-separated string of extensions.
///
/// Normalizes extensions to include a leading dot.
/// Handles both ".md" and "md" format.
///
/// # Errors
///
/// Returns an error if the string is empty or contains only whitespace/commas.
#[allow(dead_code)] // Reason: Part of public API for potential use by tests/library consumers
pub fn parse_extensions(s: &str) -> Result<Vec<String>> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("extensions string is empty"));
    }

    let extensions: Vec<String> = trimmed
        .split(',')
        .map(str::trim)
        .filter(|ext| !ext.is_empty())
        .map(|ext| {
            if ext.starts_with('.') {
                ext.to_string()
            } else {
                format!(".{ext}")
            }
        })
        .collect();

    if extensions.is_empty() {
        return Err(anyhow!("no valid extensions found"));
    }

    Ok(extensions)
}

/// File discovery configuration and operations.
pub struct FileDiscovery {
    extensions: Vec<String>,
    respect_gitignore: bool,
}

impl FileDiscovery {
    /// Create a new file discovery with given extensions and gitignore setting.
    #[must_use]
    pub const fn new(extensions: Vec<String>, respect_gitignore: bool) -> Self {
        Self {
            extensions,
            respect_gitignore,
        }
    }

    /// Discover files matching the configured criteria.
    ///
    /// For each path in `paths`:
    /// - If it's a file: include if extension matches
    /// - If it's a directory: recursively find all matching files
    ///
    /// # Errors
    ///
    /// Returns an error if a path cannot be read or doesn't exist.
    pub fn discover(&self, paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let mut results = Vec::new();

        for path in paths {
            if path.is_file() {
                // Single file: check extension
                if self.matches_extension(path) {
                    results.push(path.clone());
                }
            } else if path.is_dir() {
                // Directory: walk recursively
                self.walk_directory(path, &mut results)?;
            } else {
                return Err(anyhow!(
                    "path does not exist or is not accessible: {}",
                    path.display()
                ));
            }
        }

        Ok(results)
    }

    /// Check if a file's extension matches configured extensions.
    fn matches_extension(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| {
                let ext_with_dot = if ext.starts_with('.') {
                    ext.to_string()
                } else {
                    format!(".{ext}")
                };
                self.extensions.contains(&ext_with_dot)
            })
    }

    /// Recursively walk a directory and collect matching files.
    fn walk_directory(&self, dir: &Path, results: &mut Vec<PathBuf>) -> Result<()> {
        let mut builder = ignore::WalkBuilder::new(dir);
        // standard_filters enables: hidden file filtering, .gitignore parsing, etc.
        builder.standard_filters(self.respect_gitignore);
        let walker = builder.build();

        for entry in walker.flatten() {
            let path = entry.path();
            if path.is_file() && self.matches_extension(path) {
                results.push(path.to_path_buf());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_extensions_single() {
        let result = parse_extensions(".md").expect("Failed to parse");
        assert_eq!(result, vec![".md"]);
    }

    #[test]
    fn test_parse_extensions_normalizes_without_dot() {
        let result = parse_extensions("md").expect("Failed to parse");
        assert_eq!(result, vec![".md"]);
    }
}
