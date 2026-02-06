//! Command-line argument parsing and configuration.

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

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
}
