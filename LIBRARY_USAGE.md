# Using ascfix as a Library

ascfix can be integrated into your Rust projects as a library for programmatic ASCII diagram and Markdown table fixing.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ascfix = "0.5"
```

## Basic Usage

### Simple Example: Process Text with Default Settings

```rust
use ascfix::modes::process_by_mode;
use ascfix::cli::Mode;
use ascfix::config::Config;

fn main() {
    let markdown = r#"
┌──────┐
│ Box  │
└──────┘
"#;

    let fixed = process_by_mode(
        &Mode::Diagram,      // Processing mode
        markdown,            // Input text
        false,               // repair_fences
        &Config::default()   // Configuration
    );

    println!("{}", fixed);
}
```

### Processing Modes

ascfix provides three processing modes:

```rust
use ascfix::cli::Mode;

// Safe mode: Fix tables only
let result = process_by_mode(&Mode::Safe, text, false, &config);

// Diagram mode: Fix tables and ASCII diagrams (recommended)
let result = process_by_mode(&Mode::Diagram, text, false, &config);

// Check mode: Validate without making changes
let result = process_by_mode(&Mode::Check, text, false, &config);
```

## Custom Configuration

### Creating a Custom Config

```rust
use ascfix::config::Config;
use ascfix::modes::process_by_mode;
use ascfix::cli::Mode;

let mut config = Config::default();
config.max_file_size = Some(50_000_000); // 50MB
config.respect_gitignore = true;

let text = r#"
│ Table │ Cell │
│ Data  │ Here │
"#;

let fixed = process_by_mode(&Mode::Diagram, text, false, &config);
```

### Config Options

```rust
pub struct Config {
    /// Maximum file size in bytes (None = unlimited)
    pub max_file_size: Option<usize>,

    /// Respect .gitignore files when processing
    pub respect_gitignore: bool,

    /// Additional configuration options...
}
```

## Advanced Usage

### Quality Validation

Validate the quality of transformations:

```rust
use ascfix::quality::{validate_quality, QualityConfig};

let input = "┌───┐\n│box│\n└───┘";
let output = process_by_mode(
    &Mode::Diagram,
    input,
    false,
    &Config::default()
);

// Validate quality
let report = validate_quality(input, &output);

let quality_config = QualityConfig {
    min_text_preservation: 0.85,
    min_structure_preservation: 0.80,
    max_line_count_delta: 2,
    allow_text_corruption: false,
    allow_data_loss: false,
};

if report.is_acceptable(&quality_config) {
    println!("Transformation passed quality checks");
} else {
    println!("Transformation did not meet quality standards");
    println!("Issues: {:?}", report.issues);
}
```

### Working with Diagram Blocks

Extract and process only diagram blocks:

```rust
use ascfix::scanner::extract_diagram_blocks;
use ascfix::modes::process_diagram_block;

let text = r#"
# My Document

Some intro text

┌──────────┐
│ Diagram  │
└──────────┘

Some conclusion
"#;

// Extract diagram blocks
let blocks = extract_diagram_blocks(text);

for block in blocks {
    println!("Found diagram at line {}", block.start_line);
    println!("Block: {}", block.content);
}
```

### Table Processing

Process tables specifically:

```rust
use ascfix::tables::process_wrapped_tables;

let markdown = r#"
| Column 1 | Column 2     |
|----------|--------------|
| Short    | This is a ve |
|          | ry long cell |
"#;

let fixed = process_wrapped_tables(markdown);
println!("{}", fixed);
```

### Fence Repair

Repair unmatched code fence markers:

```rust
use ascfix::fences::repair_code_fences;

let markdown = r#"
```rust
fn main() {
    println!("Hello");
}
```rust  // Mismatched fence marker
"#;

let repaired = repair_code_fences(markdown);
```

## Error Handling

ascfix is designed to be safe - it won't panic on malformed input:

```rust
use ascfix::modes::process_by_mode;
use ascfix::cli::Mode;
use ascfix::config::Config;

let potentially_bad_input = "???";

// This won't panic - it returns the input unchanged if issues occur
let result = process_by_mode(
    &Mode::Diagram,
    potentially_bad_input,
    false,
    &Config::default()
);

assert_eq!(result, "???");
```

## Full Example: Building a Custom Tool

Here's a complete example of building a simple diagram fixer tool:

```rust
use ascfix::cli::Mode;
use ascfix::config::Config;
use ascfix::modes::process_by_mode;
use ascfix::quality::{validate_quality, QualityConfig};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read a markdown file
    let input = fs::read_to_string("document.md")?;

    // Create configuration
    let config = Config {
        max_file_size: Some(100_000_000),
        respect_gitignore: false,
    };

    // Process with diagram mode
    let output = process_by_mode(
        &Mode::Diagram,
        &input,
        false,
        &config
    );

    // Validate quality
    let report = validate_quality(&input, &output);

    let quality_config = QualityConfig {
        min_text_preservation: 0.85,
        min_structure_preservation: 0.80,
        max_line_count_delta: 2,
        allow_text_corruption: false,
        allow_data_loss: false,
    };

    if report.is_acceptable(&quality_config) {
        // Write fixed file
        fs::write("document.fixed.md", &output)?;
        println!("✓ Document fixed successfully");
        println!("Quality score: {:.2}", report.score);
    } else {
        println!("✗ Transformation did not meet quality standards");
        println!("Issues found: {}", report.issues.len());
        for issue in &report.issues {
            println!("  - {:?}", issue);
        }
    }

    Ok(())
}
```

## Type Reference

### Key Types

```rust
// Main processing function
pub fn process_by_mode(
    mode: &Mode,
    text: &str,
    repair_fences: bool,
    config: &Config,
) -> String

// Processing modes
pub enum Mode {
    Safe,
    Diagram,
    Check,
}

// Quality validation
pub struct QualityReport {
    pub score: f32,
    pub issues: Vec<QualityIssue>,
    pub metrics: QualityMetrics,
}

// Configuration
pub struct Config {
    pub max_file_size: Option<usize>,
    pub respect_gitignore: bool,
    // ... other options
}
```

## Performance Tips

1. **Batch Processing:** Process multiple files concurrently if needed
2. **Size Limits:** Use `max_file_size` to skip large files
3. **Mode Selection:** Use `Safe` mode for performance-critical scenarios
4. **Caching:** Cache config objects if processing many files

## Troubleshooting

**Q: Output is unchanged from input**
- Check the Mode - `Safe` mode only fixes tables
- Use `Diagram` mode for ASCII diagram fixes

**Q: Getting unexpected transformations**
- Review `QualityConfig` thresholds
- Use quality validation to understand what's changing

**Q: Performance is slow**
- Set a reasonable `max_file_size`
- Consider using `Safe` mode for large batches
- Process files concurrently

## Integration Examples

### With structopt CLI

```rust
use structopt::StructOpt;
use ascfix::modes::process_by_mode;
use ascfix::cli::Mode;
use ascfix::config::Config;

#[derive(StructOpt)]
struct Args {
    #[structopt(short, long)]
    input: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();
    let text = std::fs::read_to_string(&args.input)?;

    let fixed = process_by_mode(
        &Mode::Diagram,
        &text,
        false,
        &Config::default()
    );

    println!("{}", fixed);
    Ok(())
}
```

### With Tokio for Async Processing

```rust
use std::fs;
use ascfix::modes::process_by_mode;
use ascfix::cli::Mode;
use ascfix::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = vec!["file1.md", "file2.md"];

    let futures = files.into_iter().map(|file| {
        async move {
            let text = fs::read_to_string(file).ok()?;
            let fixed = process_by_mode(
                &Mode::Diagram,
                &text,
                false,
                &Config::default()
            );
            Some(fixed)
        }
    });

    // Process concurrently
    let results = futures::future::join_all(futures).await;

    Ok(())
}
```

## Support

For issues or questions about using ascfix as a library, please refer to:
- [ARCHITECTURE.md](./ARCHITECTURE.md) - Design and module overview
- [README.md](./README.md) - General usage and capabilities
- GitHub Issues - Report bugs or suggest improvements
