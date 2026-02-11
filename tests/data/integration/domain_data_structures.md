# Domain: Data Structures - Binary Tree
# Tests complex tree structures common in computer science documentation

                    ┌─────────────┐
                    │     Root    │
                    │   (Value)   │
                    └─────┬───────┘
                          │
                ┌─────────┴─────────┐
                │                   │
          ┌─────▼────┐       ┌─────▼────┐
          │   Left    │       │   Right   │
          │  Child    │       │  Child    │
          └─────┬────┘       └─────┬────┘
                │                   │
          ┌─────▼────┐       ┌─────▼────┐
          │  Left     │       │  Right    │
          │  Leaf     │       │  Leaf     │
          └───────────┘       └───────────┘

# Tree traversal algorithms table
| Traversal | Order | Example | Use Case |
|-----------|-------|---------|----------|
| Inorder | Left, Root, Right | A, B, C | BST property verification |
| Preorder | Root, Left, Right | B, A, C | Copy tree structure |
| Postorder | Left, Right, Root | A, C, B | Delete tree nodes |
| Level-order | Level by level | B, A, C | Breadth-first search |

# Binary search tree properties
```
Valid BST Properties:
- Left subtree < root < right subtree
- No duplicate values allowed
- Inorder traversal gives sorted order
- Height can be O(log n) to O(n)

Operations: Insert, Delete, Search - all O(h) time
```

# Hash table collision resolution
┌─────────────────┐    ┌─────────────────┐
│   Hash Table    │    │ Collision      │
│                 │    │ Resolution     │
│ Key ──► Hash ──►│    │                 │
│ Value           │    │ Chaining       │
└─────────────────┘    │ Open           │
                       │ Addressing     │
                       └─────────────────┘