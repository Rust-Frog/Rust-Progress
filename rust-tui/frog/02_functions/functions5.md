# Expressions vs Statements

## The Problem

```
fn square(num: i32) -> i32 {
    num * num;    // <-- Look closely!
}
```

The function says it returns `i32`, but something's 
wrong with how it's returning the value.


--- slide ---

# Expressions vs Statements

## The Core Difference

```
┌────────────────────────────────────────────────┐
│  EXPRESSION: Evaluates to a value              │
│                                                │
│  5 + 3          → 8                            │
│  num * num      → some number                  │
│  true           → true                         │
│                                                │
│  STATEMENT: Performs an action, returns ()     │
│                                                │
│  let x = 5;     → ()                           │
│  println!("x"); → ()                           │
│  5 + 3;         → ()   (note the semicolon!)   │
└────────────────────────────────────────────────┘
```

## The Key Insight

**Adding a semicolon turns an expression into a 
statement!**

--- slide ---

# The Semicolon Effect

## Without Semicolon = Expression

```
num * num     // This EVALUATES to a number
```

## With Semicolon = Statement

```
num * num;    // This PERFORMS the calculation
              // but RETURNS () (nothing!)
```

## Why This Matters for Functions

```
fn square(num: i32) -> i32 {
    num * num     // Returns the result
}

fn square(num: i32) -> i32 {
    num * num;    // Returns ()... ERROR!
}
```

--- slide ---

# How Rust Returns Values

## Implicit Return

The LAST expression (no semicolon) is returned:

```
fn double(x: i32) -> i32 {
    x * 2         // No semicolon = returned
}
```

## Explicit Return

Use `return` keyword (rarely needed):

```
fn double(x: i32) -> i32 {
    return x * 2;  // Explicit return
}
```

## The Common Mistake

```
fn double(x: i32) -> i32 {
    x * 2;    // Oops! Semicolon makes it ()
}

// Error: expected `i32`, found `()`
```

--- slide ---

# Understanding the Problem

Look at the code:

```
fn square(num: i32) -> i32 {
    num * num;
             ^
             │
             └── This semicolon is the problem!
}
```

## What's Happening

```
┌────────────────────────────────────────────────┐
│  Function signature: -> i32 (expects i32)      │
│                                                │
│  Function body: num * num;                     │
│                          ^ semicolon!          │
│                                                │
│  What's returned: () (unit, not i32!)          │
│                                                │
│  Error: mismatched types                       │
└────────────────────────────────────────────────┘
```

## Think About It

```
┌────────────────────────────────────────────────┐
│  QUESTIONS TO CONSIDER:                        │
│                                                │
│  1. What does the function promise to return?  │
│                                                │
│  2. What is `num * num;` (with semicolon)?     │
│     An expression or a statement?              │
│                                                │
│  3. How do you make it an expression again?    │
└────────────────────────────────────────────────┘
```

--- slide ---

# Key Takeaways

## The Semicolon Rule

```
┌────────────────────────────────────────────────┐
│  EXPRESSION (returns value):                   │
│    num * num                                   │
│                                                │
│  STATEMENT (returns ()):                       │
│    num * num;                                  │
│              ^                                 │
│              Adding semicolon changes meaning! │
└────────────────────────────────────────────────┘
```

## For Return Values

- Last line WITHOUT semicolon = returned
- Last line WITH semicolon = returns ()

## The Error Hint

The compiler tells you exactly what to do:

```
help: remove this semicolon to return this value
```

Read error messages carefully!
