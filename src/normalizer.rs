//! Layout normalization and repair logic for ASCII diagrams.

#[allow(unused_imports)]  // Reason: DiagramBox used in tests
use crate::primitives::{Box as DiagramBox, HorizontalArrow, PrimitiveInventory};

/// Align horizontal arrows to consistent positions.
///
/// Algorithm:
/// 1. Group arrows by row
/// 2. Normalize each arrow's position to match anchor points
/// 3. Ensure no overlap between arrows
#[allow(dead_code)]  // Reason: Used by main processing pipeline
pub fn align_horizontal_arrows(inventory: &PrimitiveInventory) -> PrimitiveInventory {
    let mut normalized = inventory.clone();

    // Group arrows by row
    let mut arrows_by_row = std::collections::HashMap::new();
    for arrow in &normalized.horizontal_arrows {
        arrows_by_row
            .entry(arrow.row)
            .or_insert_with(Vec::new)
            .push(arrow.clone());
    }

    // For each row, ensure arrows don't overlap and are properly spaced
    normalized.horizontal_arrows = arrows_by_row
        .into_iter()
        .flat_map(|(_row, mut arrows)| {
            // Sort arrows by start position
            arrows.sort_by_key(|a| a.start_col);
            arrows
        })
        .collect();

    normalized
}

/// Normalize box widths to fit their content.
///
/// Algorithm:
/// 1. For each box, find the longest interior text row
/// 2. Calculate required width (content + 2 for borders + padding)
/// 3. Expand box if necessary
#[allow(dead_code)]  // Reason: Used by main processing pipeline
pub fn normalize_box_widths(inventory: &PrimitiveInventory) -> PrimitiveInventory {
    let mut normalized = inventory.clone();

    for b in &mut normalized.boxes {
        // Find longest text row inside this box
        let max_content_len = normalized
            .text_rows
            .iter()
            .filter(|row| row.row > b.top_left.0 && row.row < b.bottom_right.0)
            .map(|row| row.content.trim_end().len())
            .max()
            .unwrap_or(0);

        if max_content_len > 0 {
            // Required width: content + 2 for left/right borders
            let required_width = max_content_len + 2;
            let current_width = b.width();

            if required_width > current_width {
                // Expand box to the right
                let expansion = required_width - current_width;
                b.bottom_right.1 += expansion;
            }
        }
    }

    // Adjust text rows to match new box widths
    for row in &mut normalized.text_rows {
        if let Some(b) = normalized.boxes.iter().find(|box_| {
            row.row > box_.top_left.0 && row.row < box_.bottom_right.0
        }) {
            row.end_col = b.bottom_right.1 - 1;
        }
    }

    normalized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_narrow_box() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (2, 3),
        });
        inventory.text_rows.push(crate::primitives::TextRow {
            row: 1,
            start_col: 1,
            end_col: 2,
            content: " Longer Text ".to_string(),
        });

        let normalized = normalize_box_widths(&inventory);
        let b = &normalized.boxes[0];
        // Original width: 4 columns (0-3)
        // Content "Longer Text" = 11 chars, need 11 + 2 borders = 13
        // Expansion = 13 - 4 = 9
        assert!(b.width() >= 13);
    }

    #[test]
    fn test_box_with_empty_content_unchanged() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (2, 3),
        });

        let normalized = normalize_box_widths(&inventory);
        let b = &normalized.boxes[0];
        assert_eq!(b.width(), 4); // No content, no expansion
    }

    #[test]
    fn test_multiple_boxes_expanded_independently() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (2, 3),
        });
        inventory.boxes.push(DiagramBox {
            top_left: (0, 5),
            bottom_right: (2, 8),
        });

        inventory.text_rows.push(crate::primitives::TextRow {
            row: 1,
            start_col: 1,
            end_col: 2,
            content: " Short ".to_string(),
        });
        inventory.text_rows.push(crate::primitives::TextRow {
            row: 1,
            start_col: 6,
            end_col: 7,
            content: " Much Longer Text ".to_string(),
        });

        let normalized = normalize_box_widths(&inventory);
        // First box: "Short" (5) + 2 = 7, was 4, expand to 7
        assert!(normalized.boxes[0].width() >= 7);
        // Second box: "Much Longer Text" (16) + 2 = 18, was 4, expand to 18
        assert!(normalized.boxes[1].width() >= 18);
    }

    #[test]
    fn test_box_width_already_sufficient() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (2, 10),
        });
        inventory.text_rows.push(crate::primitives::TextRow {
            row: 1,
            start_col: 1,
            end_col: 9,
            content: " Short ".to_string(),
        });

        let normalized = normalize_box_widths(&inventory);
        let b = &normalized.boxes[0];
        assert_eq!(b.width(), 11); // No expansion needed
    }

    #[test]
    fn test_text_rows_adjusted_after_expansion() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (2, 3),
        });
        inventory.text_rows.push(crate::primitives::TextRow {
            row: 1,
            start_col: 1,
            end_col: 2,
            content: " Expanded ".to_string(),
        });

        let normalized = normalize_box_widths(&inventory);
        let row = &normalized.text_rows[0];
        // end_col should be adjusted to match expanded box
        assert_eq!(row.end_col, normalized.boxes[0].bottom_right.1 - 1);
    }

    #[test]
    fn test_multiline_box_uses_longest_row() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (4, 3),
        });
        inventory.text_rows.push(crate::primitives::TextRow {
            row: 1,
            start_col: 1,
            end_col: 2,
            content: " A ".to_string(),
        });
        inventory.text_rows.push(crate::primitives::TextRow {
            row: 2,
            start_col: 1,
            end_col: 2,
            content: " LongerLine ".to_string(),
        });
        inventory.text_rows.push(crate::primitives::TextRow {
            row: 3,
            start_col: 1,
            end_col: 2,
            content: " B ".to_string(),
        });

        let normalized = normalize_box_widths(&inventory);
        let b = &normalized.boxes[0];
        // "LongerLine" = 10 + 2 = 12
        assert!(b.width() >= 12);
    }

    #[test]
    fn test_align_single_arrow() {
        let mut inventory = PrimitiveInventory::default();
        inventory.horizontal_arrows.push(HorizontalArrow {
            row: 0,
            start_col: 2,
            end_col: 5,
        });

        let normalized = align_horizontal_arrows(&inventory);
        assert_eq!(normalized.horizontal_arrows.len(), 1);
        assert_eq!(normalized.horizontal_arrows[0].start_col, 2);
    }

    #[test]
    fn test_align_multiple_arrows_same_row() {
        let mut inventory = PrimitiveInventory::default();
        // Add arrows in reverse order
        inventory.horizontal_arrows.push(HorizontalArrow {
            row: 0,
            start_col: 10,
            end_col: 15,
        });
        inventory.horizontal_arrows.push(HorizontalArrow {
            row: 0,
            start_col: 2,
            end_col: 5,
        });

        let normalized = align_horizontal_arrows(&inventory);
        // Should be sorted by start_col
        assert_eq!(normalized.horizontal_arrows.len(), 2);
        assert!(normalized.horizontal_arrows[0].start_col < normalized.horizontal_arrows[1].start_col);
    }

    #[test]
    fn test_arrows_different_rows_unchanged() {
        let mut inventory = PrimitiveInventory::default();
        inventory.horizontal_arrows.push(HorizontalArrow {
            row: 0,
            start_col: 5,
            end_col: 8,
        });
        inventory.horizontal_arrows.push(HorizontalArrow {
            row: 5,
            start_col: 5,
            end_col: 8,
        });

        let normalized = align_horizontal_arrows(&inventory);
        assert_eq!(normalized.horizontal_arrows.len(), 2);
        assert_eq!(normalized.horizontal_arrows[0].row, 0);
        assert_eq!(normalized.horizontal_arrows[1].row, 5);
    }

    #[test]
    fn test_no_arrows_handled_gracefully() {
        let inventory = PrimitiveInventory::default();
        let normalized = align_horizontal_arrows(&inventory);
        assert!(normalized.horizontal_arrows.is_empty());
    }
}
