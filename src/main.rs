//! ascfix — Automatic ASCII diagram repair tool for Markdown files.
//!
//! This tool normalizes and fixes ASCII diagrams in Markdown, including:
//! - Markdown tables (column alignment)
//! - ASCII boxes and arrows
//! - Text row formatting

mod cli;
mod detector;
pub mod grid;
mod io;
mod modes;
mod normalizer;
mod parser;
mod primitives;
mod processor;
mod renderer;
mod scanner;

use anyhow::Result;
use cli::Args;
use processor::Processor;

fn main() -> Result<()> {
    let args = Args::parse_args();
    let processor = Processor::new(args);
    let exit_code = processor.process_all()?;

    // Exit with appropriate code (0 for success, 1 for check mode failures)
    std::process::exit(exit_code);
}
