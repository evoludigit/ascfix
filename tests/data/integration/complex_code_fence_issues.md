# Complex: Code Fence Issues
# Tests various code fence boundary problems

# Mismatched fence lengths
```javascript
function hello() {
  console.log("Hello, World!");
}
`````

# Unclosed fence
```python
def hello():
    print("Hello, World!")
# No closing fence

# Nested code blocks with different fence types
~~~markdown
# This is markdown inside tildes
```
code inside backticks
```
~~~

# Fence with language specifier but wrong length
```javascript
const hello = () => {
  console.log("Hello!");
};
````

# Multiple consecutive fences
```
First code block
```
```
Second code block
```
```
Third code block
```

# Fence with special characters in language
```text/x-shellscript
#!/bin/bash
echo "Hello World"
```

# Very long fence markers
````````````````````````````````````````````````
This is code with very long fences
````````````````````````````````````````````````

# Mixed fence types in same document
```javascript
console.log("backticks");
```
~~~python
print("tildes")
~~~
```
No language
```