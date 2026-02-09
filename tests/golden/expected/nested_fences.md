# Nested Code Fences

This demonstrates nested fences that should be left unchanged (they're intentional).

## Documentation with code examples

`````markdown
# API Reference

```python
def api():
    pass
```

Here's another example:

```javascript
function api() {}
```
`````

## Another Example

````yaml
config:
  items:
    - name: test
      code: |
        ```bash
        echo hello
        ```
````

That's all!
