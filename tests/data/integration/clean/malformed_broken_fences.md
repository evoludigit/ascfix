# Malformed: Broken Code Fences
# Tests code blocks with fence issues

Proper Python code block:
```python
code block
```

Proper Rust code block:
```rust
let x = 42;
```

Proper generic code block:
```
No language specified
```

Multiple code blocks:
```python
def hello():
    print("Hello")
```

```javascript
function hello() {
  console.log("Hello");
}
```

Mixed with diagrams:
```
┌────────────┐
│ Code in    │
│ fence      │
└────────────┘
```
