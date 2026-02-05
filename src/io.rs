//! File I/O operations for reading and writing Markdown files safely.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Read a Markdown file's contents.
///
/// # Errors
///
/// Returns an error if the file cannot be read.
#[allow(dead_code)]  // Reason: Used by main processing pipeline (Cycle 3)
pub fn read_markdown(path: &Path) -> Result<String> {
    fs::read_to_string(path).context(format!("Failed to read {}", path.display()))
}

/// Write content to a file.
///
/// # Errors
///
/// Returns an error if the file cannot be written.
#[allow(dead_code)]  // Reason: Used by main processing pipeline (Cycle 3)
pub fn write_markdown(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content).context(format!("Failed to write {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_read_markdown() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.md");
        let content = "# Test\n\nHello, world!";
        fs::write(&file_path, content)?;

        let read_content = read_markdown(&file_path)?;
        assert_eq!(read_content, content);
        Ok(())
    }

    #[test]
    fn test_write_markdown() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.md");
        let content = "# Test\n\nHello, world!";

        write_markdown(&file_path, content)?;

        let read_content = fs::read_to_string(&file_path)?;
        assert_eq!(read_content, content);
        Ok(())
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_markdown(Path::new("/nonexistent/file.md"));
        assert!(result.is_err());
    }

    #[test]
    fn test_preserves_exact_content() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.md");
        let content = "Line 1\nLine 2\n  Indented\n\n\nMultiple blank lines";
        fs::write(&file_path, content)?;

        let read_content = read_markdown(&file_path)?;
        assert_eq!(read_content, content);
        Ok(())
    }

    #[test]
    fn test_preserves_unicode() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.md");
        let content = "Hello 世界 🎉 Ñoño";
        fs::write(&file_path, content)?;

        let read_content = read_markdown(&file_path)?;
        assert_eq!(read_content, content);
        Ok(())
    }
}
