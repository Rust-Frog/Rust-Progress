# Macro Definition Order



## Macros Must Come First

Unlike functions, macros must be defined BEFORE use!

```rust
// This FAILS:
fn main() {
    my_macro!();  // Error: macro not found!
}

macro_rules! my_macro {
    () => { println!("Hi"); };
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Functions:                             │
│  • Can be defined anywhere in file      │
│  • Rust finds them during compilation   │
│                                         │
│  Macros:                                │
│  • Must be defined BEFORE first use     │
│  • Rust expands them top-to-bottom      │
│                                         │
│  This is a key difference!              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Order Matters



## Top-Down Expansion

The compiler processes macros from top to bottom.

When it sees `my_macro!()`, it needs to know

what `my_macro` IS right then!

```rust
my_macro!();  // ← Compiler: "What is this?"
              //   "I haven't seen my_macro yet!"

macro_rules! my_macro { ... }
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Compiler reads top to bottom:          │
│                                         │
│  Line 1: my_macro!() - Unknown macro!   │
│          ↓                              │
│          ERROR before reaching Line 5   │
│                                         │
│  Line 5: macro_rules! my_macro          │
│          (Never gets here)              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Fix



## Move Definition Up

Put the macro definition BEFORE the call:

```rust
// Correct order:
macro_rules! my_macro {
    () => { println!("Hi"); };
}

fn main() {
    my_macro!();  // Works! Macro is known.
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Think of it like reading a book:       │
│                                         │
│  You can't understand a reference to    │
│  a character before they're introduced! │
│                                         │
│  Define first, use second.              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Code in Wrong Order

```rust
fn main() {
    my_macro!();
}

// TODO: Fix by moving this definition
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
```

The macro is defined AFTER it's used!



--- slide ---

# What You Need to Do



## Move the Macro Definition

```
┌─────────────────────────────────────────┐
│                                         │
│  Current order:                         │
│                                         │
│  1. fn main() { my_macro!(); }          │
│  2. macro_rules! my_macro { ... }       │
│                                         │
│  Correct order:                         │
│                                         │
│  1. macro_rules! my_macro { ... }       │
│  2. fn main() { my_macro!(); }          │
│                                         │
│  Move the ENTIRE macro_rules! block     │
│  to BEFORE main().                      │
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
│  1. What's the error telling you?       │
│     ("cannot find macro")               │
│                                         │
│  2. Where must a macro be defined       │
│     relative to its use?                │
│     (BEFORE it's used)                  │
│                                         │
│  3. What do you need to move, and       │
│     where?                              │
│     (Move macro_rules! block above main)│
│                                         │
│  Unlike functions, order matters!       │
│                                         │
└─────────────────────────────────────────┘
```



Define before use!

(Go try it in the Editor!)
