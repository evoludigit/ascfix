# Malformed: Broken Tables
# Tests tables with structural issues

| Header 1 | Header 2 |
|----------|----------|
| Cell 1 | Cell 2 |
  # Missing closing pipe

Table with inconsistent columns:
| A | B | C |
|----|----|
| 1 | 2 |  # Missing third cell
| X | Y | Z |

Table with malformed separators:
| Header |
|=======|  # Wrong separator characters
| Data |

Table with empty cells causing confusion:
| A | B |
|---|---|
|   |   |  # Completely empty row
| 1 | 2 |

Table with mixed pipe usage:
| A | B |
| C | D   # Missing closing pipe
| E | F |