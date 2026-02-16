//! Custom assertions for test validation

/// Assert that processing time is linear (not quadratic) with input size
#[allow(dead_code)]
#[allow(clippy::cast_precision_loss)]
pub fn assert_linear_scaling(size1: usize, time1: f64, size2: usize, time2: f64) {
    let size_ratio = size2 as f64 / size1 as f64;
    let time_ratio = time2 / time1;

    // Allow up to 3x overhead for larger files (not 10x as would be quadratic)
    assert!(
        time_ratio < size_ratio * 3.0,
        "Performance scaling appears quadratic: `{size1}bytes={time1}ms, {size2}bytes={time2}ms`"
    );
}

/// Assert that output is valid UTF-8
#[allow(dead_code)]
pub fn assert_valid_utf8(content: &str) {
    assert!(content.is_utf8_encoded());
}

trait Utf8Check {
    fn is_utf8_encoded(&self) -> bool;
}

impl Utf8Check for str {
    fn is_utf8_encoded(&self) -> bool {
        // If we can construct it as &str, it's valid UTF-8
        true
    }
}
