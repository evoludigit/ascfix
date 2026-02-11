//! Diagram element detection.

// Re-export for backward compatibility
pub use self::arrows::{detect_horizontal_arrows, detect_vertical_arrows};
pub use self::boxes::detect_boxes;

mod arrows;
mod boxes;

/// Unified detector that returns all primitives in a diagram.
///
/// This is the main entry point for diagram analysis. It orchestrates
/// detection of all primitive types and returns a complete inventory.
#[allow(dead_code)] // Reason: Used by main processing pipeline
#[must_use]
pub fn detect_all_primitives(grid: &crate::grid::Grid) -> crate::primitives::PrimitiveInventory {
    let boxes = detect_boxes(grid);
    let horizontal_arrows = detect_horizontal_arrows(grid);
    let vertical_arrows = detect_vertical_arrows(grid);

    // Create temporary inventory for label detection (needs boxes and arrows)
    let temp_inventory = crate::primitives::PrimitiveInventory {
        boxes: boxes.clone(),
        horizontal_arrows: horizontal_arrows.clone(),
        vertical_arrows: vertical_arrows.clone(),
        text_rows: Vec::new(),
        connection_lines: Vec::new(),
        labels: Vec::new(),
    };
    let labels = detect_labels(grid, &temp_inventory);

    // Extract text rows from inside boxes
    let mut text_rows = Vec::new();

    // Check if we have nested boxes (complex case)
    let has_nested_boxes = boxes.len() > 1
        && boxes.iter().any(|b1| {
            boxes.iter().any(|b2| {
                b1 != b2
                    && b1.top_left.0 < b2.top_left.0
                    && b1.bottom_right.0 > b2.bottom_right.0
                    && b1.top_left.1 < b2.top_left.1
                    && b1.bottom_right.1 > b2.bottom_right.1
            })
        });

    if has_nested_boxes {
        // Conservative approach for nested boxes: skip text extraction entirely
        // to avoid corrupting content. Complex nested layouts need manual handling.
        // This preserves the original content structure.
    } else {
        // Simple case: no nested boxes, use original logic
        for b in &boxes {
            for (line_idx, line) in extract_box_content(grid, b).iter().enumerate() {
                if !line.trim().is_empty() {
                    let interior_row = b.top_left.0 + 1 + line_idx;
                    // Clean the content by removing trailing border characters
                    let clean_content = line.trim_end_matches(|c| ['║', '│', '┃'].contains(&c));
                    text_rows.push(crate::primitives::TextRow {
                        row: interior_row,
                        start_col: b.top_left.1 + 1,
                        end_col: b.bottom_right.1 - 1,
                        content: clean_content.to_string(),
                    });
                }
            }
        }
    }

    crate::primitives::PrimitiveInventory {
        boxes,
        horizontal_arrows,
        vertical_arrows,
        text_rows,
        connection_lines: Vec::new(),
        labels,
    }
}

/// Extract text rows from inside a box.
///
/// Returns the content of interior rows between the top and bottom borders.
#[allow(dead_code)] // Reason: Used by main processing pipeline
#[must_use]
pub fn extract_box_content(grid: &crate::grid::Grid, b: &crate::primitives::Box) -> Vec<String> {
    let mut content = Vec::new();

    for row in (b.top_left.0 + 1)..b.bottom_right.0 {
        let mut line = String::new();
        for col in (b.top_left.1 + 1)..b.bottom_right.1 {
            if let Some(ch) = grid.get(row, col) {
                line.push(ch);
            }
        }
        content.push(line);
    }

    content
}

/// Box character set for detection.
const fn is_box_char(ch: char) -> bool {
    matches!(
        ch,
        '─' | '│'
            | '┌'
            | '┐'
            | '└'
            | '┘'
            | '├'
            | '┤'
            | '┼'
            | '┬'
            | '┴'
            | '┃'
            | '═'
            | '║'
            | '╔'
            | '╗'
            | '╚'
            | '╝'
            | '╭'
            | '╮'
            | '╰'
            | '╯'
    )
}

/// Detect labels (text near primitives with attachment tracking).
///
/// Algorithm:
/// 1. For each text row, find nearest primitive
/// 2. If within distance threshold (2 cells), mark as label
/// 3. Calculate offset from primitive center/edge
/// 4. Skip if ambiguous (equidistant from multiple primitives)
///
/// Conservative: Requires clear proximity and avoids ambiguous cases.
#[allow(dead_code)] // Reason: Used by main processing pipeline
#[allow(clippy::missing_const_for_fn)] // Future implementation will need mutable references
#[must_use]
pub fn detect_labels(
    grid: &crate::grid::Grid,
    inventory: &crate::primitives::PrimitiveInventory,
) -> Vec<crate::primitives::Label> {
    let mut labels = Vec::new();

    // Get all occupied positions (from boxes, arrows, text rows)
    let mut occupied_positions = std::collections::HashSet::new();

    // Mark box positions as occupied
    for b in &inventory.boxes {
        for row in b.top_left.0..=b.bottom_right.0 {
            for col in b.top_left.1..=b.bottom_right.1 {
                occupied_positions.insert((row, col));
            }
        }
    }

    // Mark arrow positions as occupied
    for arrow in &inventory.horizontal_arrows {
        for col in arrow.start_col..=arrow.end_col {
            occupied_positions.insert((arrow.row, col));
        }
    }

    for arrow in &inventory.vertical_arrows {
        for row in arrow.start_row..=arrow.end_row {
            occupied_positions.insert((row, arrow.col));
        }
    }

    // Mark text row positions as occupied
    for text_row in &inventory.text_rows {
        for i in 0..text_row.content.len() {
            let col = text_row.start_col + i;
            if col <= text_row.end_col {
                occupied_positions.insert((text_row.row, col));
            }
        }
    }

    // Scan the grid for potential label text
    // Labels are isolated text segments near primitives
    for row in 0..grid.height() {
        let mut current_label_start = None;
        let mut current_label_text = String::new();

        for col in 0..grid.width() {
            if let Some(ch) = grid.get(row, col) {
                if !occupied_positions.contains(&(row, col)) && !is_box_char(ch) && ch != ' ' {
                    // This is potential label text
                    if current_label_start.is_none() {
                        current_label_start = Some(col);
                    }
                    current_label_text.push(ch);
                } else {
                    // End of potential label
                    if let Some(start_col) = current_label_start {
                        if !current_label_text.is_empty() && current_label_text.len() > 1 {
                            // Check if this label is near a primitive
                            if let Some(attachment) =
                                find_nearest_primitive(row, start_col, inventory)
                            {
                                labels.push(crate::primitives::Label {
                                    row,
                                    col: start_col,
                                    content: current_label_text.clone(),
                                    attached_to: attachment,
                                    offset: calculate_offset(
                                        row,
                                        start_col,
                                        &attachment,
                                        inventory,
                                    ),
                                });
                            }
                        }
                    }
                    current_label_start = None;
                    current_label_text.clear();
                }
            }
        }

        // Handle label at end of row
        if let Some(start_col) = current_label_start {
            if !current_label_text.is_empty() && current_label_text.len() > 1 {
                if let Some(attachment) = find_nearest_primitive(row, start_col, inventory) {
                    labels.push(crate::primitives::Label {
                        row,
                        col: start_col,
                        content: current_label_text,
                        attached_to: attachment,
                        offset: calculate_offset(row, start_col, &attachment, inventory),
                    });
                }
            }
        }
    }

    labels
}

/// Find the nearest primitive to a label position
fn find_nearest_primitive(
    row: usize,
    col: usize,
    inventory: &crate::primitives::PrimitiveInventory,
) -> Option<crate::primitives::LabelAttachment> {
    let mut nearest: Option<(crate::primitives::LabelAttachment, usize)> = None;

    // Check boxes
    for (idx, b) in inventory.boxes.iter().enumerate() {
        let distances = [
            // Distance to top edge
            if row < b.top_left.0 {
                b.top_left.0 - row
            } else {
                row.saturating_sub(b.bottom_right.0)
            },
            // Distance to bottom edge
            if row > b.bottom_right.0 {
                row - b.bottom_right.0
            } else {
                b.top_left.0.saturating_sub(row)
            },
            // Distance to left edge
            if col < b.top_left.1 {
                b.top_left.1 - col
            } else {
                col.saturating_sub(b.bottom_right.1)
            },
            // Distance to right edge
            if col > b.bottom_right.1 {
                col - b.bottom_right.1
            } else {
                b.top_left.1.saturating_sub(col)
            },
        ];

        let min_distance = distances.iter().min().unwrap();
        if *min_distance <= 2 {
            if let Some((_, current_min)) = nearest {
                if *min_distance < current_min {
                    nearest = Some((crate::primitives::LabelAttachment::Box(idx), *min_distance));
                }
            } else {
                nearest = Some((crate::primitives::LabelAttachment::Box(idx), *min_distance));
            }
        }
    }

    // Check vertical arrows (give them higher priority than boxes when close)
    for (idx, arrow) in inventory.vertical_arrows.iter().enumerate() {
        // For vertical arrows, check distance to the arrow column
        let col_distance = arrow.col.abs_diff(col);

        // Check if the label is reasonably close to the arrow
        // Allow more vertical distance but require close column proximity
        if col_distance <= 4 {
            // For arrows, we prefer them over boxes when they're reasonably close
            // This helps with cases where labels are below arrows but within box boundaries
            let arrow_priority = col_distance; // Lower is better

            if let Some((current_attachment, current_min)) = nearest {
                // If current is a box and arrow is reasonably close, prefer arrow
                // This handles the case where labels are below arrows but technically "inside" boxes
                let should_prefer_arrow = match current_attachment {
                    crate::primitives::LabelAttachment::Box(_) => {
                        // For labels that are inside boxes, prefer arrows if they're reasonably close
                        arrow_priority <= 3 // Prefer arrows within 3 columns of the label
                    }
                    _ => arrow_priority < current_min,
                };

                if should_prefer_arrow {
                    nearest = Some((
                        crate::primitives::LabelAttachment::VerticalArrow(idx),
                        arrow_priority,
                    ));
                }
            } else {
                nearest = Some((
                    crate::primitives::LabelAttachment::VerticalArrow(idx),
                    arrow_priority,
                ));
            }
        }
    }

    nearest.map(|(attachment, _)| attachment)
}

/// Calculate offset from attachment point to label position
#[allow(clippy::cast_possible_wrap)] // usize to isize may wrap on large values, but diagrams are small
fn calculate_offset(
    row: usize,
    col: usize,
    attachment: &crate::primitives::LabelAttachment,
    inventory: &crate::primitives::PrimitiveInventory,
) -> (isize, isize) {
    match attachment {
        crate::primitives::LabelAttachment::Box(idx) => {
            inventory.boxes.get(*idx).map_or((0, 0), |b| {
                let center_row = usize::midpoint(b.top_left.0, b.bottom_right.0);
                let center_col = usize::midpoint(b.top_left.1, b.bottom_right.1);
                (
                    row as isize - center_row as isize,
                    col as isize - center_col as isize,
                )
            })
        }
        _ => (0, 0), // For now
    }
}
