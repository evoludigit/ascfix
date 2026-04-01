//! Command-line argument parsing using lexopt.

use anyhow::{bail, Context, Result};
use lexopt::prelude::*;
use std::path::PathBuf;

const KB: u64 = 1024;
const MB: u64 = 1024 * KB;
const GB: u64 = 1024 * MB;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const ABOUT: &str = "Repair ASCII diagrams in Markdown and text files";

fn parse_size(s: &str) -> Result<u64> {
    let original = s.trim();
    if original.is_empty() {
        bail!("size cannot be empty");
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
        .with_context(|| format!("invalid size value: '{original}'"))?;

    value
        .checked_mul(multiplier)
        .with_context(|| format!("size too large: '{original}'"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    Safe,
    Diagram,
    Check,
}

impl std::str::FromStr for Mode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "safe" => Ok(Self::Safe),
            "diagram" => Ok(Self::Diagram),
            "check" => Ok(Self::Check),
            _ => bail!("invalid mode: '{s}' (expected: safe, diagram, or check)"),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)] // CLI flags are naturally boolean
pub struct Args {
    pub paths: Vec<PathBuf>,
    pub mode: Mode,
    pub in_place: bool,
    pub check: bool,
    pub max_size: Option<u64>,
    pub fences: bool,
    pub all: bool,
    pub ext: Vec<String>,
    pub summary: bool,
    pub list_files: bool,
    pub verbose: bool,
    pub json: bool,
    pub diff: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            paths: Vec::new(),
            mode: Mode::Diagram,
            in_place: false,
            check: false,
            max_size: None,
            fences: false,
            all: false,
            ext: vec![
                String::from(".md"),
                String::from(".mdx"),
                String::from(".txt"),
            ],
            summary: false,
            list_files: false,
            verbose: false,
            json: false,
            diff: false,
        }
    }
}

fn print_help() {
    println!("{ABOUT}");
    println!();
    println!("USAGE:");
    println!("    ascfix [OPTIONS] <PATHS>...");
    println!();
    println!("ARGS:");
    println!("    <PATHS>...                 Files or directories to process");
    println!();
    println!("MODES:");
    println!("    --mode <MODE>              Processing mode [default: diagram]");
    println!(
        "        diagram                  Detect and repair ASCII boxes, arrows, and alignment"
    );
    println!("        safe                     Only normalize Markdown tables and lists (no diagram changes)");
    println!();
    println!("OPTIONS:");
    println!("    --check                    Exit with code 1 if any file would change (for CI; conflicts with -i)");
    println!(
        "    --fences                   Repair mismatched code fence markers (length, unclosed)"
    );
    println!("    --all                      Enable all processing: diagram mode + fence repair");
    println!();
    println!("    -i, --in-place             Modify files in place (default: print to stdout)");
    println!("        --max-size <SIZE>      Skip files larger than SIZE (e.g., 1MB, 500KB)");
    println!("    -e, --ext <EXT>            File extensions to process [default: .md,.mdx,.txt]");
    println!("        --diff                 Show unified diff of changes");
    println!("        --summary              Show per-file change summary");
    println!("        --list-files           List files that would be processed (dry run)");
    println!("    -v, --verbose              Verbose output");
    println!("        --json                 Output results as JSON");
    println!("    -h, --help                 Print this help");
    println!("    -V, --version              Print version");
    println!();
    println!("EXAMPLES:");
    println!("    ascfix README.md                       Fix diagrams, print to stdout");
    println!("    ascfix -i docs/                        Fix all Markdown files in docs/ in place");
    println!(
        "    ascfix --check .                       CI check: exit 1 if anything needs fixing"
    );
    println!("    ascfix --all -i README.md              Fix diagrams + fences in place");
    println!("    ascfix --mode safe -i docs/            Only fix tables and lists");
}

fn print_version() {
    println!("ascfix {VERSION}");
}

impl Args {
    /// Parse command-line arguments from the environment.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No paths are provided
    /// - Invalid flag combinations are used (e.g., `--check` with `--in-place`)
    /// - Invalid values are provided for options
    pub fn parse() -> Result<Self> {
        Self::parse_from(std::env::args_os())
    }

    /// Parse command-line arguments from an iterator.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No paths are provided
    /// - Invalid flag combinations are used (e.g., `--check` with `--in-place`)
    /// - Invalid values are provided for options
    pub fn parse_from<I, T>(args: I) -> Result<Self>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString>,
    {
        let mut result = Self::default();
        let mut parser = lexopt::Parser::from_iter(args);

        while let Some(arg) = parser.next()? {
            match arg {
                lexopt::Arg::Long("mode") => {
                    let value: String = parser.value()?.parse()?;
                    result.mode = value.parse()?;
                }
                lexopt::Arg::Short('i') | lexopt::Arg::Long("in-place") => {
                    result.in_place = true;
                }
                lexopt::Arg::Long("check") => {
                    result.check = true;
                }
                lexopt::Arg::Long("max-size") => {
                    let value: String = parser.value()?.parse()?;
                    result.max_size = Some(parse_size(&value)?);
                }
                lexopt::Arg::Long("fences") => {
                    result.fences = true;
                }
                lexopt::Arg::Long("all") => {
                    result.all = true;
                }
                lexopt::Arg::Short('e') | lexopt::Arg::Long("ext") => {
                    let value: String = parser.value()?.parse()?;
                    // Replace default extensions on first use
                    if result.ext == Self::default().ext {
                        result.ext.clear();
                    }
                    // Split by comma and add all extensions
                    for ext in value.split(',') {
                        result.ext.push(ext.trim().to_string());
                    }
                }
                lexopt::Arg::Long("summary") => {
                    result.summary = true;
                }
                lexopt::Arg::Long("list-files") => {
                    result.list_files = true;
                }
                lexopt::Arg::Short('v') | lexopt::Arg::Long("verbose") => {
                    result.verbose = true;
                }
                lexopt::Arg::Long("json") => {
                    result.json = true;
                }
                lexopt::Arg::Long("diff") => {
                    result.diff = true;
                }
                lexopt::Arg::Short('h') | lexopt::Arg::Long("help") => {
                    print_help();
                    std::process::exit(0);
                }
                lexopt::Arg::Short('V') | lexopt::Arg::Long("version") => {
                    print_version();
                    std::process::exit(0);
                }
                lexopt::Arg::Value(val) => {
                    result.paths.push(val.into());
                }
                _ => bail!("{}", arg.unexpected()),
            }
        }

        // Validation
        if result.paths.is_empty() {
            bail!("error: the following required arguments were not provided:\n  <PATHS>...\n\nUsage: ascfix [OPTIONS] <PATHS>...\n\nFor more information, try '--help'.");
        }

        if result.in_place && result.check {
            bail!("error: the argument '--in-place' cannot be used with '--check'");
        }

        Ok(result)
    }
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test basic parsing
    #[test]
    fn parse_requires_path() {
        let result = Args::parse_from(["ascfix"]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("required arguments"));
    }

    #[test]
    fn parse_single_file_defaults() {
        let args = Args::parse_from(["ascfix", "test.md"]).unwrap();

        assert_eq!(args.paths, vec![PathBuf::from("test.md")]);
        assert_eq!(args.mode, Mode::Diagram);
        assert!(!args.in_place);
        assert!(!args.check);
        assert!(!args.fences);
        assert!(!args.all);
    }

    #[test]
    fn parse_mode_diagram() {
        let args = Args::parse_from(["ascfix", "--mode", "diagram", "file.md"]).unwrap();

        assert_eq!(args.mode, Mode::Diagram);
    }

    #[test]
    fn parse_in_place() {
        let args = Args::parse_from(["ascfix", "--in-place", "file.md"]).unwrap();

        assert!(args.in_place);
    }

    #[test]
    fn parse_check_flag() {
        let args = Args::parse_from(["ascfix", "--check", "file.md"]).unwrap();

        assert!(args.check);
    }

    #[test]
    fn parse_max_size() {
        let args = Args::parse_from(["ascfix", "--max-size", "5MB", "file.md"]).unwrap();

        assert_eq!(args.max_size, Some(5 * 1024 * 1024));
    }

    #[test]
    fn check_conflicts_with_in_place() {
        let result = Args::parse_from(["ascfix", "--check", "--in-place", "file.md"]);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("cannot be used with"));
    }

    // Size parsing tests
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
        let result = parse_size("18446744073709551615GB");
        assert!(result.is_err());
    }

    // Extension normalization tests
    #[test]
    fn default_extensions_include_txt() {
        let args = Args::parse_from(["ascfix", "file.md"]).unwrap();
        let config: Config = args.into();

        assert!(config.extensions.contains(&".md".to_string()));
        assert!(config.extensions.contains(&".mdx".to_string()));
        assert!(config.extensions.contains(&".txt".to_string()));
    }

    #[test]
    fn extension_normalization_trim_lowercase_dot() {
        let args = Args::parse_from(["ascfix", "--ext", "md,.MD, txt ,.txt", "file.md"]).unwrap();

        let config: Config = args.into();

        assert_eq!(
            config.extensions,
            vec![".md".to_string(), ".txt".to_string()]
        );
    }

    #[test]
    fn extension_order_preserved() {
        let args = Args::parse_from(["ascfix", "--ext", "txt,md", "file.md"]).unwrap();

        let config: Config = args.into();

        assert_eq!(
            config.extensions,
            vec![".txt".to_string(), ".md".to_string()]
        );
    }

    // --all semantic expansion
    #[test]
    fn all_expands_to_diagram_and_fences() {
        let args = Args::parse_from(["ascfix", "--all", "file.md"]).unwrap();

        let config: Config = args.into();

        assert_eq!(config.mode, Mode::Diagram);
        assert!(config.fences);
    }

    #[test]
    fn all_overrides_safe_mode() {
        let args = Args::parse_from(["ascfix", "--mode", "safe", "--all", "file.md"]).unwrap();

        let config: Config = args.into();

        assert_eq!(config.mode, Mode::Diagram);
        assert!(config.fences);
    }

    // Complex configuration
    #[test]
    fn complex_configuration() {
        let args = Args::parse_from([
            "ascfix",
            "--mode",
            "diagram",
            "--max-size",
            "10MB",
            "--ext",
            "md,txt",
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
        assert_eq!(config.paths, vec![PathBuf::from("docs/")]);
    }
}
