//! Unit tests for file discovery module.

#[cfg(test)]
mod parse_extensions {
    use ascfix::discovery;

    #[test]
    fn test_parse_extensions_single() {
        let result = discovery::parse_extensions(".md").expect("Failed to parse");
        assert_eq!(result, vec![".md"]);
    }

    #[test]
    fn test_parse_extensions_multiple() {
        let result = discovery::parse_extensions(".md,.mdx,.txt").expect("Failed to parse");
        assert_eq!(result, vec![".md", ".mdx", ".txt"]);
    }

    #[test]
    fn test_parse_extensions_without_dots() {
        let result = discovery::parse_extensions("md,mdx").expect("Failed to parse");
        assert_eq!(result, vec![".md", ".mdx"]);
    }

    #[test]
    fn test_parse_extensions_mixed_dots() {
        let result = discovery::parse_extensions(".md,mdx,.txt").expect("Failed to parse");
        assert_eq!(result, vec![".md", ".mdx", ".txt"]);
    }

    #[test]
    fn test_parse_extensions_with_spaces() {
        let result = discovery::parse_extensions(" .md , mdx , .txt ").expect("Failed to parse");
        assert_eq!(result, vec![".md", ".mdx", ".txt"]);
    }

    #[test]
    fn test_parse_extensions_empty_string_error() {
        let result = discovery::parse_extensions("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_extensions_only_commas_error() {
        let result = discovery::parse_extensions(",,,");
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod single_file_discovery {
    use ascfix::discovery::FileDiscovery;
    use tempfile::TempDir;

    #[test]
    fn test_discover_single_md_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("test.md");
        std::fs::write(&file_path, "# Test").expect("Failed to write file");

        let discovery = FileDiscovery::new(vec![".md".to_string()], true);
        let results = discovery
            .discover(std::slice::from_ref(&file_path))
            .expect("Failed to discover");

        assert_eq!(results.len(), 1);
        assert_eq!(&results[0], &file_path);
    }

    #[test]
    fn test_discover_single_file_wrong_extension() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("test.txt");
        std::fs::write(&file_path, "# Test").expect("Failed to write file");

        let discovery = FileDiscovery::new(vec![".md".to_string()], true);
        let results = discovery
            .discover(&[file_path])
            .expect("Failed to discover");

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_discover_single_mdx_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("test.mdx");
        std::fs::write(&file_path, "# Test").expect("Failed to write file");

        let discovery = FileDiscovery::new(vec![".mdx".to_string()], true);
        let results = discovery
            .discover(std::slice::from_ref(&file_path))
            .expect("Failed to discover");

        assert_eq!(results.len(), 1);
        assert_eq!(&results[0], &file_path);
    }

    #[test]
    fn test_discover_multiple_extensions() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let md_path = temp_dir.path().join("test.md");
        let mdx_path = temp_dir.path().join("test.mdx");
        std::fs::write(&md_path, "# MD").expect("Failed to write file");
        std::fs::write(&mdx_path, "# MDX").expect("Failed to write file");

        let discovery = FileDiscovery::new(vec![".md".to_string(), ".mdx".to_string()], true);
        let results = discovery
            .discover(std::slice::from_ref(&md_path))
            .expect("Failed to discover");

        assert_eq!(results.len(), 1);
        assert_eq!(&results[0], &md_path);
    }
}

#[cfg(test)]
mod directory_discovery {
    use ascfix::discovery::FileDiscovery;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_discover_directory_with_md_files() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        fs::write(temp_dir.path().join("test1.md"), "# Test 1").expect("Failed to write");
        fs::write(temp_dir.path().join("test2.md"), "# Test 2").expect("Failed to write");
        fs::write(temp_dir.path().join("other.txt"), "# Other").expect("Failed to write");

        let discovery = FileDiscovery::new(vec![".md".to_string()], true);
        let results = discovery
            .discover(&[temp_dir.path().to_path_buf()])
            .expect("Failed to discover");

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_discover_nested_directories() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        fs::create_dir(temp_dir.path().join("nested")).expect("Failed to create dir");
        fs::write(temp_dir.path().join("test1.md"), "# Test 1").expect("Failed to write");
        fs::write(temp_dir.path().join("nested/test2.md"), "# Test 2").expect("Failed to write");

        let discovery = FileDiscovery::new(vec![".md".to_string()], true);
        let results = discovery
            .discover(&[temp_dir.path().to_path_buf()])
            .expect("Failed to discover");

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_discover_empty_directory() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let discovery = FileDiscovery::new(vec![".md".to_string()], true);
        let results = discovery
            .discover(&[temp_dir.path().to_path_buf()])
            .expect("Failed to discover");

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_discover_directory_no_matching_extensions() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        fs::write(temp_dir.path().join("test.txt"), "# Test").expect("Failed to write");
        fs::write(temp_dir.path().join("test.doc"), "# Test").expect("Failed to write");

        let discovery = FileDiscovery::new(vec![".md".to_string()], true);
        let results = discovery
            .discover(&[temp_dir.path().to_path_buf()])
            .expect("Failed to discover");

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_discover_mixed_files_and_directories() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("single.md");
        fs::write(&file_path, "# Single").expect("Failed to write");

        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).expect("Failed to create dir");
        fs::write(subdir.join("nested.md"), "# Nested").expect("Failed to write");

        let discovery = FileDiscovery::new(vec![".md".to_string()], true);
        let results = discovery
            .discover(&[file_path, subdir])
            .expect("Failed to discover");

        assert_eq!(results.len(), 2);
    }
}
