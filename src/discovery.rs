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
}

impl FileDiscovery {
    /// Create a new file discovery with given extensions.
    ///
    /// Note: Simple directory filtering is used (skips hidden dirs, `node_modules`, `target`, etc.)
    /// For complex filtering needs, use external tools like `find` or `fd`.
    #[must_use]
    pub const fn new(extensions: Vec<String>) -> Self {
        Self { extensions }
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
        self.walk_directory_recursive(dir, results)
    }

    /// Internal recursive directory walker using `std::fs`.
    fn walk_directory_recursive(&self, dir: &Path, results: &mut Vec<PathBuf>) -> Result<()> {
        // Directories to skip (common build/cache directories)
        const SKIP_DIRS: &[&str] = &[
            "target",        // Rust build
            "node_modules",  // JavaScript
            "vendor",        // Go, PHP
            "dist",          // Build output
            "build",         // Build output
            ".git",          // Version control
            ".svn",          // Version control
            ".hg",           // Version control
        ];

        if !dir.is_dir() {
            return Ok(());
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            // Skip if it's a symlink (prevent loops)
            if entry.file_type()?.is_symlink() {
                continue;
            }

            // Get directory/file name
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };

            // Skip hidden files/directories (starting with .)
            if name.starts_with('.') {
                continue;
            }

            // Skip common build directories
            if SKIP_DIRS.contains(&name) {
                continue;
            }

            if path.is_dir() {
                // Recurse into subdirectories
                self.walk_directory_recursive(&path, results)?;
            } else if path.is_file() && self.matches_extension(&path) {
                results.push(path);
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
    fn test_parse_extensions_single() {
        let result = parse_extensions(".md").expect("Failed to parse");
        assert_eq!(result, vec![".md"]);
    }

    #[test]
    fn test_parse_extensions_normalizes_without_dot() {
        let result = parse_extensions("md").expect("Failed to parse");
        assert_eq!(result, vec![".md"]);
    }

    #[test]
    fn test_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let discovery = FileDiscovery::new(vec![".md".to_string()]);
        let results = discovery.discover(&[temp_dir.path().to_path_buf()]).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_single_markdown_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.md");
        fs::write(&file_path, "# Test").unwrap();

        let discovery = FileDiscovery::new(vec![".md".to_string()]);
        let results = discovery.discover(&[temp_dir.path().to_path_buf()]).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], file_path);
    }

    #[test]
    fn test_skips_hidden_directories() {
        let temp_dir = TempDir::new().unwrap();

        // Create hidden directory with .md file
        let hidden_dir = temp_dir.path().join(".git");
        fs::create_dir(&hidden_dir).unwrap();
        fs::write(hidden_dir.join("file.md"), "content").unwrap();

        let discovery = FileDiscovery::new(vec![".md".to_string()]);
        let results = discovery.discover(&[temp_dir.path().to_path_buf()]).unwrap();

        assert_eq!(results.len(), 0, "Should skip hidden directories");
    }

    #[test]
    fn test_skips_target_directory() {
        let temp_dir = TempDir::new().unwrap();

        // Create target directory with .md file
        let target_dir = temp_dir.path().join("target");
        fs::create_dir(&target_dir).unwrap();
        fs::write(target_dir.join("file.md"), "content").unwrap();

        let discovery = FileDiscovery::new(vec![".md".to_string()]);
        let results = discovery.discover(&[temp_dir.path().to_path_buf()]).unwrap();

        assert_eq!(results.len(), 0, "Should skip target directory");
    }

    #[test]
    fn test_skips_node_modules() {
        let temp_dir = TempDir::new().unwrap();

        // Create node_modules directory with .md file
        let nm_dir = temp_dir.path().join("node_modules");
        fs::create_dir(&nm_dir).unwrap();
        fs::write(nm_dir.join("readme.md"), "content").unwrap();

        let discovery = FileDiscovery::new(vec![".md".to_string()]);
        let results = discovery.discover(&[temp_dir.path().to_path_buf()]).unwrap();

        assert_eq!(results.len(), 0, "Should skip node_modules");
    }

    #[test]
    fn test_recursive_search() {
        let temp_dir = TempDir::new().unwrap();

        // Create nested structure
        let subdir = temp_dir.path().join("docs");
        fs::create_dir(&subdir).unwrap();
        let file_path = subdir.join("readme.md");
        fs::write(&file_path, "# README").unwrap();

        let discovery = FileDiscovery::new(vec![".md".to_string()]);
        let results = discovery.discover(&[temp_dir.path().to_path_buf()]).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], file_path);
    }

    #[test]
    fn test_filters_non_markdown() {
        let temp_dir = TempDir::new().unwrap();

        // Create mix of file types
        fs::write(temp_dir.path().join("test.md"), "markdown").unwrap();
        fs::write(temp_dir.path().join("test.txt"), "text").unwrap();
        fs::write(temp_dir.path().join("test.rs"), "rust").unwrap();

        let discovery = FileDiscovery::new(vec![".md".to_string()]);
        let results = discovery.discover(&[temp_dir.path().to_path_buf()]).unwrap();

        assert_eq!(results.len(), 1, "Should only find .md files");
    }

    #[test]
    fn test_multiple_extensions() {
        let temp_dir = TempDir::new().unwrap();

        fs::write(temp_dir.path().join("test.md"), "markdown").unwrap();
        fs::write(temp_dir.path().join("test.txt"), "text").unwrap();
        fs::write(temp_dir.path().join("test.rs"), "rust").unwrap();

        let discovery = FileDiscovery::new(vec![".md".to_string(), ".txt".to_string()]);
        let results = discovery.discover(&[temp_dir.path().to_path_buf()]).unwrap();

        assert_eq!(results.len(), 2, "Should find .md and .txt files");
    }
}
