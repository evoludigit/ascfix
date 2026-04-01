//! Tests for box detection and rendering, especially connected boxes with junction characters.

use ascfix::cli::Mode;
use ascfix::config::Config;
use ascfix::detector::detect_boxes;
use ascfix::grid::Grid;
use ascfix::modes::process_by_mode;
use ascfix::primitives::BoxStyle;

/// Baseline: a single box with clean corners must be detected.
#[test]
fn test_detect_single_box() {
    let grid = Grid::from_lines(&[
        "┌─────┐",
        "│ Box │",
        "└─────┘",
    ]);
    let boxes = detect_boxes(&grid);
    assert_eq!(boxes.len(), 1);
    assert_eq!(boxes[0].top_left, (0, 0));
    assert_eq!(boxes[0].bottom_right, (2, 6));
    assert_eq!(boxes[0].style, BoxStyle::Single);
}

/// Two boxes side by side (not connected) must both be detected.
#[test]
fn test_detect_two_separate_boxes() {
    let grid = Grid::from_lines(&[
        "┌───┐   ┌───┐",
        "│ A │   │ B │",
        "└───┘   └───┘",
    ]);
    let boxes = detect_boxes(&grid);
    assert_eq!(boxes.len(), 2);
}

/// Two boxes connected by a vertical arrow with ┬/┴ junctions.
/// This is the core bug: the flood-fill merges them into one component
/// whose bounding box doesn't have corners, so zero boxes are detected.
#[test]
fn test_detect_boxes_connected_by_vertical_arrow() {
    let grid = Grid::from_lines(&[
        "┌─────┐",
        "│ Top │",
        "└──┬──┘",
        "   │   ",
        "┌──┴──┐",
        "│ Bot │",
        "└─────┘",
    ]);
    let boxes = detect_boxes(&grid);
    assert_eq!(boxes.len(), 2, "Expected 2 boxes, got {boxes:?}");
}

/// Three boxes in a vertical chain connected by ┬/┴ junctions.
#[test]
fn test_detect_three_connected_boxes_vertical() {
    let grid = Grid::from_lines(&[
        "┌─────┐",
        "│  A  │",
        "└──┬──┘",
        "   │   ",
        "┌──┴──┐",
        "│  B  │",
        "└──┬──┘",
        "   │   ",
        "┌──┴──┐",
        "│  C  │",
        "└─────┘",
    ]);
    let boxes = detect_boxes(&grid);
    assert_eq!(boxes.len(), 3, "Expected 3 boxes, got {boxes:?}");
}

/// Box with ┬ on bottom connecting to two boxes below via ┴ on top (tree shape).
#[test]
fn test_detect_tree_shaped_connected_boxes() {
    let grid = Grid::from_lines(&[
        "        ┌───────┐        ",
        "        │Parent │        ",
        "        └───┬───┘        ",
        "        ┌───┴───┐        ",
        "        │       │        ",
        "   ┌────┴──┐ ┌──┴─────┐ ",
        "   │ Left  │ │ Right  │ ",
        "   └───────┘ └────────┘ ",
    ]);
    let boxes = detect_boxes(&grid);
    assert_eq!(boxes.len(), 3, "Expected 3 boxes (Parent, Left, Right), got {boxes:?}");
}

/// Boxes with arrow symbols (▼) embedded in borders must still be detected.
#[test]
fn test_detect_boxes_with_arrow_symbols_in_border() {
    let grid = Grid::from_lines(&[
        "┌──────────┐",
        "│ Start    │",
        "└─────┬────┘",
        "      │",
        "      └─────┬──────────┐",
        "            │          │",
        "        ┌───▼────┐  ┌──▼───┐",
        "        │ Path 1 │  │ Path2 │",
        "        └────────┘  └───────┘",
    ]);
    let boxes = detect_boxes(&grid);
    assert_eq!(boxes.len(), 3, "Expected Start + Path1 + Path2: {boxes:?}");
    assert!(boxes.iter().any(|b| b.top_left == (0, 0)));
    assert!(boxes.iter().any(|b| b.top_left == (6, 8)));
    assert!(boxes.iter().any(|b| b.top_left == (6, 20)));
}

/// Nested boxes (parent containing children) with off-by-one border widths.
#[test]
fn test_detect_nested_boxes_with_misaligned_borders() {
    let grid = Grid::from_lines(&[
        "┌─────────────────────────────────────┐",
        "│ Parent Container                    │",
        "│                                     │",
        "│  ┌──────────┐    ┌──────────┐      │",
        "│  │ Child 1  │    │ Child 2  │      │",
        "│  └──────────┘    └──────────┘      │",
        "│                                     │",
        "└─────────────────────────────────────┘",
    ]);
    let boxes = detect_boxes(&grid);
    assert_eq!(boxes.len(), 3, "Expected parent + 2 children: {boxes:?}");
}

/// Connected boxes with uneven padding must not produce ││ artifacts.
/// This is the exact pattern from `example_readme`: boxes connected by ┬/┴
/// where content touches borders (no right padding).
#[test]
fn test_no_double_border_connected_boxes() {
    let input = "\
┌──────────────┐
│ Gateway :4000 │
└──────┬───────┘
┌──────┴──┐
│ Backend │
│  :4001  │
└─────────┘";
    let config = Config::default();
    let result = process_by_mode(&Mode::Diagram, input, false, &config);
    assert!(
        !result.contains("││"),
        "Double border artifact found:\n{result}"
    );
    assert!(result.contains("Gateway"), "Content lost:\n{result}");
    assert!(result.contains("Backend"), "Content lost:\n{result}");
}

/// Box with multi-line content where some lines have no right padding.
#[test]
fn test_no_double_border_multiline_uneven_padding() {
    let input = "\
┌───────────┐
│  Sidecar  │
│ (FastAPI)  │
│  AI + IMAP │
│  + registry│
└───────────┘";
    let config = Config::default();
    let result = process_by_mode(&Mode::Diagram, input, false, &config);
    assert!(
        !result.contains("││"),
        "Double border artifact found:\n{result}"
    );
    assert!(result.contains("Sidecar"), "Content lost:\n{result}");
    assert!(result.contains("registry"), "Content lost:\n{result}");
}

/// Side-by-side boxes must preserve both boxes' content through the normalizer.
#[test]
fn test_side_by_side_boxes_preserve_both_contents() {
    let input = "\
┌────────────┐ ┌──────────────┐
│ Management │ │   Backend    │
│   :8042    │ │    :4001     │
└────────────┘ └──────────────┘";
    let config = Config::default();
    let result = process_by_mode(&Mode::Diagram, input, false, &config);
    assert!(result.contains("Management"), "Management lost:\n{result}");
    assert!(result.contains("Backend"), "Backend lost:\n{result}");
    assert!(result.contains(":8042"), ":8042 lost:\n{result}");
    assert!(result.contains(":4001"), ":4001 lost:\n{result}");
}
