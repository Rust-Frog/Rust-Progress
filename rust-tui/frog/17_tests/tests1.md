# Testing in Rust



## Why Write Tests?

Tests verify your code works as expected:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Does add(2, 2) really equal 4?
// A test can prove it!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Tests help you:                        │
│                                         │
│  • Catch bugs before users do           │
│  • Refactor with confidence             │
│  • Document expected behavior           │
│  • Prevent regressions                  │
│                                         │
│  Rust has built-in test support!        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Test Structure



## The Anatomy of a Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        // Test code here
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  #[cfg(test)]                           │
│    Only compile this when testing       │
│                                         │
│  mod tests { }                          │
│    A module to contain tests            │
│                                         │
│  use super::*;                          │
│    Import everything from parent module │
│                                         │
│  #[test]                                │
│    Marks a function as a test           │
│                                         │
│  fn test_name() { }                     │
│    The actual test function             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The #[test] Attribute



## Marking Test Functions

```rust
#[test]
fn my_test() {
    // This function is a test
}

fn not_a_test() {
    // This is a regular function
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  #[test] tells the test runner:         │
│  "Run this function as a test!"         │
│                                         │
│  Without #[test], the function is       │
│  just a regular helper function.        │
│                                         │
│  Run tests with:                        │
│  cargo test                             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The assert! Macro



## Asserting Truth

```rust
#[test]
fn test_basics() {
    assert!(true);   // Passes
    assert!(1 + 1 == 2);  // Passes
    assert!(false);  // FAILS - panics!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  assert!(condition)                     │
│         ↑                               │
│         Must be true!                   │
│                                         │
│  If condition is true:                  │
│    → Test continues                     │
│                                         │
│  If condition is false:                 │
│    → Test PANICS (fails)                │
│                                         │
│  assert! is the simplest assertion.     │
│  Just check if something is true!       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Testing Functions



## Call and Assert

```rust
fn is_even(n: i64) -> bool {
    n % 2 == 0
}

#[test]
fn test_is_even() {
    assert!(is_even(4));   // 4 is even → true
    assert!(is_even(0));   // 0 is even → true
    assert!(!is_even(5));  // 5 is NOT even → true
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Call the function, assert the result!  │
│                                         │
│  is_even(4) returns true                │
│  assert!(true) passes ✓                 │
│                                         │
│  is_even(5) returns false               │
│  assert!(!false) = assert!(true) ✓      │
│                                         │
│  Use ! to negate when testing for false │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Importing from Parent Module



## use super::*

```rust
fn is_even(n: i64) -> bool {  // In outer scope
    n % 2 == 0
}

mod tests {
    use super::*;  // Import is_even!

    #[test]
    fn test() {
        is_even(4);  // Now accessible
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  use super::*                           │
│      ↑     ↑                            │
│      │     └─ Everything (wildcard)     │
│      └─ Parent module                   │
│                                         │
│  The tests module is INSIDE the file.   │
│  super refers to the parent scope.      │
│  * imports all public items.            │
│                                         │
│  Without this, is_even isn't visible!   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Multiple Assertions



## Testing Various Cases

```rust
#[test]
fn test_is_even() {
    // Test positive even
    assert!(is_even(2));
    assert!(is_even(100));

    // Test zero
    assert!(is_even(0));

    // Test odd numbers (should be false)
    assert!(!is_even(1));
    assert!(!is_even(99));
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Good tests check multiple cases:       │
│                                         │
│  • Normal/expected inputs               │
│  • Edge cases (0, negative, etc.)       │
│  • Both true and false outcomes         │
│                                         │
│  Each assert! must pass for the test    │
│  to succeed!                            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Testing is_even

You have:

```rust
fn is_even(n: i64) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    // TODO: Import `is_even`.

    #[test]
    fn you_can_assert() {
        // TODO: Test the function `is_even`
        assert!();
        assert!();
    }
}
```



--- slide ---

# What to Do



## Two Tasks

```
┌─────────────────────────────────────────┐
│                                         │
│  TASK 1: Import is_even                 │
│                                         │
│  The tests module can't see is_even!    │
│  Add: use super::*;                     │
│  (imports everything from parent)       │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  TASK 2: Fill in the assertions         │
│                                         │
│  assert!() is empty - needs a condition!│
│                                         │
│  First assert!: test an even number     │
│  assert!(is_even(???));                 │
│                                         │
│  Second assert!: test an odd number     │
│  assert!(!is_even(???));                │
│  (Note the ! to negate)                 │
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
│  1. How do you import from parent?      │
│     (use super::*)                      │
│                                         │
│  2. What goes inside assert!()?         │
│     (A boolean expression)              │
│                                         │
│  3. What does is_even return for 4?     │
│     (true - 4 is even)                  │
│                                         │
│  4. What does is_even return for 5?     │
│     (false - 5 is odd)                  │
│                                         │
│  5. How do you assert something is      │
│     false?                              │
│     (assert!(!expression) - negate it!) │
│                                         │
│  6. What numbers should you test?       │
│     (One even, one odd - your choice!)  │
│                                         │
└─────────────────────────────────────────┘
```



Import the function and fill in assertions!

(Now go to the Editor and try it!)
