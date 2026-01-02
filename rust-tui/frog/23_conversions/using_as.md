# Type Casting with `as`



## The Simplest Conversion

The `as` keyword casts between primitive types:

```rust
let x: i32 = 5;
let y: f64 = x as f64;  // i32 → f64

let a: f64 = 3.14;
let b: i32 = a as i32;  // f64 → i32 (truncates!)
```



```
┌─────────────────────────────────────────┐
│                                         │
│  `as` does simple conversions:          │
│                                         │
│  • Integer to integer (i32 → i64)       │
│  • Float to integer (truncates!)        │
│  • Integer to float                     │
│  • Pointer casts                        │
│                                         │
│  Syntax: value as target_type           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Type Mismatches



## Rust Is Strict About Types

Rust won't automatically convert types:

```rust
let total: f64 = 10.0;
let count: usize = 4;

let avg = total / count;  // ERROR!
//              ↑
//        f64 and usize can't divide!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Many languages auto-convert:           │
│  10.0 / 4 → 2.5 (automatic!)            │
│                                         │
│  Rust does NOT:                         │
│  • Types must match exactly             │
│  • You must be explicit about casts     │
│  • This prevents subtle bugs!           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Fixing Type Mismatches



## Cast One Side

```rust
let total: f64 = 10.0;
let count: usize = 4;

// Option 1: Cast count to f64
let avg = total / count as f64;
//                ↑↑↑↑↑↑↑↑↑↑↑
//                4 becomes 4.0

// Option 2: Cast total to usize (loses precision!)
let avg = total as usize / count;
//        ↑↑↑↑↑↑↑↑↑↑↑↑↑↑
//        10.0 becomes 10
```



```
┌─────────────────────────────────────────┐
│                                         │
│  For division, prefer casting TO float: │
│                                         │
│  10.0 / 4.0 = 2.5 ✓                     │
│  10 / 4 = 2 (integer division!)         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Calculate an Average

```rust
fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // TODO: Make a conversion before dividing
    total / values.len()
}
```

`total` is `f64`, but `values.len()` is `usize`!



--- slide ---

# What You Need to Do



## Cast the Length

```
┌─────────────────────────────────────────┐
│                                         │
│  Problem:                               │
│                                         │
│  total       : f64                      │
│  values.len(): usize                    │
│                                         │
│  You can't divide f64 by usize!         │
│                                         │
│  Solution:                              │
│                                         │
│  total / values.len() as f64            │
│                       ↑↑↑↑↑↑            │
│         Cast usize to f64               │
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
│  1. What type is `total`?               │
│     (f64 - a float)                     │
│                                         │
│  2. What type is `values.len()`?        │
│     (usize - an integer)                │
│                                         │
│  3. What keyword converts types?        │
│     (as)                                │
│                                         │
│  4. Which side should you cast?         │
│     (The length, to preserve precision) │
│                                         │
└─────────────────────────────────────────┘
```



Use `as` to bridge type mismatches!

(Go try it in the Editor!)
