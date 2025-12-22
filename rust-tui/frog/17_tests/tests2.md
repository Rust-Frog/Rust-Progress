# assert_eq! and assert_ne!



## Comparing Values

When testing equality, use assert_eq!:

```rust
#[test]
fn test_addition() {
    assert_eq!(2 + 2, 4);  // Passes!
    assert_eq!(2 + 2, 5);  // Fails!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  assert_eq!(left, right)                │
│            ↑      ↑                     │
│            │      └─ Expected value     │
│            └─ Actual value              │
│                                         │
│  Checks: left == right                  │
│                                         │
│  If equal: test passes                  │
│  If not equal: test fails with message  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why assert_eq! Over assert!



## Better Error Messages

```rust
// Using assert!
assert!(result == 4);
// Failure: "assertion failed: result == 4"

// Using assert_eq!
assert_eq!(result, 4);
// Failure: "assertion failed: `(left == right)`
//           left: `5`,
//          right: `4`"
```



```
┌─────────────────────────────────────────┐
│                                         │
│  assert_eq! shows BOTH values!          │
│                                         │
│  When a test fails, you see:            │
│  • What you got (left)                  │
│  • What you expected (right)            │
│                                         │
│  This makes debugging much easier!      │
│                                         │
│  Always prefer assert_eq! when          │
│  comparing two values.                  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# assert_ne!



## Asserting Inequality

```rust
#[test]
fn test_not_equal() {
    assert_ne!(1, 2);  // 1 != 2, passes!
    assert_ne!(1, 1);  // 1 != 1 is false, fails!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  assert_ne!(left, right)                │
│                                         │
│  Checks: left != right                  │
│                                         │
│  ne = "not equal"                       │
│                                         │
│  Use when you want to verify two        │
│  values are DIFFERENT.                  │
│                                         │
│  Less common than assert_eq!, but       │
│  useful for certain tests.              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Testing Return Values



## Function Output Verification

```rust
fn power_of_2(n: u8) -> u64 {
    1 << n  // Bit shift: 2^n
}

#[test]
fn test_power_of_2() {
    assert_eq!(power_of_2(0), 1);   // 2^0 = 1
    assert_eq!(power_of_2(1), 2);   // 2^1 = 2
    assert_eq!(power_of_2(2), 4);   // 2^2 = 4
    assert_eq!(power_of_2(3), 8);   // 2^3 = 8
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Pattern:                               │
│  assert_eq!(function(input), expected)  │
│                                         │
│  Call the function with known input,    │
│  compare to the expected output.        │
│                                         │
│  Test multiple inputs to be thorough!   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Understanding Bit Shifts



## What 1 << n Means

```rust
1 << 0  // 1 (binary: 1)
1 << 1  // 2 (binary: 10)
1 << 2  // 4 (binary: 100)
1 << 3  // 8 (binary: 1000)
1 << 4  // 16 (binary: 10000)
```



```
┌─────────────────────────────────────────┐
│                                         │
│  1 << n = "shift 1 left by n bits"      │
│         = 2 to the power of n           │
│         = 2^n                           │
│                                         │
│  n=0: 2^0 = 1                           │
│  n=1: 2^1 = 2                           │
│  n=2: 2^2 = 4                           │
│  n=3: 2^3 = 8                           │
│                                         │
│  This is a fast way to compute          │
│  powers of 2!                           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Argument Order Convention



## left vs right

```rust
// Convention:
assert_eq!(actual, expected);
//         ↑       ↑
//         │       └─ What you expect
//         └─ What you got (function result)

// Example:
assert_eq!(power_of_2(3), 8);
//         └─ actual    └─ expected
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Convention (not required, but common): │
│                                         │
│  assert_eq!(actual, expected)           │
│                                         │
│  Put the function call first,           │
│  the expected value second.             │
│                                         │
│  Makes error messages more intuitive!   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Testing power_of_2

You have:

```rust
fn power_of_2(n: u8) -> u64 {
    1 << n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Test the function `power_of_2`
        assert_eq!();
        assert_eq!();
        assert_eq!();
        assert_eq!();
    }
}
```



--- slide ---

# What to Do



## Fill in Four Assertions

```
┌─────────────────────────────────────────┐
│                                         │
│  TASK: Complete the four assert_eq!     │
│                                         │
│  Each assert_eq! needs two arguments:   │
│  1. Call to power_of_2 with some n      │
│  2. The expected result (2^n)           │
│                                         │
│  EXAMPLE:                               │
│  assert_eq!(power_of_2(0), 1);          │
│                                         │
│  TEST CASES TO CONSIDER:                │
│                                         │
│  power_of_2(0) = 2^0 = 1                │
│  power_of_2(1) = 2^1 = 2                │
│  power_of_2(2) = 2^2 = 4                │
│  power_of_2(3) = 2^3 = 8                │
│  power_of_2(4) = 2^4 = 16               │
│                                         │
│  Pick any four to test!                 │
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
│  1. What does assert_eq! take?          │
│     (Two values to compare)             │
│                                         │
│  2. What is power_of_2(0)?              │
│     (1 - because 2^0 = 1)               │
│                                         │
│  3. What is power_of_2(1)?              │
│     (2 - because 2^1 = 2)               │
│                                         │
│  4. What is power_of_2(2)?              │
│     (4 - because 2^2 = 4)               │
│                                         │
│  5. What is power_of_2(3)?              │
│     (8 - because 2^3 = 8)               │
│                                         │
│  6. What's the syntax?                  │
│     assert_eq!(power_of_2(n), result)   │
│                                         │
└─────────────────────────────────────────┘
```



Fill in all four assert_eq! calls!

(Now go to the Editor and try it!)
