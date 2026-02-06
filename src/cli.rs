//! Command-line argument parsing and configuration.

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Parse a size string with optional suffix (B, KB, MB, GB).
/// Examples: "100", "50MB", "1GB", "500KB"
#[allow(dead_code)] // Used by clap's value_parser
fn parse_size(s: &str) -> Result<u64, String> {
    let s = s.trim().to_uppercase();

    let (num_str, multiplier) = match s.as_str() {
        _ if s.ends_with("GB") => (s.strip_suffix("GB").unwrap(), 1024 * 1024 * 1024),
        _ if s.ends_with("MB") => (s.strip_suffix("MB").unwrap(), 1024 * 1024),
        _ if s.ends_with("KB") => (s.strip_suffix("KB").unwrap(), 1024),
        _ if s.ends_with('B') => (s.strip_suffix('B').unwrap(), 1),
        _ => (s.as_str(), 1),
    };

    num_str
        .trim()
        .parse::<u64>()
        .map(|n| n * multiplier)
        .map_err(|_| format!("invalid size: {s}"))
}

#[derive(Parser, Debug, Clone)]
#[command(name = "ascfix")]
#[command(about = "Repair ASCII diagrams in Markdown files", long_about = None)]
pub struct Args {
    /// File or glob pattern to process
    pub files: Vec<PathBuf>,

    /// Processing mode
    #[arg(long, value_enum, default_value = "safe")]
    pub mode: Mode,

    /// Modify files in place instead of printing to stdout
    #[arg(short, long)]
    pub in_place: bool,

    /// Check if files need fixing (exit 1 if yes, 0 if no)
    #[arg(long)]
    pub check: bool,

    /// Maximum file size to process (e.g., "100MB", "1GB", default: unlimited)
    #[arg(long, value_parser = parse_size)]
    pub max_size: Option<u64>,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Only fix Markdown tables (safe mode, default)
    #[value(name = "safe")]
    Safe,
    /// Enable box and arrow normalization
    #[value(name = "diagram")]
    Diagram,
    /// Validate without checking content
    #[value(name = "check")]
    Check,
}

impl Args {
    /// Parse command-line arguments.
    #[must_use]
    pub fn parse_args() -> Self {
        Parser::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parse_single_file() {
        let args = Args::try_parse_from(["ascfix", "test.md"]);
        assert!(args.is_ok());
        let parsed = args.unwrap();
        assert_eq!(parsed.files.len(), 1);
        assert_eq!(parsed.files[0], PathBuf::from("test.md"));
        assert_eq!(parsed.mode, Mode::Safe);
        assert!(!parsed.in_place);
        assert!(!parsed.check);
    }

    #[test]
    fn test_args_parse_mode_diagram() {
        let args = Args::try_parse_from(["ascfix", "--mode", "diagram", "test.md"]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().mode, Mode::Diagram);
    }

    #[test]
    fn test_args_parse_in_place() {
        let args = Args::try_parse_from(["ascfix", "--in-place", "test.md"]);
        assert!(args.is_ok());
        assert!(args.unwrap().in_place);
    }

    #[test]
    fn test_args_parse_check() {
        let args = Args::try_parse_from(["ascfix", "--check", "test.md"]);
        assert!(args.is_ok());
        assert!(args.unwrap().check);
    }

    #[test]
    fn test_args_parse_max_size_mb() {
        let args = Args::try_parse_from(["ascfix", "--max-size", "100MB", "test.md"]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().max_size, Some(100 * 1024 * 1024));
    }

    #[test]
    fn test_args_parse_max_size_gb() {
        let args = Args::try_parse_from(["ascfix", "--max-size", "2GB", "test.md"]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().max_size, Some(2 * 1024 * 1024 * 1024));
    }

    #[test]
    fn test_args_parse_max_size_kb() {
        let args = Args::try_parse_from(["ascfix", "--max-size", "500KB", "test.md"]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().max_size, Some(500 * 1024));
    }

    #[test]
    fn test_args_parse_max_size_bytes() {
        let args = Args::try_parse_from(["ascfix", "--max-size", "1024", "test.md"]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().max_size, Some(1024));
    }

    #[test]
    fn test_args_parse_max_size_invalid() {
        let args = Args::try_parse_from(["ascfix", "--max-size", "invalid", "test.md"]);
        assert!(args.is_err());
    }

    #[test]
    fn test_args_parse_no_max_size() {
        let args = Args::try_parse_from(["ascfix", "test.md"]);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().max_size, None);
    }
}
