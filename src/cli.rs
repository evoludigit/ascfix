//! Command-line argument parsing and configuration.

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

const KB: u64 = 1024;
const MB: u64 = 1024 * KB;
const GB: u64 = 1024 * MB;

fn parse_size(s: &str) -> Result<u64, String> {
    let original = s.trim();
    if original.is_empty() {
        return Err("size cannot be empty".into());
    }

    let s = original.to_uppercase();

    let (num_str, multiplier) = s.strip_suffix("GB").map_or_else(
        || {
            s.strip_suffix("MB").map_or_else(
                || {
                    s.strip_suffix("KB").map_or_else(
                        || {
                            s.strip_suffix('B')
                                .map_or_else(|| (s.as_str(), 1), |n| (n, 1))
                        },
                        |n| (n, KB),
                    )
                },
                |n| (n, MB),
            )
        },
        |n| (n, GB),
    );

    let value: u64 = num_str
        .trim()
        .parse()
        .map_err(|_| format!("invalid size value: '{original}'"))?;

    value
        .checked_mul(multiplier)
        .ok_or_else(|| format!("size too large: '{original}'"))
}

#[derive(Parser, Debug, Clone)]
#[command(name = "ascfix")]
#[command(about = "Repair ASCII diagrams in Markdown and text files")]
#[allow(clippy::struct_excessive_bools)] // CLI flags are naturally boolean
pub struct Args {
    #[arg(required = true)]
    pub paths: Vec<PathBuf>,

    #[arg(long, value_enum, default_value = "safe")]
    pub mode: Mode,

    #[arg(short, long)]
    pub in_place: bool,

    #[arg(long, conflicts_with = "in_place")]
    pub check: bool,

    #[arg(long, value_parser = parse_size)]
    pub max_size: Option<u64>,

    #[arg(long)]
    pub fences: bool,

    #[arg(long)]
    pub all: bool,

    #[arg(
        long,
        short = 'e',
        value_delimiter = ',',
        default_value = ".md,.mdx,.txt"
    )]
    pub ext: Vec<String>,

    #[arg(long)]
    pub no_gitignore: bool,

    #[arg(long)]
    pub summary: bool,

    #[arg(long)]
    pub list_files: bool,

    #[arg(short, long)]
    pub verbose: bool,

    #[arg(long)]
    pub json: bool,

    #[arg(long)]
    pub diff: bool,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    Safe,
    Diagram,
    Check,
}

#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)] // Configuration flags are naturally boolean
#[allow(dead_code)] // Used in main and tests, not in lib
pub struct Config {
    pub paths: Vec<PathBuf>,
    pub mode: Mode,
    pub in_place: bool,
    pub check: bool,
    pub max_size: Option<u64>,
    pub fences: bool,
    pub extensions: Vec<String>,
    pub no_gitignore: bool,
}

/// Normalize extension list: deduplicate, trim, lowercase, ensure leading dot.
#[allow(dead_code)] // Used in Config From impl, which is used in main and tests
fn normalize_exts(exts: &[String]) -> Vec<String> {
    let mut normalized: Vec<String> = exts
        .iter()
        .map(|s| {
            let trimmed = s.trim();
            let mut ext = if trimmed.starts_with('.') {
                trimmed.to_lowercase()
            } else {
                format!(".{}", trimmed.to_lowercase())
            };
            // Remove trailing dots if any
            while ext.ends_with('.') && ext.len() > 1 {
                ext.pop();
            }
            ext
        })
        .filter(|s| !s.is_empty())
        .collect();

    // Remove duplicates while preserving order
    let mut seen = std::collections::HashSet::new();
    normalized.retain(|e| seen.insert(e.clone()));

    normalized
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        let mode = if args.all { Mode::Diagram } else { args.mode };
        let fences = if args.all { true } else { args.fences };

        Self {
            paths: args.paths,
            mode,
            in_place: args.in_place,
            check: args.check,
            max_size: args.max_size,
            fences,
            extensions: normalize_exts(&args.ext),
            no_gitignore: args.no_gitignore,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::error::ErrorKind;
    use std::path::PathBuf;

    // ------------------------------------------------------------
    // Size parsing tests
    // ------------------------------------------------------------

    #[test]
    fn parse_size_bytes() {
        assert_eq!(parse_size("1024").unwrap(), 1024);
        assert_eq!(parse_size("10B").unwrap(), 10);
    }

    #[test]
    fn parse_size_kb_mb_gb() {
        assert_eq!(parse_size("1KB").unwrap(), 1024);
        assert_eq!(parse_size("2MB").unwrap(), 2 * 1024 * 1024);
        assert_eq!(parse_size("3GB").unwrap(), 3 * 1024 * 1024 * 1024);
    }

    #[test]
    fn parse_size_lowercase_and_whitespace() {
        assert_eq!(parse_size(" 2mb ").unwrap(), 2 * 1024 * 1024);
    }

    #[test]
    fn parse_size_invalid() {
        assert!(parse_size("").is_err());
        assert!(parse_size("abc").is_err());
        assert!(parse_size("10XB").is_err());
    }

    #[test]
    fn parse_size_overflow() {
        // Large enough to overflow when multiplied
        let result = parse_size("18446744073709551615GB");
        assert!(result.is_err());
    }

    // ------------------------------------------------------------
    // Basic parsing tests
    // ------------------------------------------------------------

    #[test]
    fn parse_requires_path() {
        let result = Args::try_parse_from(["ascfix"]);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().kind(),
            ErrorKind::MissingRequiredArgument
        );
    }

    #[test]
    fn parse_single_file_defaults() {
        let args = Args::try_parse_from(["ascfix", "test.md"]).unwrap();

        assert_eq!(args.paths, vec![PathBuf::from("test.md")]);
        assert_eq!(args.mode, Mode::Safe);
        assert!(!args.in_place);
        assert!(!args.check);
        assert!(!args.fences);
        assert!(!args.all);
    }

    #[test]
    fn parse_mode_diagram() {
        let args = Args::try_parse_from(["ascfix", "--mode", "diagram", "file.md"]).unwrap();

        assert_eq!(args.mode, Mode::Diagram);
    }

    #[test]
    fn parse_in_place() {
        let args = Args::try_parse_from(["ascfix", "--in-place", "file.md"]).unwrap();

        assert!(args.in_place);
    }

    #[test]
    fn parse_check_flag() {
        let args = Args::try_parse_from(["ascfix", "--check", "file.md"]).unwrap();

        assert!(args.check);
    }

    #[test]
    fn parse_max_size() {
        let args = Args::try_parse_from(["ascfix", "--max-size", "5MB", "file.md"]).unwrap();

        assert_eq!(args.max_size, Some(5 * 1024 * 1024));
    }

    // ------------------------------------------------------------
    // Conflict handling
    // ------------------------------------------------------------

    #[test]
    fn check_conflicts_with_in_place() {
        let result = Args::try_parse_from(["ascfix", "--check", "--in-place", "file.md"]);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    // ------------------------------------------------------------
    // Extension normalization
    // ------------------------------------------------------------

    #[test]
    fn default_extensions_include_txt() {
        let args = Args::try_parse_from(["ascfix", "file.md"]).unwrap();
        let config: Config = args.into();

        assert!(config.extensions.contains(&".md".to_string()));
        assert!(config.extensions.contains(&".mdx".to_string()));
        assert!(config.extensions.contains(&".txt".to_string()));
    }

    #[test]
    fn extension_normalization_trim_lowercase_dot() {
        let args =
            Args::try_parse_from(["ascfix", "--ext", "md,.MD, txt ,.txt", "file.md"]).unwrap();

        let config: Config = args.into();

        assert_eq!(
            config.extensions,
            vec![".md".to_string(), ".txt".to_string()]
        );
    }

    #[test]
    fn extension_order_preserved() {
        let args = Args::try_parse_from(["ascfix", "--ext", "txt,md", "file.md"]).unwrap();

        let config: Config = args.into();

        assert_eq!(
            config.extensions,
            vec![".txt".to_string(), ".md".to_string()]
        );
    }

    // ------------------------------------------------------------
    // --all semantic expansion
    // ------------------------------------------------------------

    #[test]
    fn all_expands_to_diagram_and_fences() {
        let args = Args::try_parse_from(["ascfix", "--all", "file.md"]).unwrap();

        let config: Config = args.into();

        assert_eq!(config.mode, Mode::Diagram);
        assert!(config.fences);
    }

    #[test]
    fn all_overrides_safe_mode() {
        let args = Args::try_parse_from(["ascfix", "--mode", "safe", "--all", "file.md"]).unwrap();

        let config: Config = args.into();

        assert_eq!(config.mode, Mode::Diagram);
        assert!(config.fences);
    }

    // ------------------------------------------------------------
    // Combined realistic scenario
    // ------------------------------------------------------------

    #[test]
    fn complex_configuration() {
        let args = Args::try_parse_from([
            "ascfix",
            "--mode",
            "diagram",
            "--max-size",
            "10MB",
            "--ext",
            "md,txt",
            "--no-gitignore",
            "docs/",
        ])
        .unwrap();

        let config: Config = args.into();

        assert_eq!(config.mode, Mode::Diagram);
        assert_eq!(config.max_size, Some(10 * 1024 * 1024));
        assert_eq!(
            config.extensions,
            vec![".md".to_string(), ".txt".to_string()]
        );
        assert!(config.no_gitignore);
        assert_eq!(config.paths, vec![PathBuf::from("docs/")]);
    }
}
