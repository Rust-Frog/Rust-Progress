# Quiz 1: Variables, Functions, If



This is your first quiz!

It tests everything you've learned so far.



No hints here - just a detailed recap of what you know.

You've got this!



--- slide ---

# Recap: Variables (Part 1)



## Creating Variables

You create variables with `let`:

```rust
let x = 5;
```



The `let` keyword says "create a new storage box."

The name `x` is the label on the box.

The value `5` goes inside.



## Immutability

By default, variables are IMMUTABLE (can't change):

```rust
let x = 5;
x = 10;    // ERROR! Can't change immutable variable
```



To make it changeable, use `mut`:

```rust
let mut x = 5;
x = 10;    // OK! x is mutable
```



--- slide ---

# Recap: Variables (Part 2)



## Types

Every variable has a type. Rust often infers it:

```rust
let x = 5;        // i32 (integer)
let y = 3.14;     // f64 (float)
let z = true;     // bool
let s = "hello";  // &str (string slice)
```



Common integer types:
- `i32` = signed 32-bit (default)
- `u32` = unsigned 32-bit (no negatives)
- `i64`, `u64` = 64-bit versions



## Type Annotations

You can specify types explicitly:

```rust
let x: i32 = 5;
let name: &str = "Alice";
```



--- slide ---

# Recap: Variables (Part 3)



## Shadowing

You can declare a variable with the same name:

```rust
let x = 5;
let x = x + 1;   // New x, shadowing old x
let x = "hello"; // Can even change type!
```



Shadowing creates a NEW variable - different from mutation!



## Constants

Constants are ALWAYS immutable and need type annotation:

```rust
const MAX_POINTS: u32 = 100_000;
```



Constants use `SCREAMING_CASE` naming.



--- slide ---

# Recap: Functions (Part 1)



## Defining Functions

Use the `fn` keyword:

```rust
fn greet() {
    println!("Hello!");
}
```



## Parameters

Parameters need type annotations:

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```



Multiple parameters separated by commas:

```rust
fn add(a: i32, b: i32) {
    println!("{}", a + b);
}
```



--- slide ---

# Recap: Functions (Part 2)



## Return Types

Use `->` to declare what the function returns:

```rust
fn double(x: i32) -> i32 {
    x * 2
}
```



## Returning Values

The LAST expression (no semicolon) is returned:

```rust
fn square(n: i32) -> i32 {
    n * n    // No semicolon = returned
}
```



Adding semicolon makes it a statement (returns nothing):

```rust
fn broken(n: i32) -> i32 {
    n * n;   // ERROR! Returns () not i32
}
```



--- slide ---

# Recap: Functions (Part 3)



## Calling Functions

Call with parentheses, passing arguments:

```rust
let result = double(5);    // result = 10
greet("Alice");            // Prints "Hello, Alice!"
```



## Key Rules

```
┌────────────────────────────────────────────────┐
│                                                │
│  • Parameter types are REQUIRED                │
│  • Return type required if returning something │
│  • Argument count must match parameters        │
│  • Argument types must match                   │
│  • Last expression without ; is returned       │
│                                                │
└────────────────────────────────────────────────┘
```



--- slide ---

# Recap: If Statements (Part 1)



## Basic If

Run code conditionally:

```rust
if x > 5 {
    println!("big");
}
```



## If-Else

Two paths:

```rust
if x > 5 {
    println!("big");
} else {
    println!("small");
}
```



## Else If

Chain multiple conditions:

```rust
if x > 10 {
    println!("large");
} else if x > 5 {
    println!("medium");
} else {
    println!("small");
}
```



--- slide ---

# Recap: If Statements (Part 2)



## Comparisons (Conditions)

```
┌────────────────────────────────────────────────┐
│                                                │
│  a > b     greater than                        │
│  a < b     less than                           │
│  a >= b    greater than or equal               │
│  a <= b    less than or equal                  │
│  a == b    equal (two equals!)                 │
│  a != b    not equal                           │
│                                                │
└────────────────────────────────────────────────┘
```



## If as Expression

Rust's `if` RETURNS a value:

```rust
let size = if x > 5 { "big" } else { "small" };
```



ALL branches must return the SAME type!



--- slide ---

# Good Luck!



This quiz combines ALL three concepts:

- Define a function with parameters and return type
- Use if/else to make a decision
- Return different values based on a condition



Read the problem carefully.

Look at the tests to understand what's expected.



You have ALL the knowledge you need.

Trust what you've learned!



Remember: Cheating means fooling yourself.
