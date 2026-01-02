# Clippy: Your Helpful Linting Friend



## What Is Clippy?

Clippy is a tool that catches common mistakes

and suggests better ways to write Rust code.



```
┌─────────────────────────────────────────┐
│                                         │
│  🔍 Clippy finds:                       │
│                                         │
│  • Common mistakes                      │
│  • Suboptimal code patterns             │
│  • Stylistic issues                     │
│  • Potential bugs                       │
│                                         │
│  It's like having an expert Rustacean   │
│  reviewing your code!                   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# How Clippy Works



## Lints = Suggestions

Clippy provides "lints" - warnings about your code:

```
warning: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:14
  |
2 |     let pi = 3.14;
  |              ^^^^
  |
  = help: consider using the constant directly
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Clippy tells you:                      │
│                                         │
│  1. WHAT it found (approximate PI)      │
│  2. WHERE it is (line 2, column 14)     │
│  3. HOW to fix it (use the constant)    │
│                                         │
│  Always read the suggestion!            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Mathematical Constants



## The Standard Library Has Them

Rust provides precise mathematical constants:

```rust
use std::f32::consts::PI;
use std::f64::consts::PI;  // For f64

// Also available:
// E, FRAC_PI_2, SQRT_2, and more!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Why use std constants?                 │
│                                         │
│  3.14       = 3.14 (only 2 decimals!)   │
│  3.14159    = 3.14159 (5 decimals)      │
│  PI constant = 3.14159265358979...      │
│                                         │
│  The constant is as precise as the      │
│  floating-point type allows!            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Using Approximate PI

```rust
fn main() {
    let pi = 3.14;  // ← Clippy doesn't like this!
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("Area: {area:.5}");
}
```

Clippy says: "Use the constant directly!"



--- slide ---

# The Problem



## Manual PI Is Imprecise

```
┌─────────────────────────────────────────┐
│                                         │
│  Your code:  let pi = 3.14;             │
│                                         │
│  This is APPROXIMATE - you're losing    │
│  precision for no good reason!          │
│                                         │
│  Real PI: 3.14159265358979323846...     │
│  Your PI: 3.14                          │
│           ↑                             │
│           Missing digits!               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Fix



## Use std::f32::consts::PI

```rust
use std::f32::consts::PI;

fn main() {
    let radius: f32 = 5.0;

    let area = PI * radius.powi(2);
    //         ↑ Use the constant!

    println!("Area: {area:.5}");
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Two ways to use it:                    │
│                                         │
│  1. Import: use std::f32::consts::PI;   │
│     Then use: PI                        │
│                                         │
│  2. Full path: std::f32::consts::PI     │
│     No import needed                    │
│                                         │
│  Note: f32 for f32 type, f64 for f64!   │
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
│  1. What module contains PI?            │
│     (std::f32::consts or std::f64::consts)│
│                                         │
│  2. What type is radius?                │
│     (f32, so use f32's PI)              │
│                                         │
│  3. Do you need the variable `pi`?      │
│     (No! Just use PI directly)          │
│                                         │
│  4. What import do you need?            │
│     (use std::f32::consts::PI;)         │
│                                         │
└─────────────────────────────────────────┘
```



Use standard library constants for precision!

(Go try it in the Editor!)
