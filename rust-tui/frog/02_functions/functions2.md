# Function Parameters

## The Problem

```
fn call_me(num:) {   // <-- Something's missing!
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
```

Look at `num:` — what comes after the colon?


--- slide ---

# Parameters in Rust

## What is a Parameter?

A parameter is a variable that receives a value 
when the function is called.

```
fn greet(name) {  // 'name' is a parameter
    println!("Hello, {name}!");
}

greet("Alice");   // "Alice" is an argument
```

## The Key Rule

```
┌────────────────────────────────────────────────┐
│  In Rust, EVERY parameter MUST have a type!    │
│                                                │
│  This is required — not optional like in       │
│  some other languages.                         │
└────────────────────────────────────────────────┘
```

--- slide ---

# Parameter Syntax

## The Format

```
fn function_name(param_name: Type) {
                           ^^^^^
                           │
                           └── Type is REQUIRED
}
```

## Examples

```
fn print_number(x: i32) { ... }
fn say_hello(name: &str) { ... }
fn is_valid(flag: bool) { ... }
fn calculate(a: f64, b: f64) { ... }
```

## Multiple Parameters

Separate with commas, each needs a type:

```
fn describe(name: &str, age: u32, active: bool) {
            ^^^^^^^^^^  ^^^^^^^^  ^^^^^^^^^^^^
            First       Second    Third
}
```

--- slide ---

# Why Require Types?

## Rust's Design Decision

```
┌────────────────────────────────────────────────┐
│  BENEFITS OF REQUIRED TYPES                    │
├────────────────────────────────────────────────┤
│  • Compiler catches type errors early          │
│  • Better error messages                       │
│  • Code is self-documenting                    │
│  • Reduces bugs from wrong types               │
└────────────────────────────────────────────────┘
```

## Compare to Other Languages

```
// Python: Type is not specified
def call_me(num):
    ...

// Rust: Type is REQUIRED
fn call_me(num: i32) {
    ...
}
```

--- slide ---

# Understanding the Problem

Look at the code:

```
fn call_me(num:) {
              ^^
              │
              └── There's a colon but no type!
```

## The Error

```
error: expected type, found `)`
```

## Think About It

```
┌────────────────────────────────────────────────┐
│  QUESTIONS TO CONSIDER:                        │
│                                                │
│  1. What should come after the colon?          │
│                                                │
│  2. Look at how num is used: `0..num`          │
│     This is a range — what type makes sense?   │
│                                                │
│  3. Look at the function call: `call_me(3)`    │
│     The argument is 3 — an integer!            │
└────────────────────────────────────────────────┘
```

--- slide ---

# Common Integer Types

## For Parameters

```
┌──────────┬───────────────────────────────┐
│ Type     │ Common Use                    │
├──────────┼───────────────────────────────┤
│ i32      │ General integers (default)    │
│ u32      │ Positive-only integers        │
│ usize    │ Array/slice indexing          │
│ i64      │ Large integers                │
└──────────┴───────────────────────────────┘
```

## Hint

The loop uses `0..num` which means `num` should 
be some kind of integer. Think about what makes 
sense for a "number of times to ring"!
