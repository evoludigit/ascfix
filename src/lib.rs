//! ascfix library — Automatic ASCII diagram repair tool for Markdown files.
//!
//! This library exposes the core functionality for processing and normalizing
//! ASCII diagrams in Markdown content.

pub mod cli;
pub mod modes;

// Internal modules (not part of public API)
pub mod detector;
pub mod fences;
pub mod grid;
pub mod normalizer;
pub mod parser;
pub mod primitives;
pub mod renderer;
pub mod scanner;
