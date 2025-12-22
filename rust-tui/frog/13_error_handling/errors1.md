# Error Handling in Rust



## Beyond Option

You learned Option: Some(value) or None.

But None doesn't explain WHAT went wrong.



```
┌─────────────────────────────────────────┐
│                                         │
│  fn parse_number(s: &str) -> Option<i32>│
│                                         │
│  Returns None... but WHY?               │
│                                         │
│  • Was the string empty?                │
│  • Was it not a number?                 │
│  • Was it too big?                      │
│  • Was it negative when we need positive│
│                                         │
│  None tells us NOTHING about the cause! │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Option vs Result



## Two Sides of Failure

```
┌─────────────────────────────────────────┐
│                                         │
│  Option<T>                              │
│                                         │
│  • Some(value) = success                │
│  • None = absence (no reason given)     │
│                                         │
│  Use for: "might not exist"             │
│    Finding in a list                    │
│    Optional configuration               │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  Result<T, E>                           │
│                                         │
│  • Ok(value) = success                  │
│  • Err(error) = failure WITH reason     │
│                                         │
│  Use for: "might fail" operations       │
│    File I/O                             │
│    Parsing                              │
│    Network requests                     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Result Type



## Rust's Error Handling Primitive

```rust
enum Result<T, E> {
    Ok(T),    // Success, with value of type T
    Err(E),   // Failure, with error of type E
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Result<T, E> has TWO type parameters:  │
│                                         │
│  T = the SUCCESS type                   │
│      What you get when things work      │
│                                         │
│  E = the ERROR type                     │
│      What you get when things fail      │
│                                         │
│  Both are meaningful!                   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Result Examples



## Concrete Types

```rust
Result<i32, String>
// Ok(42) or Err("something went wrong")

Result<User, DatabaseError>
// Ok(user) or Err(DatabaseError::NotFound)

Result<String, ParseIntError>
// Ok("parsed value") or Err(parse_error)
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The error type E can be ANYTHING:      │
│                                         │
│  • String (simple error message)        │
│  • Custom enum (structured errors)      │
│  • Standard library error types         │
│                                         │
│  You choose based on what information   │
│  callers need about the failure.        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Creating Ok Values



## Success Case

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b != 0 {
        Ok(a / b)  // Wrap success in Ok
    } else {
        // ... handle error
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Ok(value)                              │
│     ↑                                   │
│     Wrap your success value             │
│                                         │
│  Like Some() for Option, but for Result │
│                                         │
│  The value becomes Result<T, E>         │
│  where T is inferred from the value.    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Creating Err Values



## Failure Case

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b != 0 {
        Ok(a / b)
    } else {
        Err(String::from("Cannot divide by zero"))
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Err(error_value)                       │
│      ↑                                  │
│      Wrap your error information        │
│                                         │
│  The error can be:                      │
│  • A String message                     │
│  • A custom error type                  │
│  • An error from another function       │
│                                         │
│  The error EXPLAINS what went wrong!    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Error Information Matters



## Debugging and Recovery

```rust
// With Option - no information
fn validate(s: &str) -> Option<&str> {
    if s.is_empty() { None }
    else { Some(s) }
}
// None... but why? Was it empty? Invalid chars? Too long?

// With Result - clear information
fn validate(s: &str) -> Result<&str, String> {
    if s.is_empty() {
        Err(String::from("Input was empty"))
    } else {
        Ok(s)
    }
}
// Err("Input was empty") - we know exactly what happened!
```



--- slide ---

# Result in the Prelude



## No Import Needed

Like Option, Result is in the prelude:

```rust
// These are available automatically:
// - Result
// - Ok
// - Err

let success: Result<i32, String> = Ok(42);
let failure: Result<i32, String> = Err(String::from("oops"));
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Result, Ok, and Err are used           │
│  everywhere in Rust.                    │
│                                         │
│  No use statement needed!               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Handling Results



## Pattern Matching

```rust
let result = divide(10, 2);

match result {
    Ok(value) => println!("Result: {}", value),
    Err(e) => println!("Error: {}", e),
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  match on Result:                       │
│                                         │
│  Ok(value) => ...                       │
│    → Extract and use the success value  │
│                                         │
│  Err(e) => ...                          │
│    → Extract and handle the error       │
│                                         │
│  You MUST handle both cases!            │
│  (Or use methods like unwrap)           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Converting Option to Result



## Adding Error Context

Sometimes you have Option but want Result:

```rust
// Option version
fn find_user(id: u32) -> Option<User> { ... }

// Result version - more informative
fn find_user(id: u32) -> Result<User, String> {
    // Instead of None, return Err with explanation
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  CONVERSION:                            │
│                                         │
│  Option        →    Result              │
│  ───────            ──────              │
│  Some(v)       →    Ok(v)               │
│  None          →    Err(error_message)  │
│                                         │
│  Replace None with Err that explains!   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Transformation



## Step by Step

```rust
// BEFORE: Option<String>
fn generate(name: String) -> Option<String> {
    if name.is_empty() {
        None  // No explanation!
    } else {
        Some(format!("Hi! My name is {}", name))
    }
}

// AFTER: Result<String, String>
fn generate(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err(String::from("Empty names aren't allowed"))
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}
```



--- slide ---

# What Changes?



## Summary of Differences

```
┌─────────────────────────────────────────┐
│                                         │
│  RETURN TYPE:                           │
│    Option<T>  →  Result<T, E>           │
│                                         │
│  SUCCESS:                               │
│    Some(value)  →  Ok(value)            │
│                                         │
│  FAILURE:                               │
│    None  →  Err(explanation)            │
│                                         │
│  The success value type (T) can stay    │
│  the same!                              │
│                                         │
│  You're adding error information (E),   │
│  not changing the success path.         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Choosing Error Types



## What Should E Be?

For your error type, you can use:

```
┌─────────────────────────────────────────┐
│                                         │
│  String                                 │
│    Simple, flexible, human-readable     │
│    Good for: quick prototypes, messages │
│                                         │
│  &'static str                           │
│    String literal, no allocation        │
│    Good for: fixed error messages       │
│                                         │
│  Custom enum                            │
│    Structured, matchable errors         │
│    Good for: libraries, complex logic   │
│                                         │
│  Standard library types                 │
│    ParseIntError, IoError, etc.         │
│    Good for: propagating system errors  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Converting Option to Result

You have a function returning Option:

```rust
fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        None
    } else {
        Some(format!("Hi! My name is {name}"))
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  TASK:                                  │
│                                         │
│  Change this to return Result instead!  │
│                                         │
│  1. Change the return type              │
│     Option<String> → Result<String, String>│
│                                         │
│  2. Change None to Err with a message   │
│     None → Err(String::from("..."))     │
│                                         │
│  3. Change Some to Ok                   │
│     Some(...) → Ok(...)                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Tests Tell You What



## Reading the Expectations

```rust
// Test expects Ok for valid input
assert_eq!(
    generate_nametag_text("Beyoncé".to_string()).as_deref(),
    Ok("Hi! My name is Beyoncé"),
);

// Test expects Err with specific message for empty input
assert_eq!(
    ... .map_err(|e| e.as_str()),
    Err("Empty names aren't allowed"),
);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The tests reveal:                      │
│                                         │
│  • Success should return Ok(...)        │
│  • Error should return Err(...)         │
│  • Error message: "Empty names aren't   │
│    allowed"                             │
│                                         │
│  Match the exact error message!         │
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
│  1. What is the current return type?    │
│                                         │
│  2. What should the new return type be? │
│     (Result with what types?)           │
│                                         │
│  3. In the success case, what do you    │
│     wrap the value with?                │
│     (Hint: Not Some anymore!)           │
│                                         │
│  4. In the failure case, what do you    │
│     return instead of None?             │
│                                         │
│  5. What error message does the test    │
│     expect? (Look carefully!)           │
│                                         │
│  6. How do you create a String from     │
│     a string literal?                   │
│                                         │
└─────────────────────────────────────────┘
```



Three changes: return type, success wrapper, error case!

(Now go to the Editor and try it!)
