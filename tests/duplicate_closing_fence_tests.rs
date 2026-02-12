//! Tests for duplicate closing fence marker detection and repair.
//!
//! Issue #4: Currently ascfix does not detect or fix duplicate closing fence markers.
//! These tests verify that the issue is fixed.

#[test]
fn test_duplicate_closing_fence_basic() {
    let input = r"Simple example with duplicate closing fence:

```python
def example():
    pass
```
```

After the code.";

    let expected = r"Simple example with duplicate closing fence:

```python
def example():
    pass
```

After the code.";

    // Get the ascfix library - we'll use the modes directly
    let processed = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        input,
        true,
        &ascfix::config::Config::default(),
    );

    assert_eq!(
        processed, expected,
        "Duplicate closing fence should be removed.\nExpected:\n{expected}\n\nGot:\n{processed}",
    );
}

#[test]
fn test_duplicate_closing_fence_multiple_blocks() {
    let input = r#"# Multiple code blocks with duplicates

First block:

```javascript
function hello() {
  console.log("world");
}
```
```

Some text between blocks.

Second block:

```rust
fn main() {
    println!("hello");
}
```
```

End of document."#;

    let expected = r#"# Multiple code blocks with duplicates

First block:

```javascript
function hello() {
  console.log("world");
}
```

Some text between blocks.

Second block:

```rust
fn main() {
    println!("hello");
}
```

End of document."#;

    let processed = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        input,
        true,
        &ascfix::config::Config::default(),
    );

    assert_eq!(
        processed, expected,
        "All duplicate closing fences should be removed.\nExpected:\n{expected}\n\nGot:\n{processed}",
    );
}

#[test]
fn test_duplicate_closing_fence_consecutive() {
    let input = r#"Code with multiple duplicate closing fences:

```bash
#!/bin/bash
echo "test"
```
```
```

This should have all duplicates removed."#;

    let expected = r#"Code with multiple duplicate closing fences:

```bash
#!/bin/bash
echo "test"
```

This should have all duplicates removed."#;

    let processed = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        input,
        true,
        &ascfix::config::Config::default(),
    );

    assert_eq!(
        processed, expected,
        "Consecutive duplicate closing fences should be removed.\nExpected:\n{expected}\n\nGot:\n{processed}",
    );
}

#[test]
fn test_duplicate_closing_fence_with_indentation() {
    let input = r"Code with indented duplicate fence:

```python
def test():
    return True
```
  ```

After code.";

    let expected = r"Code with indented duplicate fence:

```python
def test():
    return True
```

After code.";

    let processed = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        input,
        true,
        &ascfix::config::Config::default(),
    );

    assert_eq!(
        processed, expected,
        "Indented duplicate closing fence should be removed.\nExpected:\n{expected}\n\nGot:\n{processed}",
    );
}

#[test]
fn test_duplicate_closing_fence_tildes() {
    let input = r"Code block with tildes:

~~~python
def example():
    pass
~~~
~~~

After.";

    let expected = r"Code block with tildes:

~~~python
def example():
    pass
~~~

After.";

    let processed = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        input,
        true,
        &ascfix::config::Config::default(),
    );

    assert_eq!(
        processed, expected,
        "Duplicate closing fence with tildes should be removed.\nExpected:\n{expected}\n\nGot:\n{processed}",
    );
}

#[test]
fn test_no_duplicate_closing_fence_normal_code_blocks() {
    // Verify we don't break normal code blocks
    let input = r"Normal code block:

```python
def example():
    pass
```

After code.";

    let processed = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        input,
        true,
        &ascfix::config::Config::default(),
    );

    assert_eq!(
        processed, input,
        "Normal code blocks without duplicates should not be modified.\nGot:\n{processed}",
    );
}

#[test]
fn test_no_duplicate_closing_fence_nested_code_blocks() {
    // Verify we don't break nested/longer fences
    let input = r"Longer fence for nesting:

`````python
```
code here
```
`````

After code.";

    let processed = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        input,
        true,
        &ascfix::config::Config::default(),
    );

    assert_eq!(
        processed, input,
        "Nested code blocks with longer fences should not be modified.\nGot:\n{processed}",
    );
}
