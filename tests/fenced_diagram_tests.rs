//! Tests for processing ASCII diagrams inside code fences.

use ascfix::cli::Mode;
use ascfix::config::Config;
use ascfix::modes::process_by_mode;

/// Diagram inside a bare code fence should be processed when `fenced_diagrams` is enabled.
#[test]
fn test_bare_fence_diagram_processed() {
    let input = "\
# Architecture

```
┌──────┐
│Hello │
└──────┘
```

Some text.
";
    let config = Config {
        fenced_diagrams: true,
        ..Default::default()
    };
    let result = process_by_mode(&Mode::Diagram, input, false, &config);
    // The box should be normalized (padding added)
    assert!(result.contains("Hello"), "Content lost:\n{result}");
    // Fence markers must be preserved
    assert!(result.contains("```"), "Fence markers lost:\n{result}");
    // The box should still be inside a fence block
    let lines: Vec<&str> = result.lines().collect();
    let fence_positions: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l.trim() == "```")
        .map(|(i, _)| i)
        .collect();
    assert!(
        fence_positions.len() >= 2,
        "Expected at least 2 fence markers, got {fence_positions:?}\n{result}"
    );
}

/// Diagram inside a language-tagged fence should NOT be processed.
#[test]
fn test_language_tagged_fence_untouched() {
    let input = "\
```python
┌──────┐
│Hello │
└──────┘
```
";
    let config = Config {
        fenced_diagrams: true,
        ..Default::default()
    };
    let result = process_by_mode(&Mode::Diagram, input, false, &config);
    // Content should be exactly preserved (language-tagged fences are code)
    assert_eq!(result, input, "Language-tagged fence content was modified");
}

/// Default config (`fenced_diagrams=false`) should not touch fence content.
#[test]
fn test_fenced_diagrams_disabled_by_default() {
    let input = "\
```
┌──────┐
│Hello │
└──────┘
```
";
    let config = Config::default();
    let result = process_by_mode(&Mode::Diagram, input, false, &config);
    assert_eq!(
        result, input,
        "Fence content was modified with default config"
    );
}

/// Mixed content: bare fence with diagram + language-tagged fence with code.
/// Only the bare fence's diagram should be processed.
#[test]
fn test_mixed_fences_only_bare_processed() {
    let input = "\
# Mixed

```
┌──────┐
│Hello │
└──────┘
```

```bash
echo 'hi'
```
";
    let config = Config {
        fenced_diagrams: true,
        ..Default::default()
    };
    let result = process_by_mode(&Mode::Diagram, input, false, &config);
    // Bash code block must be untouched
    assert!(
        result.contains("echo 'hi'"),
        "Bash code block was modified:\n{result}"
    );
    assert!(
        result.contains("```bash"),
        "Bash fence marker lost:\n{result}"
    );
    // Diagram content should still exist
    assert!(result.contains("Hello"), "Diagram content lost:\n{result}");
}
