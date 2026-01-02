# Multiple Macro Patterns



## Macros with Different Inputs

Macros can match different patterns!

Like function overloading, but for macros:

```rust
macro_rules! my_macro {
    () => {
        println!("No arguments");
    };
    ($val:expr) => {
        println!("One argument: {}", $val);
    };
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  my_macro!()     → "No arguments"       │
│  my_macro!(42)   → "One argument: 42"   │
│                                         │
│  Each pattern is called an "arm"        │
│  or "branch" of the macro.              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Pattern Syntax



## Matching Expressions

`$val:expr` captures an expression:

```rust
macro_rules! my_macro {
    ($val:expr) => {
        println!("Got: {}", $val);
    };
}

my_macro!(1 + 2);    // Got: 3
my_macro!("hello");  // Got: hello
```



```
┌─────────────────────────────────────────┐
│                                         │
│  $val:expr means:                       │
│                                         │
│  $val = name to capture into            │
│  :expr = must be an expression          │
│                                         │
│  Other types:                           │
│  :ident = identifier (variable name)    │
│  :ty = type                             │
│  :tt = token tree (anything)            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Semicolon Rule



## Arms Need Semicolons

Each macro arm must end with a semicolon:

```rust
macro_rules! my_macro {
    () => {
        println!("First");
    };   // ← Semicolon!
    ($val:expr) => {
        println!("Second: {}", $val);
    };   // ← Semicolon (optional on last)
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  (pattern) => { code };                 │
│                      ↑                  │
│                Semicolon required!      │
│                                         │
│  Think of it like match arms:           │
│  Each one needs proper termination.     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Missing Semicolon



## A Common Mistake

```rust
macro_rules! my_macro {
    () => {
        println!("First");
    }     // ← Missing semicolon!
    ($val:expr) => {
        println!("Second");
    }
}
```

Rust error: unexpected token `$`

It's confused because the arms run together!



```
┌─────────────────────────────────────────┐
│                                         │
│  Without semicolons, Rust sees:         │
│                                         │
│  { println!("First"); } ($val:expr)     │
│                         ↑               │
│             "What is this ($val) doing  │
│              here unexpectedly?"        │
│                                         │
│  The semicolon SEPARATES the arms.      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Find the Missing Semicolon

```rust
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }   // ← Look here!
    ($val:expr) => {
        println!("Look at this: {}", $val);
    }
}
```

One character is missing between the arms!



--- slide ---

# What You Need to Do



## Add the Semicolon

```
┌─────────────────────────────────────────┐
│                                         │
│  Look at the first arm:                 │
│                                         │
│  () => {                                │
│      println!("Check out my macro!");   │
│  }                                      │
│   ↑                                     │
│   Add a semicolon here!                 │
│                                         │
│  () => {                                │
│      println!("Check out my macro!");   │
│  };                                     │
│   ↑                                     │
│   Fixed!                                │
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
│  1. How many arms does this macro have? │
│     (Two: one empty, one with $val)     │
│                                         │
│  2. What separates macro arms?          │
│     (Semicolons)                        │
│                                         │
│  3. Which arm is missing its semicolon? │
│     (The first one - after the })       │
│                                         │
│  4. The hint says "one or two chars"    │
│     What's the smallest fix?            │
│     (Just add ; after first arm's })    │
│                                         │
└─────────────────────────────────────────┘
```



Semicolons separate macro arms!

(Go try it in the Editor!)
