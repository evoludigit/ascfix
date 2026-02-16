//! Test data generators for stress tests

/// Generate a large markdown file with mixed content
#[allow(dead_code)]
pub fn generate_large_markdown(size_bytes: usize) -> String {
    let mut content = String::with_capacity(size_bytes);
    let mut current_size = 0;

    let patterns = vec![
        "# Heading\n\nSome text here.\n\n",
        "- List item 1\n- List item 2\n\n",
        "| Col1 | Col2 |\n|------|------|\n| A | B |\n\n",
        "```\ncode block\n```\n\n",
        "┌─────┐\n│ Box │\n└─────┘\n\n",
    ];

    while current_size < size_bytes {
        for pattern in &patterns {
            content.push_str(pattern);
            current_size += pattern.len();
            if current_size >= size_bytes {
                break;
            }
        }
    }

    content
}

/// Generate deeply nested boxes
#[allow(dead_code)]
pub fn generate_nested_boxes(depth: usize) -> String {
    if depth == 0 {
        return String::from("inner");
    }

    let inner = generate_nested_boxes(depth - 1);
    let lines: Vec<&str> = inner.lines().collect();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(5) + 4;

    let mut result = String::new();
    result.push('┌');
    result.push_str(&"─".repeat(width - 2));
    result.push_str("┐\n│");
    result.push_str(&" ".repeat(width - 2));
    result.push_str("│\n");

    for line in &lines {
        result.push_str("│  ");
        result.push_str(line);
        result.push_str(&" ".repeat(width - 6 - line.len()));
        result.push_str("  │\n");
    }

    result.push('│');
    result.push_str(&" ".repeat(width - 2));
    result.push_str("│\n└");
    result.push_str(&"─".repeat(width - 2));
    result.push('┘');

    result
}

/// Generate a wide table with many columns
#[allow(dead_code)]
pub fn generate_wide_table(columns: usize) -> String {
    let headers: Vec<String> = (1..=columns).map(|i| format!("Col{i}")).collect();
    let separator = vec!["---"; columns].join(" | ");
    let data_row: Vec<String> = (1..=columns).map(|i| format!("Data{i}")).collect();

    format!(
        "| {} |\n|{}|\n| {} |",
        headers.join(" | "),
        separator,
        data_row.join(" | ")
    )
}

/// Generate a grid of boxes
#[allow(dead_code)]
pub fn generate_box_grid(rows: usize, cols: usize) -> String {
    let box_pattern = "┌───┐\n│   │\n└───┘";
    let mut result = String::new();

    for _row in 0..rows {
        for line_idx in 0..3 {
            for _col in 0..cols {
                let box_line = box_pattern.lines().nth(line_idx).unwrap();
                result.push_str(box_line);
                result.push(' ');
            }
            result.push('\n');
        }
        result.push('\n');
    }

    result
}

/// Generate markdown with extremely long lines
#[allow(dead_code)]
pub fn generate_long_lines(line_length: usize, num_lines: usize) -> String {
    let line = "a".repeat(line_length);
    let mut result = String::new();
    for _ in 0..num_lines {
        result.push_str(&line);
        result.push('\n');
    }
    result
}

/// Generate a file with many consecutive blank lines
#[allow(dead_code)]
pub fn generate_many_blank_lines(num_blanks: usize) -> String {
    let mut result = String::from("# Header\n\n");
    for _ in 0..num_blanks {
        result.push('\n');
    }
    result.push_str("Content\n");
    result
}

/// Generate markdown with deeply nested lists
#[allow(dead_code)]
#[allow(clippy::format_push_string)]
pub fn generate_nested_lists(depth: usize) -> String {
    let mut result = String::new();
    for level in 0..depth {
        result.push_str(&"  ".repeat(level));
        result.push_str(&format!("- Item at level {level}\n"));
    }
    result
}
