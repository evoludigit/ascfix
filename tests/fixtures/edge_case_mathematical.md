# Edge Case: Mathematical Expressions
# Tests handling of mathematical notation in ASCII diagrams

# Matrix representation
┌                     ┐
│  1  2  3  4  5  6  │
│  7  8  9 10 11 12  │
│ 13 14 15 16 17 18  │
│ 19 20 21 22 23 24  │
└                     ┘

# Set notation and operations
Universal Set U = {1, 2, 3, 4, 5, 6}
Subset A = {2, 4, 6} (even numbers)
Subset B = {1, 3, 5} (odd numbers)

A ∪ B = {1, 2, 3, 4, 5, 6}  # Union
A ∩ B = ∅                    # Intersection (empty set)
A - B = {2, 4, 6}           # Difference
Â = {1, 3, 5}               # Complement

# Algorithm complexity
┌─────────────────────────────────────┐
│        Big O Notation               │
├─────────────────────────────────────┤
│ O(1)     - Constant time            │
│ O(log n) - Logarithmic time         │
│ O(n)     - Linear time              │
│ O(n log n) - Linearithmic time      │
│ O(n²)    - Quadratic time           │
│ O(2ⁿ)    - Exponential time         │
│ O(n!)    - Factorial time           │
└─────────────────────────────────────┘

# Graph theory
Undirected Graph G = (V, E)
V = {A, B, C, D, E}  # Vertices
E = {(A,B), (B,C), (C,D), (D,E), (E,A)}  # Edges

    A ─── B
    │     │
    │     │
    E ─── C ─── D

# Probability and statistics
P(A) = 0.6              # Probability of event A
P(B|A) = 0.8            # Conditional probability
P(A∩B) = P(A) × P(B|A)  # Joint probability
E[X] = Σ(x × P(x))      # Expected value
Var(X) = E[X²] - E[X]²  # Variance
σ = √Var(X)            # Standard deviation

# Calculus notation
∫ f(x) dx                # Indefinite integral
∫₀^∞ e^(-x²) dx = √π/2   # Definite integral
d/dx f(x) = f'(x)        # Derivative
∂f/∂x                    # Partial derivative
∇f = (∂f/∂x, ∂f/∂y)     # Gradient vector