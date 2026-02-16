//! ascfix library — Automatic ASCII diagram repair tool for Markdown files.
//!
//! This library exposes the core functionality for processing and normalizing
//! ASCII diagrams in Markdown content.

pub mod cli;
pub mod config;
pub mod discovery;
pub mod fences;
pub mod links;
pub mod lists;
pub mod modes;
pub mod output;
pub mod tables;

// Internal modules (not part of public API)
pub mod detector;
pub mod grid;
pub mod io;
pub mod normalizer;
pub mod parser;
pub mod primitives;
pub mod processor;
pub mod quality;
pub mod renderer;
pub mod scanner;
pub mod transformation_analysis;
