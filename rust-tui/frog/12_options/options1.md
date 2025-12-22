# The Problem of "Nothing"



## What If There's No Value?

Sometimes a value might not exist:

- Looking up a key that's not in a HashMap
- Finding the first element of an empty list
- Parsing a string that isn't a valid number



```
┌─────────────────────────────────────────┐
│                                         │
│  In other languages:                    │
│                                         │
│  • null / nil / None                    │
│  • Returns -1 as "not found"            │
│  • Throws an exception                  │
│                                         │
│  Problems with these approaches:        │
│                                         │
│  • Null pointer exceptions (crashes!)   │
│  • Magic values (-1) can be forgotten   │
│  • Exceptions bypass normal flow        │
│                                         │
│  Rust has a better solution...          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Enter: Option



## Making "Maybe" Explicit

Rust's `Option` type represents a value that

might or might not exist:

```rust
enum Option<T> {
    Some(T),  // There IS a value
    None,     // There is NO value
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Option<T> is an enum with two variants:│
│                                         │
│  Some(value)                            │
│    → "I have a value, here it is!"      │
│                                         │
│  None                                   │
│    → "I have nothing"                   │
│                                         │
│  The <T> means it works with ANY type.  │
│  Option<i32>, Option<String>, etc.      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Option Is Powerful



## The Compiler Helps You

With Option, you CAN'T forget to handle "nothing":

```rust
fn find_user(id: u32) -> Option<User> { ... }

let user = find_user(42);
// user is Option<User>, NOT User!

// This won't compile:
println!("{}", user.name);  // ERROR!

// You MUST handle both cases:
match user {
    Some(u) => println!("{}", u.name),
    None => println!("User not found"),
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The TYPE SYSTEM forces you to handle   │
│  the possibility of "nothing".          │
│                                         │
│  No more null pointer exceptions!       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Creating Some Values



## Wrapping Values in Some

To create an Option with a value:

```rust
let some_number: Option<i32> = Some(42);
let some_string: Option<String> = Some(String::from("hello"));
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Some(value)                            │
│      ↑                                  │
│      Wraps the value in Option          │
│                                         │
│  The type becomes Option<T> where T     │
│  is the type of the inner value.        │
│                                         │
│  Some(42)     → Option<i32>             │
│  Some("hi")   → Option<&str>            │
│  Some(true)   → Option<bool>            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Creating None



## Representing Absence

To create an Option with no value:

```rust
let no_number: Option<i32> = None;
let no_string: Option<String> = None;
```



```
┌─────────────────────────────────────────┐
│                                         │
│  None                                   │
│                                         │
│  Represents "no value" or "nothing".    │
│                                         │
│  Important: None still has a TYPE!      │
│                                         │
│  None by itself doesn't know what       │
│  type it's "none of".                   │
│                                         │
│  You often need type annotation:        │
│    let x: Option<i32> = None;           │
│                                         │
│  Or Rust infers from context.           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Option in the Prelude



## No Import Needed

Option, Some, and None are in the prelude:

```rust
// You DON'T need to write:
use std::option::Option;
use std::option::Option::Some;
use std::option::Option::None;

// Just use them directly:
let x = Some(5);
let y: Option<i32> = None;
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Option is SO common that Rust          │
│  automatically imports:                 │
│                                         │
│  • Option (the type)                    │
│  • Some (the variant)                   │
│  • None (the variant)                   │
│                                         │
│  You can use them anywhere without      │
│  any use statements!                    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Returning Option from Functions



## Expressing "Might Fail"

Functions use Option to express possible absence:

```rust
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None  // Can't divide by zero!
    } else {
        Some(a / b)  // Here's the result
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Return type: Option<i32>               │
│                                         │
│  This tells the caller:                 │
│  "You might get an i32, or nothing"     │
│                                         │
│  Return Some(value) for success.        │
│  Return None for failure/absence.       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Conditional Returns



## Building Option Logic

When returning Option, you decide the logic:

```rust
fn check_age(age: u32) -> Option<&'static str> {
    if age < 18 {
        None  // Not eligible
    } else {
        Some("Welcome!")  // Eligible
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Pattern for Option-returning functions:│
│                                         │
│  1. Check conditions                    │
│                                         │
│  2. If invalid → return None            │
│                                         │
│  3. If valid → return Some(value)       │
│                                         │
│  Use if/else, match, or other control   │
│  flow to decide which to return.        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Multiple Conditions



## Complex Option Logic

You can have multiple branches:

```rust
fn categorize(value: i32) -> Option<&'static str> {
    if value < 0 {
        None  // Invalid: negative
    } else if value == 0 {
        Some("zero")
    } else if value < 10 {
        Some("small")
    } else {
        Some("large")
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Multiple Some branches: fine!          │
│  Multiple None branches: also fine!     │
│                                         │
│  Return whatever makes sense for        │
│  your function's logic.                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Extracting Values



## Getting the Value Out

Once you have an Option, how do you use the value?



```
┌─────────────────────────────────────────┐
│                                         │
│  METHODS TO EXTRACT:                    │
│                                         │
│  .unwrap()                              │
│    → Get value or PANIC if None         │
│    → Dangerous! Only use when certain   │
│                                         │
│  .expect("message")                     │
│    → Like unwrap but custom panic msg   │
│                                         │
│  .unwrap_or(default)                    │
│    → Get value or use default if None   │
│                                         │
│  .unwrap_or_default()                   │
│    → Get value or type's default        │
│                                         │
│  Pattern matching (safest!)             │
│    → Handle both cases explicitly       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The unwrap() Method



## Quick But Risky

```rust
let x: Option<i32> = Some(42);
let value = x.unwrap();  // value is 42

let y: Option<i32> = None;
let value = y.unwrap();  // PANIC! Program crashes!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .unwrap()                              │
│                                         │
│  • If Some(v) → returns v               │
│  • If None → PANICS (crashes!)          │
│                                         │
│  USE WITH CAUTION:                      │
│                                         │
│  Only use when you're 100% certain      │
│  the value is Some, not None.           │
│                                         │
│  In tests, it's often acceptable.       │
│  In production code, prefer safer opts. │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The expect() Method



## Unwrap With Context

```rust
let config = load_config().expect("Config file missing!");
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .expect("message")                     │
│                                         │
│  Like unwrap(), but if it panics,       │
│  your message is shown.                 │
│                                         │
│  Better for debugging:                  │
│                                         │
│  .unwrap()                              │
│    → "called unwrap on None"            │
│       (not helpful!)                    │
│                                         │
│  .expect("Failed to load user data")    │
│    → "Failed to load user data"         │
│       (tells you WHAT failed)           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Pattern Matching Option



## The Safest Way

```rust
let maybe_number: Option<i32> = Some(42);

match maybe_number {
    Some(n) => println!("Got: {}", n),
    None => println!("Got nothing"),
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  match handles BOTH cases:              │
│                                         │
│  Some(n) => ...                         │
│    → n is the inner value               │
│    → Use n in this arm                  │
│                                         │
│  None => ...                            │
│    → No value to extract                │
│    → Handle the absence                 │
│                                         │
│  Compiler ensures you handle both!      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Comparing Options



## Equality Works

You can compare Option values directly:

```rust
assert_eq!(Some(5), Some(5));   // true
assert_eq!(Some(5), Some(10));  // false
assert_eq!(Some(5), None);      // false
assert_eq!(None::<i32>, None);  // true
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Some(x) == Some(y) if x == y           │
│  Some(x) == None is always false        │
│  None == None is true                   │
│                                         │
│  This is why tests can use:             │
│  assert_eq!(result, Some(expected))     │
│  assert_eq!(result, None)               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise Part 1



## Implementing the Function

You need to implement `maybe_ice_cream`:

```rust
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16>
```



```
┌─────────────────────────────────────────┐
│                                         │
│  THE RULES:                             │
│                                         │
│  • hour_of_day is 0-23 (24-hour clock)  │
│                                         │
│  • Before 22:00 → 5 scoops left         │
│    (hours 0 through 21)                 │
│                                         │
│  • At 22:00 or 23:00 → 0 scoops left    │
│    (someone ate it all!)                │
│                                         │
│  • Hour 24 or higher → Invalid!         │
│    (return None)                        │
│                                         │
│  Think about the conditions carefully.  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Analyzing the Test Cases



## What the Tests Expect

```rust
assert_eq!(maybe_ice_cream(0), Some(5));   // Midnight: 5 scoops
assert_eq!(maybe_ice_cream(9), Some(5));   // Morning: 5 scoops
assert_eq!(maybe_ice_cream(18), Some(5));  // Evening: 5 scoops
assert_eq!(maybe_ice_cream(22), Some(0));  // 10 PM: 0 scoops
assert_eq!(maybe_ice_cream(23), Some(0));  // 11 PM: 0 scoops
assert_eq!(maybe_ice_cream(24), None);     // Invalid hour
assert_eq!(maybe_ice_cream(25), None);     // Invalid hour
```



```
┌─────────────────────────────────────────┐
│                                         │
│  THREE DISTINCT CASES:                  │
│                                         │
│  1. Hour >= 24 → None (invalid)         │
│                                         │
│  2. Hour >= 22 → Some(0) (eaten)        │
│                                         │
│  3. Hour < 22 → Some(5) (available)     │
│                                         │
│  Order your conditions carefully!       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise Part 2



## Fixing the Test

There's also a test to fix:

```rust
#[test]
fn raw_value() {
    let ice_creams = maybe_ice_cream(12);
    assert_eq!(ice_creams, 5);  // Comparing Option to i32!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  THE PROBLEM:                           │
│                                         │
│  ice_creams is Option<u16>              │
│  5 is just u16                          │
│                                         │
│  You can't compare Option<u16> to u16!  │
│                                         │
│  You need to EXTRACT the value from     │
│  the Option before comparing.           │
│                                         │
│  What method gets the value out?        │
│  (Hint: we discussed several above)     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Think It Through



## Before You Type



```
┌─────────────────────────────────────────┐
│  FOR THE FUNCTION:                      │
│                                         │
│  1. What should happen if hour > 23?    │
│                                         │
│  2. What should happen if hour >= 22    │
│     but <= 23?                          │
│                                         │
│  3. What should happen if hour < 22?    │
│                                         │
│  4. How do you return "nothing"?        │
│                                         │
│  5. How do you return "a value"?        │
│                                         │
├─────────────────────────────────────────┤
│  FOR THE TEST:                          │
│                                         │
│  1. What type is ice_creams?            │
│                                         │
│  2. What type is 5?                     │
│                                         │
│  3. How do you extract the inner value  │
│     from an Option?                     │
│                                         │
└─────────────────────────────────────────┘
```



Two tasks: implement the function AND fix the test!

(Now go to the Editor and try it!)
