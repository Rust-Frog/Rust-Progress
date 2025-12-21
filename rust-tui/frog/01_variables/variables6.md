# Constants: Carved in Stone

A constant is like carving something in granite.
It can NEVER change. Not even with `mut`.

```
     ┌───────────────────┐
     │        3          │
     └───────────────────┘
           NUMBER
      (etched in granite)
```

Once carved, it stays that way forever.
Every time you look at it, same value.

```rust
const MAX_SCORE: i32 = 100;
```

This isn't just a box with super glue.
This is a PERMANENT FACT about your program.

--- slide ---

# Constants vs Variables

```
┌─────────────────┬────────────────┬───────────────┐
│                 │ Variable       │ Constant      │
├─────────────────┼────────────────┼───────────────┤
│ Keyword         │ let            │ const         │
│ Can change?     │ With mut: yes  │ NEVER         │
│ Type required?  │ Optional       │ REQUIRED      │
│ Naming style    │ snake_case     │ SCREAMING     │
│ When set?       │ When program   │ Before prog   │
│                 │ runs           │ even starts   │
└─────────────────┴────────────────┴───────────────┘
```

## Examples

```rust
let x = 5;                  // Variable (snake_case)
const MAX_SCORE: i32 = 100; // Constant (SCREAMING)
```

Notice: Constants use ALL_CAPS and REQUIRE the type.

--- slide ---

# Why SCREAMING_CASE?

So you can spot constants INSTANTLY in code:

```rust
let score = calculate_score();

if score > MAX_SCORE {
            ^^^^^^^^^
            "Oh, that's a constant! It never changes."
    println!("You beat the max score!");
}
```

When you see ALL_CAPS, you know:
- This NEVER changes, ever
- It's a fixed value, not calculated
- It's probably defined at the top of the file
- It's a fundamental fact about the program

This is super helpful when reading unfamiliar code.

--- slide ---

# Constants MUST Have Types

Variables can figure out their own type:
```rust
let x = 5;  // Rust infers i32
```

Constants CANNOT:
```rust
const NUMBER = 3;      // ERROR! What type?
const NUMBER: i32 = 3; // GOOD! It's an i32
             ^^^^
             REQUIRED!
```

## Why the strict rule?

Constants are evaluated BEFORE your program runs.
Rust needs to know EXACTLY what they are upfront.

Think of it like construction blueprints.
You need exact specifications before building starts.

--- slide ---

# Your Exercise

The code has:

```rust
const NUMBER = 3;
```

Missing the type! Rust is confused:

```
"Is this an i32? An i64? A u8? A f64?
 I need to know EXACTLY!"
```

Different number types have different:
- Memory size
- Maximum values
- What operations they support

Rust needs this info upfront for constants.

## The Fix

Add the type after the name:

```rust
const NUMBER: i32 = 3;
             ^^^^
             Add this!
```

--- slide ---

# When to Use Constants

Constants are for things that are TRULY fixed:

```rust
// Mathematical constants
const PI: f64 = 3.14159265359;

// Game settings
const MAX_PLAYERS: u32 = 4;
const STARTING_LIVES: u32 = 3;

// App info
const VERSION: &str = "1.0.0";

// Physical constants
const GRAVITY: f64 = 9.81;
```

If a value might change later (like a user setting), 
use a variable instead.

If it's a law of physics, a hard limit, or a fixed 
part of your program's design → constant.
