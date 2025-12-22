# Functional Iteration



## Beyond Loops

Iterators can replace imperative loops:

```rust
// Imperative (loop)
let mut sum = 0;
for i in 1..=5 {
    sum += i;
}

// Functional (iterator)
let sum: u64 = (1..=5).sum();
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Iterator methods replace loops:        │
│                                         │
│  • .sum() - add all elements            │
│  • .product() - multiply all elements   │
│  • .fold() - custom accumulation        │
│                                         │
│  More concise, often clearer intent!    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Range Iterators



## Creating Number Sequences

```rust
1..5    // 1, 2, 3, 4 (exclusive end)
1..=5   // 1, 2, 3, 4, 5 (inclusive end)
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Ranges are iterators!                  │
│                                         │
│  1..5   → [1, 2, 3, 4]                  │
│  1..=5  → [1, 2, 3, 4, 5]               │
│                                         │
│  The ..= syntax INCLUDES the end value. │
│  The .. syntax EXCLUDES it.             │
│                                         │
│  For factorial of n, want 1..=n         │
│  (1, 2, 3, ..., n)                      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The .product() Method



## Multiplying All Elements

```rust
let factorial_5: u64 = (1..=5).product();
// 1 * 2 * 3 * 4 * 5 = 120
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .product() multiplies all elements:    │
│                                         │
│  (1..=5).product()                      │
│  = 1 * 2 * 3 * 4 * 5                    │
│  = 120                                  │
│                                         │
│  Perfect for factorial!                 │
│                                         │
│  n! = 1 × 2 × 3 × ... × n               │
│     = (1..=n).product()                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The .fold() Method



## Custom Accumulation

```rust
let product = (1..=5).fold(1, |acc, x| acc * x);
// Start with 1, multiply each element
// 1 → 1*1=1 → 1*2=2 → 2*3=6 → 6*4=24 → 24*5=120
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .fold(initial, |accumulator, element|)│
│        ↑         ↑            ↑         │
│        │         │            └─ Current│
│        │         └─ Running total       │
│        └─ Starting value                │
│                                         │
│  Step through:                          │
│  acc=1, x=1: acc*x = 1                  │
│  acc=1, x=2: acc*x = 2                  │
│  acc=2, x=3: acc*x = 6                  │
│  acc=6, x=4: acc*x = 24                 │
│  acc=24, x=5: acc*x = 120               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Factorial



## Definition

```
n! = 1 × 2 × 3 × ... × n

0! = 1 (by definition)
1! = 1
2! = 2
3! = 6
4! = 24
5! = 120
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Special case: 0! = 1                   │
│                                         │
│  For 0, the range 1..=0 is EMPTY.       │
│  .product() of empty range = 1          │
│  (identity for multiplication)          │
│                                         │
│  So (1..=0).product() = 1 ✓             │
│                                         │
│  It handles the edge case automatically!│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Functional Factorial

```rust
fn factorial(num: u64) -> u64 {
    // Do not use:
    // - early returns (return keyword)
    // Try not to use:
    // - imperative loops (for/while)
    // - additional variables
    // For extra challenge:
    // - no recursion
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  CONSTRAINTS:                           │
│                                         │
│  ✗ No explicit return keyword           │
│  ✗ No for or while loops                │
│  ✗ No extra variables                   │
│  ✗ No recursion (extra challenge)       │
│                                         │
│  This forces FUNCTIONAL style!          │
│                                         │
│  Use iterator methods instead.          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Two Approaches



## Both Work

```
┌─────────────────────────────────────────┐
│                                         │
│  APPROACH 1: .product()                 │
│                                         │
│  (1..=num).product()                    │
│                                         │
│  Simple! Directly computes factorial.   │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  APPROACH 2: .fold()                    │
│                                         │
│  (1..=num).fold(1, |acc, x| acc * x)    │
│                                         │
│  More explicit about the operation.     │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  Both handle 0! correctly:              │
│  Empty range (1..=0) gives identity (1) │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Think It Through



## Before You Type



```
┌─────────────────────────────────────────┐
│  ASK YOURSELF:                          │
│                                         │
│  1. What is factorial mathematically?   │
│     (Product of 1 through n)            │
│                                         │
│  2. What range gives 1, 2, ..., n?      │
│     (1..=n - inclusive range)           │
│                                         │
│  3. What method multiplies all elements?│
│     (.product())                        │
│                                         │
│  4. What about factorial(0)?            │
│     (1..=0 is empty, product = 1 ✓)     │
│                                         │
│  5. What about factorial(1)?            │
│     (1..=1 is just [1], product = 1 ✓)  │
│                                         │
│  6. How do you write it in one line?    │
│     ((1..=num).product())               │
│                                         │
└─────────────────────────────────────────┘
```



One line: range and product!

(Now go to the Editor and try it!)
