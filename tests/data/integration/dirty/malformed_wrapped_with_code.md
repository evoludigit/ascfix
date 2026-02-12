# Malformed: Wrapped Tables with Code
# Tests that code blocks in table cells are preserved (not unwrapped)

## Table with code block in cell
| Language | Example | Notes |
|----------|---------|-------|
| Python | ```python | Simple function |
|        | def hello(): | |
|        |     print("Hello") | |
|        | ``` | |
| Rust | ```rust | Ownership example |
|      | fn main() { | |
|      |     let s = String::from("hi"); | |
|      | } | |
|      | ``` | |

## Table with inline code and wrapped text
| Function | Description |
|----------|-------------|
| `map()` | Applies a function to each element in a collection, |
|         | transforming values and returning a new collection |
| `filter()` | Selects elements matching a predicate function, |
|            | creating a subset of the original collection |

## Multiple code blocks
| Component | Setup | Usage |
|-----------|-------|-------|
| Database | ```sql | ```python |
|          | CREATE TABLE users ( | db.connect() |
|          |   id INT PRIMARY KEY | db.query("SELECT *") |
|          | ); | ``` |
|          | ``` | |
