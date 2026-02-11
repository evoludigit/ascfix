# Regression: GitHub-Flavored Markdown
# Tests GFM-specific features like task lists, strikethrough, autolinks

# Task Lists
- [x] Implement basic box detection
- [x] Add arrow alignment
- [ ] Implement connection lines
- [ ] Add label placement
- [x] Support nested boxes

# Strikethrough text
~~This feature is deprecated~~
~~Multiple words can be struck through~~

# Autolinks
Visit https://github.com for more information.
Check out user/repo at github.com/user/repo.
Email support@example.com for help.

# Tables with GFM extensions
| Feature | Status | Notes |
|---------|--------|-------|
| Basic tables | ✅ Complete | Standard markdown |
| Alignment | ✅ Complete | `:---` and `---:` |
| Inline code | ✅ Complete | `code` in cells |
| Links | ✅ Complete | `[text](url)` in cells |
| **Bold** | ✅ Complete | `**text**` works |
| *Italic* | ✅ Complete | `*text*` works |
| ~~Strikethrough~~ | ✅ Complete | `~~text~~` works |
| Emojis | ✅ Complete | :rocket: :+1: |

# Code blocks with syntax highlighting
```rust
// Rust code with syntax highlighting
#[derive(Debug)]
struct User {
    name: String,
    email: String,
}

fn main() {
    let user = User {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    println!("{:?}", user);
}
```

# Footnotes
Here's a sentence with a footnote.[^1]

[^1]: This is the footnote content.

# Definition lists (if supported)
Term 1
: Definition 1

Term 2
: Definition 2
: Second definition for term 2