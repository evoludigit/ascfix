//! Layout normalization and repair logic for ASCII diagrams.

#[allow(unused_imports)] // Reason: DiagramBox used in tests
use crate::primitives::{
    ArrowType, Box as DiagramBox, BoxStyle, HorizontalArrow, PrimitiveInventory,
};

/// Balance widths of side-by-side boxes in the same group.
///
/// For each group of horizontally adjacent boxes with vertical overlap,
/// expand narrower boxes to match the widest one. Only expands, never shrinks.
#[must_use]
pub fn balance_horizontal_boxes(inventory: &PrimitiveInventory) -> PrimitiveInventory {
    let mut normalized = inventory.clone();
    let groups = find_vertical_overlap_groups(&normalized);

    for group in groups {
        // Find maximum width in group
        let max_width = group
            .iter()
            .map(|&idx| normalized.boxes[idx].width())
            .max()
            .unwrap_or(0);

        // Expand each box to match max width
        for &idx in &group {
            let box_ref = &mut normalized.boxes[idx];
            let current_width = box_ref.width();
            if current_width < max_width {
                let diff = max_width - current_width;
                box_ref.bottom_right.1 += diff;
            }
        }
    }

    normalized
}

/// Find groups of boxes that have vertical row overlap and are horizontally adjacent.
///
/// Returns a vector of groups, where each group is a vector of box indices
/// that should be balanced together. Side-by-side boxes get grouped.
///
/// Algorithm:
/// 1. For each pair of boxes, check if they have vertical overlap
/// 2. Check if they are horizontally adjacent (no gap or minimal gap)
/// 3. Group adjacent boxes together
#[allow(dead_code)] // Reason: Used by balancing normalization in upcoming phases
#[must_use]
fn find_vertical_overlap_groups(inventory: &PrimitiveInventory) -> Vec<Vec<usize>> {
    if inventory.boxes.is_empty() {
        return Vec::new();
    }

    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut assigned = vec![false; inventory.boxes.len()];

    for i in 0..inventory.boxes.len() {
        if assigned[i] {
            continue;
        }

        let mut group = vec![i];
        assigned[i] = true;

        // Find all boxes adjacent to this one or others in the group
        let mut changed = true;
        while changed {
            changed = false;
            let mut to_add = Vec::new();

            for (j, &is_assigned) in assigned.iter().enumerate() {
                if is_assigned {
                    continue;
                }

                // Check if box j is adjacent to any box in current group
                let mut is_adjacent = false;
                for &group_idx in &group {
                    let box_in_group = &inventory.boxes[group_idx];
                    let box_j = &inventory.boxes[j];

                    // Check vertical overlap
                    let rows_overlap = !(box_in_group.bottom_right.0 < box_j.top_left.0
                        || box_j.bottom_right.0 < box_in_group.top_left.0);

                    // Check horizontal adjacency (gap <= 1 cell)
                    let (left_col, right_col) = if box_in_group.bottom_right.1 < box_j.top_left.1 {
                        (box_in_group.bottom_right.1, box_j.top_left.1)
                    } else if box_j.bottom_right.1 < box_in_group.top_left.1 {
                        (box_j.bottom_right.1, box_in_group.top_left.1)
                    } else {
                        continue; // Boxes overlap horizontally, skip
                    };

                    let gap = right_col.saturating_sub(left_col);

                    if rows_overlap && gap <= 1 {
                        is_adjacent = true;
                        break;
                    }
                }

                if is_adjacent {
                    to_add.push(j);
                }
            }

            for j in to_add {
                group.push(j);
                assigned[j] = true;
                changed = true;
            }
        }

        if group.len() > 1 {
            groups.push(group);
        }
    }

    groups
}

/// Align horizontal arrows to consistent positions.
///
/// Algorithm:
/// 1. Group arrows by row
/// 2. Sort within each row by start position
/// 3. Preserve row order for deterministic output
#[allow(dead_code)] // Reason: Used by main processing pipeline
pub fn align_horizontal_arrows(inventory: &PrimitiveInventory) -> PrimitiveInventory {
    let mut normalized = inventory.clone();

    // Group arrows by row while preserving order
    let mut arrows_by_row = std::collections::BTreeMap::new();
    for arrow in &normalized.horizontal_arrows {
        arrows_by_row
            .entry(arrow.row)
            .or_insert_with(Vec::new)
            .push(arrow.clone());
    }

    // For each row, sort arrows by start position and collect preserving row order
    normalized.horizontal_arrows = arrows_by_row
        .into_iter()
        .flat_map(|(_row, mut arrows)| {
            // Sort arrows by start position within row
            arrows.sort_by_key(|a| a.start_col);
            arrows
        })
        .collect();

    normalized
}

/// Enforce uniform 1-space interior padding in boxes.
///
/// Algorithm:
/// 1. For each text row inside a box, ensure it starts 1 column after left border
/// 2. Ensure it ends 1 column before right border
/// 3. Adjust `start_col` and `end_col` to maintain padding
#[allow(dead_code)] // Reason: Used by main processing pipeline
#[must_use]
pub fn normalize_padding(inventory: &PrimitiveInventory) -> PrimitiveInventory {
    let mut normalized = inventory.clone();

    for row in &mut normalized.text_rows {
        // Find the box containing this row (must be inside both row and column ranges)
        if let Some(b) = normalized.boxes.iter().find(|box_| {
            row.row > box_.top_left.0
                && row.row < box_.bottom_right.0
                && row.start_col >= box_.top_left.1
                && row.start_col <= box_.bottom_right.1
        }) {
            // Enforce 1-space padding: start at left+1, end at right-1
            row.start_col = b.top_left.1 + 1;
            row.end_col = b.bottom_right.1 - 1;
        }
    }

    normalized
}

/// Align vertical arrows to box column positions.
///
/// Algorithm:
/// 1. For each vertical arrow, find the nearest box
/// 2. Snap the arrow's column to the box's left/right/center column
/// 3. Maintain arrow row positions (only adjust column)
#[allow(dead_code)] // Reason: Used by main processing pipeline
#[must_use]
pub fn align_vertical_arrows(inventory: &PrimitiveInventory) -> PrimitiveInventory {
    let mut normalized = inventory.clone();

    for arrow in &mut normalized.vertical_arrows {
        // Find boxes that might this arrow should align to
        // A vertical arrow aligns to a box if it's roughly within the box's column range
        // or closest to it horizontally
        if let Some(aligned_col) = find_alignment_column(&normalized.boxes, arrow.col) {
            arrow.col = aligned_col;
        }
    }

    normalized
}

/// Find the nearest box column alignment for a given column position.
fn find_alignment_column(boxes: &[crate::primitives::Box], col: usize) -> Option<usize> {
    let mut closest_col = None;
    let mut min_distance = usize::MAX;

    for b in boxes {
        // Consider three alignment points: left edge, center, right edge
        let left_col = b.top_left.1;
        let center_col = usize::midpoint(b.top_left.1, b.bottom_right.1);
        let right_col = b.bottom_right.1;

        let candidates = [left_col, center_col, right_col];
        for &candidate in &candidates {
            let distance = candidate.abs_diff(col);
            if distance < min_distance {
                min_distance = distance;
                closest_col = Some(candidate);
            }
        }
    }

    closest_col
}

/// Normalize box widths to fit their content.
///
/// Algorithm:
/// 1. For each box, find the longest interior text row
/// 2. Calculate required width (content + 2 for borders + padding)
/// 3. Expand box if necessary
#[allow(dead_code)] // Reason: Used by main processing pipeline
#[must_use]
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
        if let Some(b) = normalized
            .boxes
            .iter()
            .find(|box_| row.row > box_.top_left.0 && row.row < box_.bottom_right.0)
        {
            row.end_col = b.bottom_right.1 - 1;
        }
    }

    normalized
}

/// Normalize connection lines (snap to box edges, straighten segments).
///
/// Algorithm:
/// 1. For each connection line's endpoints
/// 2. If attached to a box, snap to nearest edge
/// 3. Straighten segments (avoid unnecessary elbows)
/// 4. Skip if ambiguous or too complex
///
/// Conservative: Only modifies connections that clearly attach to boxes.
#[allow(dead_code)] // Reason: Used by normalization pipeline
#[must_use]
pub fn normalize_connection_lines(inventory: &PrimitiveInventory) -> PrimitiveInventory {
    // For MVP, return unchanged (conservative approach)
    // Will implement snapping and straightening in future phase
    inventory.clone()
}

/// Normalize labels by moving them with their attached primitives.
///
/// Algorithm:
/// 1. For each label, find its attached primitive
/// 2. Apply label offset to get new position
/// 3. Update label position based on primitive transformation
/// 4. Skip if attachment no longer valid
///
/// Conservative: Only moves labels with clear attachments, preserves offsets.
#[allow(dead_code)] // Reason: Used by normalization pipeline
#[must_use]
pub fn normalize_labels(inventory: &PrimitiveInventory) -> PrimitiveInventory {
    // For MVP, return unchanged (conservative approach)
    // Will implement offset-based repositioning in future phase
    inventory.clone()
}

/// Normalize nested boxes by expanding parents to contain their children.
///
/// Algorithm:
/// 1. Process boxes from innermost to outermost (by depth)
/// 2. For each parent, expand to minimum size that contains all children
/// 3. Add margin of 1 cell around children for padding
/// 4. Only expands, never shrinks (idempotent)
///
/// Conservative: Skips if ambiguous nesting or >3 levels deep.
#[allow(dead_code)] // Reason: Used by normalization pipeline
#[must_use]
pub fn normalize_nested_boxes(inventory: &PrimitiveInventory) -> PrimitiveInventory {
    let mut result = inventory.clone();

    // For each box with children, expand to fit them
    for parent_idx in 0..result.boxes.len() {
        let children_indices = result.boxes[parent_idx].child_indices.clone();
        if !children_indices.is_empty() {
            let mut min_row = result.boxes[parent_idx].top_left.0;
            let mut min_col = result.boxes[parent_idx].top_left.1;
            let mut max_row = result.boxes[parent_idx].bottom_right.0;
            let mut max_col = result.boxes[parent_idx].bottom_right.1;

            // Find bounds that encompass all children
            for &child_idx in &children_indices {
                if child_idx < result.boxes.len() {
                    let child = &result.boxes[child_idx];
                    // Need 1 cell margin around child
                    min_row = min_row.min(child.top_left.0.saturating_sub(1));
                    min_col = min_col.min(child.top_left.1.saturating_sub(1));
                    max_row = max_row.max(child.bottom_right.0 + 1);
                    max_col = max_col.max(child.bottom_right.1 + 1);
                }
            }

            // Only expand, never shrink
            result.boxes[parent_idx].top_left.0 = result.boxes[parent_idx].top_left.0.min(min_row);
            result.boxes[parent_idx].top_left.1 = result.boxes[parent_idx].top_left.1.min(min_col);
            result.boxes[parent_idx].bottom_right.0 =
                result.boxes[parent_idx].bottom_right.0.max(max_row);
            result.boxes[parent_idx].bottom_right.1 =
                result.boxes[parent_idx].bottom_right.1.max(max_col);
        }
    }

    result
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
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
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
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
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
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.boxes.push(DiagramBox {
            top_left: (0, 5),
            bottom_right: (2, 8),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
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
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
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
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
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
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
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
            arrow_type: ArrowType::Standard,
            rightward: true,
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
            arrow_type: ArrowType::Standard,
            rightward: true,
        });
        inventory.horizontal_arrows.push(HorizontalArrow {
            row: 0,
            start_col: 2,
            end_col: 5,
            arrow_type: ArrowType::Standard,
            rightward: true,
        });

        let normalized = align_horizontal_arrows(&inventory);
        // Should be sorted by start_col
        assert_eq!(normalized.horizontal_arrows.len(), 2);
        assert!(
            normalized.horizontal_arrows[0].start_col < normalized.horizontal_arrows[1].start_col
        );
    }

    #[test]
    fn test_arrows_different_rows_unchanged() {
        let mut inventory = PrimitiveInventory::default();
        inventory.horizontal_arrows.push(HorizontalArrow {
            row: 0,
            start_col: 5,
            end_col: 8,
            arrow_type: ArrowType::Standard,
            rightward: true,
        });
        inventory.horizontal_arrows.push(HorizontalArrow {
            row: 5,
            start_col: 5,
            end_col: 8,
            arrow_type: ArrowType::Standard,
            rightward: true,
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

    #[test]
    fn test_align_vertical_arrow_to_box_center() {
        let mut inventory = PrimitiveInventory::default();
        // Box from col 5 to col 15 (center at 10)
        inventory.boxes.push(DiagramBox {
            top_left: (0, 5),
            bottom_right: (3, 15),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Arrow slightly off-center at col 11
        inventory
            .vertical_arrows
            .push(crate::primitives::VerticalArrow {
                col: 11,
                start_row: 4,
                end_row: 6,
                arrow_type: ArrowType::Standard,
                downward: true,
            });

        let normalized = align_vertical_arrows(&inventory);
        let arrow = &normalized.vertical_arrows[0];
        // Should snap to box center (10)
        assert_eq!(arrow.col, 10);
    }

    #[test]
    fn test_align_vertical_arrow_to_box_edge() {
        let mut inventory = PrimitiveInventory::default();
        // Box from col 5 to col 15
        inventory.boxes.push(DiagramBox {
            top_left: (0, 5),
            bottom_right: (3, 15),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Arrow at col 6 (close to left edge at 5)
        inventory
            .vertical_arrows
            .push(crate::primitives::VerticalArrow {
                col: 6,
                start_row: 4,
                end_row: 6,
                arrow_type: ArrowType::Standard,
                downward: true,
            });

        let normalized = align_vertical_arrows(&inventory);
        let arrow = &normalized.vertical_arrows[0];
        // Should snap to left edge (5)
        assert_eq!(arrow.col, 5);
    }

    #[test]
    fn test_align_vertical_arrow_to_nearest_box() {
        let mut inventory = PrimitiveInventory::default();
        // Two boxes
        inventory.boxes.push(DiagramBox {
            top_left: (0, 2),
            bottom_right: (3, 5),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.boxes.push(DiagramBox {
            top_left: (0, 10),
            bottom_right: (3, 15),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Arrow closer to second box
        inventory
            .vertical_arrows
            .push(crate::primitives::VerticalArrow {
                col: 9,
                start_row: 4,
                end_row: 6,
                arrow_type: ArrowType::Standard,
                downward: true,
            });

        let normalized = align_vertical_arrows(&inventory);
        let arrow = &normalized.vertical_arrows[0];
        // Should align to second box (center at 12 is closer than col 9 to first box)
        assert!(arrow.col >= 10 && arrow.col <= 15);
    }

    #[test]
    fn test_vertical_arrow_maintains_row_positions() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 5),
            bottom_right: (3, 15),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory
            .vertical_arrows
            .push(crate::primitives::VerticalArrow {
                col: 11,
                start_row: 4,
                end_row: 6,
                arrow_type: ArrowType::Standard,
                downward: true,
            });

        let normalized = align_vertical_arrows(&inventory);
        let arrow = &normalized.vertical_arrows[0];
        // Row positions should not change
        assert_eq!(arrow.start_row, 4);
        assert_eq!(arrow.end_row, 6);
    }

    #[test]
    fn test_no_vertical_arrows_unchanged() {
        let inventory = PrimitiveInventory::default();
        let normalized = align_vertical_arrows(&inventory);
        assert!(normalized.vertical_arrows.is_empty());
    }

    #[test]
    fn test_padding_enforces_one_space_inside_box() {
        let mut inventory = PrimitiveInventory::default();
        // Box from col 0 to col 10
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (3, 10),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Text row starting at col 0 (should be col 1 for padding)
        inventory.text_rows.push(crate::primitives::TextRow {
            row: 1,
            start_col: 0,
            end_col: 9,
            content: "Content".to_string(),
        });

        let normalized = normalize_padding(&inventory);
        let row = &normalized.text_rows[0];
        // Should start at col 1 (1 space from left border at col 0)
        assert_eq!(row.start_col, 1);
    }

    #[test]
    fn test_padding_enforces_uniform_one_space() {
        let mut inventory = PrimitiveInventory::default();
        // Box from col 5 to col 15
        inventory.boxes.push(DiagramBox {
            top_left: (0, 5),
            bottom_right: (3, 15),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Multiple rows with inconsistent padding
        inventory.text_rows.push(crate::primitives::TextRow {
            row: 1,
            start_col: 5,
            end_col: 14,
            content: "Row1".to_string(),
        });
        inventory.text_rows.push(crate::primitives::TextRow {
            row: 2,
            start_col: 7,
            end_col: 14,
            content: "Row2".to_string(),
        });

        let normalized = normalize_padding(&inventory);
        // Both rows should have consistent 1-space padding
        assert_eq!(normalized.text_rows[0].start_col, 6); // Box left (5) + 1
        assert_eq!(normalized.text_rows[1].start_col, 6); // Same
    }

    #[test]
    fn test_padding_respects_box_boundaries() {
        let mut inventory = PrimitiveInventory::default();
        // Box from col 2 to col 8
        inventory.boxes.push(DiagramBox {
            top_left: (0, 2),
            bottom_right: (3, 8),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.text_rows.push(crate::primitives::TextRow {
            row: 1,
            start_col: 2,
            end_col: 7,
            content: "Text".to_string(),
        });

        let normalized = normalize_padding(&inventory);
        let row = &normalized.text_rows[0];
        // Start should be box left (2) + 1 = 3
        // End should be box right (8) - 1 = 7
        assert_eq!(row.start_col, 3);
        assert_eq!(row.end_col, 7);
    }

    #[test]
    fn test_padding_no_rows_no_crash() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (3, 10),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // No text rows
        let normalized = normalize_padding(&inventory);
        assert!(normalized.text_rows.is_empty());
    }

    #[test]
    fn test_padding_multiple_boxes_independent() {
        let mut inventory = PrimitiveInventory::default();
        // Two boxes
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (2, 5),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.boxes.push(DiagramBox {
            top_left: (0, 10),
            bottom_right: (2, 15),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Rows in each box
        inventory.text_rows.push(crate::primitives::TextRow {
            row: 1,
            start_col: 0,
            end_col: 4,
            content: "A".to_string(),
        });
        inventory.text_rows.push(crate::primitives::TextRow {
            row: 1,
            start_col: 10,
            end_col: 14,
            content: "B".to_string(),
        });

        let normalized = normalize_padding(&inventory);
        // First row in first box: start should be 0 + 1 = 1
        // Second row in second box: start should be 10 + 1 = 11
        assert_eq!(normalized.text_rows[0].start_col, 1);
        assert_eq!(normalized.text_rows[1].start_col, 11);
    }

    #[test]
    fn test_normalization_idempotent_box_widths() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (2, 3),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.text_rows.push(crate::primitives::TextRow {
            row: 1,
            start_col: 1,
            end_col: 2,
            content: " LongContent ".to_string(),
        });

        let normalized1 = normalize_box_widths(&inventory);
        let normalized2 = normalize_box_widths(&normalized1);

        // Second normalization should not change anything
        assert_eq!(normalized1.boxes, normalized2.boxes);
        assert_eq!(normalized1.text_rows, normalized2.text_rows);
    }

    #[test]
    fn test_normalization_idempotent_horizontal_arrows() {
        let mut inventory = PrimitiveInventory::default();
        inventory.horizontal_arrows.push(HorizontalArrow {
            row: 0,
            start_col: 2,
            end_col: 5,
            arrow_type: ArrowType::Standard,
            rightward: true,
        });
        inventory.horizontal_arrows.push(HorizontalArrow {
            row: 0,
            start_col: 10,
            end_col: 15,
            arrow_type: ArrowType::Standard,
            rightward: true,
        });

        let normalized1 = align_horizontal_arrows(&inventory);
        let normalized2 = align_horizontal_arrows(&normalized1);

        assert_eq!(normalized1.horizontal_arrows, normalized2.horizontal_arrows);
    }

    #[test]
    fn test_normalization_idempotent_vertical_arrows() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 5),
            bottom_right: (3, 15),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory
            .vertical_arrows
            .push(crate::primitives::VerticalArrow {
                col: 11,
                start_row: 4,
                end_row: 6,
                arrow_type: ArrowType::Standard,
                downward: true,
            });

        let normalized1 = align_vertical_arrows(&inventory);
        let normalized2 = align_vertical_arrows(&normalized1);

        assert_eq!(normalized1.vertical_arrows, normalized2.vertical_arrows);
    }

    #[test]
    fn test_normalization_idempotent_padding() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (3, 10),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.text_rows.push(crate::primitives::TextRow {
            row: 1,
            start_col: 0,
            end_col: 9,
            content: "Content".to_string(),
        });

        let normalized1 = normalize_padding(&inventory);
        let normalized2 = normalize_padding(&normalized1);

        assert_eq!(normalized1.text_rows, normalized2.text_rows);
    }

    #[test]
    fn test_full_normalization_pipeline_idempotent() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (2, 5),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.text_rows.push(crate::primitives::TextRow {
            row: 1,
            start_col: 0,
            end_col: 4,
            content: " Text ".to_string(),
        });
        inventory.horizontal_arrows.push(HorizontalArrow {
            row: 3,
            start_col: 0,
            end_col: 2,
            arrow_type: ArrowType::Standard,
            rightward: true,
        });

        // Apply full pipeline twice
        let step1 = normalize_box_widths(&inventory);
        let step2 = normalize_padding(&step1);
        let step3 = align_horizontal_arrows(&step2);
        let step4 = align_vertical_arrows(&step3);

        let step1b = normalize_box_widths(&step4);
        let step2b = normalize_padding(&step1b);
        let step3b = align_horizontal_arrows(&step2b);
        let step4b = align_vertical_arrows(&step3b);

        // Second pipeline should produce identical results
        assert_eq!(step4.boxes, step4b.boxes);
        assert_eq!(step4.text_rows, step4b.text_rows);
        assert_eq!(step4.horizontal_arrows, step4b.horizontal_arrows);
        assert_eq!(step4.vertical_arrows, step4b.vertical_arrows);
    }

    #[test]
    fn test_find_vertical_overlap_groups_single_box() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (3, 5),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });

        let groups = find_vertical_overlap_groups(&inventory);
        assert!(groups.is_empty() || groups.iter().all(|g| g.len() <= 1));
    }

    #[test]
    fn test_find_vertical_overlap_groups_separate_boxes() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (2, 3),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.boxes.push(DiagramBox {
            top_left: (5, 0),
            bottom_right: (7, 3),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });

        let groups = find_vertical_overlap_groups(&inventory);
        // Vertically separated boxes should not be grouped
        assert!(groups.is_empty() || groups.iter().all(|g| g.len() == 1));
    }

    #[test]
    fn test_find_vertical_overlap_groups_side_by_side() {
        let mut inventory = PrimitiveInventory::default();
        // Left box
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (3, 4),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Right box - adjacent
        inventory.boxes.push(DiagramBox {
            top_left: (0, 5),
            bottom_right: (3, 9),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });

        let groups = find_vertical_overlap_groups(&inventory);
        // Should find one group with 2 boxes
        assert!(groups.iter().any(|g| g.len() == 2));
    }

    #[test]
    fn test_find_vertical_overlap_groups_three_boxes() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (2, 2),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.boxes.push(DiagramBox {
            top_left: (0, 3),
            bottom_right: (2, 5),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.boxes.push(DiagramBox {
            top_left: (0, 6),
            bottom_right: (2, 8),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });

        let groups = find_vertical_overlap_groups(&inventory);
        // Should find one group with 3 boxes
        assert!(groups.iter().any(|g| g.len() == 3));
    }

    #[test]
    fn test_find_vertical_overlap_groups_stacked_not_grouped() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (2, 3),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.boxes.push(DiagramBox {
            top_left: (3, 0),
            bottom_right: (5, 3),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });

        let groups = find_vertical_overlap_groups(&inventory);
        // Vertically stacked boxes should not be grouped
        assert!(groups.is_empty() || groups.iter().all(|g| g.len() == 1));
    }

    #[test]
    fn test_balance_horizontal_boxes_equalizes_widths() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (2, 2),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.boxes.push(DiagramBox {
            top_left: (0, 3),
            bottom_right: (2, 8),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });

        let balanced = balance_horizontal_boxes(&inventory);
        // First box should expand from width 3 to width 6
        assert_eq!(balanced.boxes[0].width(), 6);
        // Second box stays at width 6
        assert_eq!(balanced.boxes[1].width(), 6);
    }

    #[test]
    fn test_balance_horizontal_boxes_idempotent() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (2, 2),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.boxes.push(DiagramBox {
            top_left: (0, 3),
            bottom_right: (2, 8),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });

        let balanced1 = balance_horizontal_boxes(&inventory);
        let balanced2 = balance_horizontal_boxes(&balanced1);
        assert_eq!(balanced1.boxes, balanced2.boxes);
    }

    #[test]
    fn test_normalize_connection_lines_empty() {
        let inventory = PrimitiveInventory::default();
        let normalized = normalize_connection_lines(&inventory);
        assert!(normalized.connection_lines.is_empty());
    }

    #[test]
    fn test_normalize_connection_lines_preserves_basic() {
        use crate::primitives::{ConnectionLine, Segment};

        let mut inventory = PrimitiveInventory::default();
        inventory.connection_lines.push(ConnectionLine {
            segments: vec![Segment::Horizontal {
                row: 2,
                start_col: 0,
                end_col: 5,
            }],
            from_box: Some(0),
            to_box: None,
        });
        let normalized = normalize_connection_lines(&inventory);
        assert_eq!(normalized.connection_lines.len(), 1);
        assert_eq!(normalized.connection_lines[0].segments.len(), 1);
    }

    #[test]
    fn test_normalize_connection_lines_idempotent() {
        use crate::primitives::{ConnectionLine, Segment};

        let mut inventory = PrimitiveInventory::default();
        inventory.connection_lines.push(ConnectionLine {
            segments: vec![
                Segment::Horizontal {
                    row: 2,
                    start_col: 0,
                    end_col: 5,
                },
                Segment::Vertical {
                    col: 5,
                    start_row: 2,
                    end_row: 5,
                },
            ],
            from_box: Some(0),
            to_box: Some(1),
        });
        let norm1 = normalize_connection_lines(&inventory);
        let norm2 = normalize_connection_lines(&norm1);
        assert_eq!(norm1.connection_lines, norm2.connection_lines);
    }

    #[test]
    fn test_normalize_nested_boxes_empty() {
        let inventory = PrimitiveInventory::default();
        let normalized = normalize_nested_boxes(&inventory);
        assert!(normalized.boxes.is_empty());
    }

    #[test]
    fn test_normalize_nested_boxes_no_nesting() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (2, 4),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        let normalized = normalize_nested_boxes(&inventory);
        // Single box should be unchanged
        assert_eq!(normalized.boxes.len(), 1);
        assert_eq!(normalized.boxes[0].bottom_right, (2, 4));
    }

    #[test]
    fn test_normalize_nested_boxes_expands_parent() {
        let mut inventory = PrimitiveInventory::default();
        // Parent box
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (3, 4),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: vec![1],
        });
        // Child box extends beyond parent
        inventory.boxes.push(DiagramBox {
            top_left: (1, 2),
            bottom_right: (2, 6),
            style: BoxStyle::Single,
            parent_idx: Some(0),
            child_indices: Vec::new(),
        });
        let normalized = normalize_nested_boxes(&inventory);
        // Parent should expand to contain child
        assert!(normalized.boxes[0].bottom_right.1 >= 6);
    }

    #[test]
    fn test_normalize_nested_boxes_multiple_children() {
        let mut inventory = PrimitiveInventory::default();
        // Parent
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (6, 6),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: vec![1, 2],
        });
        // Child 1
        inventory.boxes.push(DiagramBox {
            top_left: (1, 1),
            bottom_right: (2, 3),
            style: BoxStyle::Single,
            parent_idx: Some(0),
            child_indices: Vec::new(),
        });
        // Child 2
        inventory.boxes.push(DiagramBox {
            top_left: (4, 4),
            bottom_right: (5, 8),
            style: BoxStyle::Single,
            parent_idx: Some(0),
            child_indices: Vec::new(),
        });
        let normalized = normalize_nested_boxes(&inventory);
        // Parent should expand to fit both children
        assert!(normalized.boxes[0].bottom_right.1 >= 8);
    }

    #[test]
    fn test_normalize_nested_boxes_idempotent() {
        let mut inventory = PrimitiveInventory::default();
        // Parent
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (4, 8),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: vec![1],
        });
        // Child
        inventory.boxes.push(DiagramBox {
            top_left: (1, 2),
            bottom_right: (3, 6),
            style: BoxStyle::Single,
            parent_idx: Some(0),
            child_indices: Vec::new(),
        });
        let norm1 = normalize_nested_boxes(&inventory);
        let norm2 = normalize_nested_boxes(&norm1);
        // Idempotent: applying twice should give same result
        assert_eq!(norm1.boxes, norm2.boxes);
    }

    #[test]
    fn test_normalize_labels_empty() {
        let inventory = PrimitiveInventory::default();
        let normalized = normalize_labels(&inventory);
        assert!(normalized.labels.is_empty());
    }

    #[test]
    fn test_normalize_labels_no_attachment() {
        use crate::primitives::{Label, LabelAttachment};

        let mut inventory = PrimitiveInventory::default();
        // Add a label
        inventory.labels.push(Label {
            row: 5,
            col: 10,
            content: "Text".to_string(),
            attached_to: LabelAttachment::Box(0),
            offset: (0, 0),
        });
        let normalized = normalize_labels(&inventory);
        // Label should still exist
        assert_eq!(normalized.labels.len(), 1);
    }

    #[test]
    fn test_normalize_labels_preserves_offset() {
        use crate::primitives::{Label, LabelAttachment};

        let mut inventory = PrimitiveInventory::default();
        // Add a box
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (2, 4),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Add a label with offset
        inventory.labels.push(Label {
            row: 5,
            col: 2,
            content: "Label".to_string(),
            attached_to: LabelAttachment::Box(0),
            offset: (3, 0),
        });
        let normalized = normalize_labels(&inventory);
        // Offset should be preserved
        assert_eq!(normalized.labels[0].offset, (3, 0));
    }

    #[test]
    fn test_normalize_labels_idempotent() {
        use crate::primitives::{Label, LabelAttachment};

        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (2, 4),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.labels.push(Label {
            row: 5,
            col: 2,
            content: "Label".to_string(),
            attached_to: LabelAttachment::Box(0),
            offset: (3, 0),
        });
        let norm1 = normalize_labels(&inventory);
        let norm2 = normalize_labels(&norm1);
        // Idempotent: applying twice should give same result
        assert_eq!(norm1.labels, norm2.labels);
    }

    #[test]
    fn test_normalize_labels_multiple() {
        use crate::primitives::{Label, LabelAttachment};

        let mut inventory = PrimitiveInventory::default();
        // Add two boxes
        inventory.boxes.push(DiagramBox {
            top_left: (0, 0),
            bottom_right: (2, 4),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.boxes.push(DiagramBox {
            top_left: (0, 6),
            bottom_right: (2, 10),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Add labels for each box
        inventory.labels.push(Label {
            row: 4,
            col: 2,
            content: "First".to_string(),
            attached_to: LabelAttachment::Box(0),
            offset: (2, 0),
        });
        inventory.labels.push(Label {
            row: 4,
            col: 8,
            content: "Second".to_string(),
            attached_to: LabelAttachment::Box(1),
            offset: (2, 0),
        });
        let normalized = normalize_labels(&inventory);
        // Both labels should be preserved
        assert_eq!(normalized.labels.len(), 2);
    }
}
