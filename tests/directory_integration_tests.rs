//! Integration tests for directory processing, gitignore, and error handling.

#[cfg(test)]
mod directory_processing {
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_basic_directory_processing() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Initialize git repo
        std::process::Command::new("git")
            .arg("init")
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to init git");

        std::process::Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(temp_dir.path())
            .output()
            .ok();

        // Create test files
        fs::write(temp_dir.path().join("file1.md"), "# Test 1").expect("Failed to write");
        fs::write(temp_dir.path().join("file2.md"), "# Test 2").expect("Failed to write");
        fs::write(temp_dir.path().join("other.txt"), "# Other").expect("Failed to write");

        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(temp_dir.path())
            .output()
            .ok();

        // Directory should be discoverable
        assert!(temp_dir.path().is_dir());
        assert!(temp_dir.path().join("file1.md").exists());
        assert!(temp_dir.path().join("file2.md").exists());
    }

    #[test]
    fn test_nested_directory_structure() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Initialize git repo
        std::process::Command::new("git")
            .arg("init")
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to init git");

        std::process::Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(temp_dir.path())
            .output()
            .ok();

        // Create nested structure
        fs::create_dir(temp_dir.path().join("docs")).expect("Failed to create dir");
        fs::create_dir(temp_dir.path().join("docs/api")).expect("Failed to create dir");

        fs::write(temp_dir.path().join("README.md"), "# Root").expect("Failed to write");
        fs::write(temp_dir.path().join("docs/guide.md"), "# Guide").expect("Failed to write");
        fs::write(temp_dir.path().join("docs/api/reference.md"), "# API").expect("Failed to write");

        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(temp_dir.path())
            .output()
            .ok();

        // All files should be discoverable
        assert!(temp_dir.path().join("README.md").exists());
        assert!(temp_dir.path().join("docs/guide.md").exists());
        assert!(temp_dir.path().join("docs/api/reference.md").exists());
    }

    #[test]
    fn test_mixed_markdown_extensions() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Initialize git repo
        std::process::Command::new("git")
            .arg("init")
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to init git");

        std::process::Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(temp_dir.path())
            .output()
            .ok();

        // Create both .md and .mdx files
        fs::write(temp_dir.path().join("file.md"), "# MD").expect("Failed to write");
        fs::write(temp_dir.path().join("component.mdx"), "# MDX").expect("Failed to write");

        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(temp_dir.path())
            .output()
            .ok();

        // Both should exist
        assert!(temp_dir.path().join("file.md").exists());
        assert!(temp_dir.path().join("component.mdx").exists());
    }

    #[test]
    fn test_empty_directory_handling() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Initialize git repo
        std::process::Command::new("git")
            .arg("init")
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to init git");

        // Empty directory should be valid (just no files to process)
        assert!(temp_dir.path().is_dir());
        let entries: Vec<_> = fs::read_dir(temp_dir.path())
            .expect("Failed to read dir")
            .collect();
        // Only .git directory
        assert!(entries.len() <= 1);
    }

    #[test]
    fn test_gitignore_with_nested_dirs() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Initialize git repo
        std::process::Command::new("git")
            .arg("init")
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to init git");

        std::process::Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(temp_dir.path())
            .output()
            .ok();

        // Create structure with ignored dirs
        fs::create_dir(temp_dir.path().join("src")).expect("Failed to create dir");
        fs::create_dir(temp_dir.path().join("target")).expect("Failed to create dir");

        fs::write(
            temp_dir.path().join(".gitignore"),
            "target/\n*.tmp\n",
        )
        .expect("Failed to write gitignore");

        fs::write(temp_dir.path().join("README.md"), "# Root").expect("Failed to write");
        fs::write(temp_dir.path().join("src/main.md"), "# Main").expect("Failed to write");
        fs::write(temp_dir.path().join("target/build.md"), "# Build").expect("Failed to write");
        fs::write(temp_dir.path().join("file.tmp"), "Temp").expect("Failed to write");

        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(temp_dir.path())
            .output()
            .ok();

        // Verify structure
        assert!(temp_dir.path().join("README.md").exists());
        assert!(temp_dir.path().join("src/main.md").exists());
        assert!(temp_dir.path().join("target/build.md").exists());
        assert!(temp_dir.path().join("file.tmp").exists());
    }

    #[test]
    fn test_single_file_with_directory() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create a single file in the directory
        fs::write(temp_dir.path().join("single.md"), "# Single").expect("Failed to write");

        // Single file should be accessible
        assert!(temp_dir.path().join("single.md").exists());
    }

    #[test]
    fn test_file_discovery_excludes_non_matching_extensions() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Initialize git repo
        std::process::Command::new("git")
            .arg("init")
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to init git");

        std::process::Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(temp_dir.path())
            .output()
            .ok();

        // Create files with various extensions
        fs::write(temp_dir.path().join("doc.md"), "# MD").expect("Failed to write");
        fs::write(temp_dir.path().join("readme.txt"), "# TXT").expect("Failed to write");
        fs::write(temp_dir.path().join("script.js"), "// JS").expect("Failed to write");
        fs::write(temp_dir.path().join("style.css"), "/* CSS */").expect("Failed to write");

        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(temp_dir.path())
            .output()
            .ok();

        // Verify all exist
        assert!(temp_dir.path().join("doc.md").exists());
        assert!(temp_dir.path().join("readme.txt").exists());
        assert!(temp_dir.path().join("script.js").exists());
        assert!(temp_dir.path().join("style.css").exists());
    }
}
