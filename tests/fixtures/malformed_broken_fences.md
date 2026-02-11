# Malformed: Broken Code Fences
# Tests code blocks with fence issues

```python
code block
```  # Normal fence

```python
code
``  # Mismatched fence length

```python
code
````  # Too many closing ticks

~~~python
code
```  # Wrong fence type

   ```python
   code
   ```  # Indented fence

```
No language
```
```  # Empty opening fence

```python
code
  # Missing closing fence entirely