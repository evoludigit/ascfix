//! Tests for .gitignore support in file discovery.

#[cfg(test)]
mod gitignore_support {
    use ascfix::discovery::FileDiscovery;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_gitignore_respected_by_default() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Initialize a git repo for gitignore to work
        std::process::Command::new("git")
            .arg("init")
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to init git");

        // Create a .gitignore that excludes .mdx files
        fs::write(temp_dir.path().join(".gitignore"), "*.mdx").expect("Failed to write gitignore");

        // Create both .md and .mdx files
        fs::write(temp_dir.path().join("included.md"), "# Included").expect("Failed to write");
        fs::write(temp_dir.path().join("excluded.mdx"), "# Excluded").expect("Failed to write");

        // Configure git to avoid warnings in tests
        std::process::Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to config git");
        std::process::Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to config git");

        // Commit files to make gitignore effective
        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to git add");

        // Default discovery should respect gitignore
        let discovery = FileDiscovery::new(vec![".md".to_string(), ".mdx".to_string()], true);
        let results = discovery
            .discover(&[temp_dir.path().to_path_buf()])
            .expect("Failed to discover");

        // Should only find the .md file
        assert_eq!(results.len(), 1);
        assert!(results[0].ends_with("included.md"));
    }

    #[test]
    fn test_no_gitignore_flag_includes_ignored_files() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create a .gitignore that excludes .mdx files
        fs::write(temp_dir.path().join(".gitignore"), "*.mdx").expect("Failed to write gitignore");

        // Create both .md and .mdx files
        fs::write(temp_dir.path().join("included.md"), "# Included").expect("Failed to write");
        fs::write(temp_dir.path().join("excluded.mdx"), "# Excluded").expect("Failed to write");

        // Discovery with respect_gitignore=false should include all files
        let discovery = FileDiscovery::new(vec![".md".to_string(), ".mdx".to_string()], false);
        let results = discovery
            .discover(&[temp_dir.path().to_path_buf()])
            .expect("Failed to discover");

        // Should find both files
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_nested_gitignore() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Initialize a git repo
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
        std::process::Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(temp_dir.path())
            .output()
            .ok();

        // Create nested directories
        fs::create_dir(temp_dir.path().join("subdir")).expect("Failed to create dir");

        // Create root .gitignore that excludes .tmp files
        fs::write(temp_dir.path().join(".gitignore"), "*.tmp").expect("Failed to write gitignore");

        // Create .md files at root and in subdirectory
        fs::write(temp_dir.path().join("root.md"), "# Root").expect("Failed to write");
        fs::write(temp_dir.path().join("subdir/nested.md"), "# Nested").expect("Failed to write");

        // Create a .tmp file that should be ignored
        fs::write(temp_dir.path().join("subdir/ignored.tmp"), "# Ignored")
            .expect("Failed to write");

        // Stage files
        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to git add");

        // With gitignore enabled
        let discovery = FileDiscovery::new(vec![".md".to_string(), ".tmp".to_string()], true);
        let results = discovery
            .discover(&[temp_dir.path().to_path_buf()])
            .expect("Failed to discover");

        // Should find both .md files but not the .tmp file
        assert_eq!(results.len(), 2);
        let md_count = results
            .iter()
            .filter(|p| p.extension().is_some_and(|e| e == "md"))
            .count();
        assert_eq!(md_count, 2);
    }

    #[test]
    fn test_single_file_ignores_gitignore() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create a .gitignore that excludes .mdx files
        fs::write(temp_dir.path().join(".gitignore"), "*.mdx").expect("Failed to write gitignore");

        // Create an .mdx file
        let mdx_path = temp_dir.path().join("file.mdx");
        fs::write(&mdx_path, "# MDX").expect("Failed to write");

        // When processing a single file directly, it should be included even if gitignore excludes it
        let discovery = FileDiscovery::new(vec![".mdx".to_string()], true);
        let results = discovery
            .discover(std::slice::from_ref(&mdx_path))
            .expect("Failed to discover");

        assert_eq!(results.len(), 1);
        assert_eq!(&results[0], &mdx_path);
    }

    #[test]
    fn test_complex_gitignore_patterns() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Initialize a git repo
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
        std::process::Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(temp_dir.path())
            .output()
            .ok();

        // Create a more complex .gitignore
        fs::write(
            temp_dir.path().join(".gitignore"),
            "*.tmp\nbuild/\n.DS_Store\n",
        )
        .expect("Failed to write gitignore");

        // Create various files
        fs::write(temp_dir.path().join("doc.md"), "# Doc").expect("Failed to write");
        fs::write(temp_dir.path().join("temp.tmp"), "Temp").expect("Failed to write");

        fs::create_dir(temp_dir.path().join("build")).expect("Failed to create dir");
        fs::write(temp_dir.path().join("build/artifact.md"), "# Artifact")
            .expect("Failed to write");

        // Stage files
        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to git add");

        // With gitignore enabled
        let discovery = FileDiscovery::new(vec![".md".to_string(), ".tmp".to_string()], true);
        let results = discovery
            .discover(&[temp_dir.path().to_path_buf()])
            .expect("Failed to discover");

        // Should only find doc.md (build/ and *.tmp are ignored)
        assert_eq!(results.len(), 1);
        assert!(results[0].ends_with("doc.md"));
    }
}
