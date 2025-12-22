# Custom Error Types



## Beyond String Errors

Using `String` as error type works:

```rust
fn validate(x: i32) -> Result<i32, String> {
    if x < 0 {
        Err(String::from("negative number"))
    } else {
        Ok(x)
    }
}
```

But it has limitations...



--- slide ---

# Problems With String Errors



## Why Strings Aren't Ideal

```
┌─────────────────────────────────────────┐
│                                         │
│  PROBLEMS WITH String AS ERROR:         │
│                                         │
│  1. NOT MATCHABLE                       │
│     Can't easily handle different cases │
│     if err == "negative" { }  // Brittle│
│                                         │
│  2. NO STRUCTURE                        │
│     Just text, no programmatic data     │
│     Can't extract the value that failed │
│                                         │
│  3. TYPO-PRONE                          │
│     "negaitve" vs "negative"            │
│     No compiler help                    │
│                                         │
│  4. NOT SELF-DOCUMENTING                │
│     What errors are possible?           │
│     Reader must search the code         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Enter: Error Enums



## Structured Error Types

Define your own error enum:

```rust
#[derive(Debug)]
enum ValidationError {
    Negative,
    Zero,
    TooLarge,
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Benefits of error enums:               │
│                                         │
│  • MATCHABLE: switch on error type      │
│  • EXHAUSTIVE: compiler lists all cases │
│  • DOCUMENTED: errors are in the type   │
│  • TYPE-SAFE: no typos possible         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Using Error Enums



## In Result Types

```rust
#[derive(Debug)]
enum CreationError {
    Negative,
    Zero,
}

fn create_positive(value: i64) -> Result<u64, CreationError> {
    if value < 0 {
        Err(CreationError::Negative)
    } else if value == 0 {
        Err(CreationError::Zero)
    } else {
        Ok(value as u64)
    }
}
```



--- slide ---

# Handling Custom Errors



## Match on Error Variants

```rust
match create_positive(-5) {
    Ok(n) => println!("Got: {}", n),
    Err(CreationError::Negative) => {
        println!("Can't use negative numbers!");
    }
    Err(CreationError::Zero) => {
        println!("Zero is not allowed!");
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  With enum errors, callers can:         │
│                                         │
│  • Handle each error type differently   │
│  • The compiler ensures all cases       │
│    are covered                          │
│  • Add new variants = compiler warns    │
│    about unhandled cases                │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Error Enums With Data



## Carrying Information

Variants can carry relevant data:

```rust
#[derive(Debug)]
enum ValidationError {
    TooSmall { minimum: i32, got: i32 },
    TooLarge { maximum: i32, got: i32 },
    InvalidChar { position: usize, char: char },
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Now errors carry CONTEXT:              │
│                                         │
│  Err(ValidationError::TooSmall {        │
│      minimum: 10,                       │
│      got: 5,                            │
│  })                                     │
│                                         │
│  "Expected at least 10, but got 5"      │
│  All the info you need to report it!    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Designing Error Types



## What to Include

```
┌─────────────────────────────────────────┐
│                                         │
│  GOOD ERROR ENUM DESIGN:                │
│                                         │
│  1. One variant per failure CAUSE       │
│     Negative, Zero, TooLarge            │
│     (not Generic, Other, Unknown)       │
│                                         │
│  2. Include relevant DATA               │
│     The value that failed               │
│     The limit that was exceeded         │
│     The position where error occurred   │
│                                         │
│  3. Make variants ACTIONABLE            │
│     Caller should know what to do       │
│     "Negative" → "use a positive value" │
│                                         │
│  4. Derive Debug for printing           │
│     #[derive(Debug)]                    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# PartialEq for Testing



## Making Errors Comparable

For tests, derive PartialEq:

```rust
#[derive(Debug, PartialEq)]
enum CreationError {
    Negative,
    Zero,
}

// Now you can use assert_eq! with errors
assert_eq!(create(-5), Err(CreationError::Negative));
```



```
┌─────────────────────────────────────────┐
│                                         │
│  #[derive(Debug, PartialEq)]            │
│                                         │
│  Debug: for printing {:?}               │
│  PartialEq: for == comparison           │
│                                         │
│  This lets you write:                   │
│  assert_eq!(result, Err(ExpectedError)) │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Conditional Logic in new()



## Validating on Construction

A common pattern: validate in constructors:

```rust
struct PositiveNumber(u64);

impl PositiveNumber {
    fn new(value: i64) -> Result<Self, CreationError> {
        if value < 0 {
            Err(CreationError::Negative)
        } else if value == 0 {
            Err(CreationError::Zero)
        } else {
            Ok(Self(value as u64))
        }
    }
}
```



--- slide ---

# The Pattern



## Validate → Return Error or Ok

```
┌─────────────────────────────────────────┐
│                                         │
│  fn new(value: T) -> Result<Self, E> {  │
│                                         │
│      // Check condition 1               │
│      if invalid_condition_1 {           │
│          return Err(Error::Variant1);   │
│      }                                  │
│                                         │
│      // Check condition 2               │
│      if invalid_condition_2 {           │
│          return Err(Error::Variant2);   │
│      }                                  │
│                                         │
│      // All checks passed               │
│      Ok(Self(value))                    │
│  }                                      │
│                                         │
│  Each condition maps to an error type!  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Order of Checks Matters



## Most Specific First

```rust
fn new(value: i64) -> Result<Self, CreationError> {
    // Check most specific/restrictive first
    if value < 0 {
        Err(CreationError::Negative)
    } else if value == 0 {
        Err(CreationError::Zero)
    } else {
        Ok(Self(value as u64))
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  WHY ORDER MATTERS:                     │
│                                         │
│  value = -5                             │
│                                         │
│  if value < 0     → TRUE → Negative     │
│  if value == 0    → (not reached)       │
│                                         │
│  value = 0                              │
│                                         │
│  if value < 0     → FALSE               │
│  if value == 0    → TRUE → Zero         │
│                                         │
│  value = 5                              │
│                                         │
│  if value < 0     → FALSE               │
│  if value == 0    → FALSE               │
│  else             → Ok(...)             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Implementing Validation Logic

You have:

```rust
#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: Currently always returns Ok
        Ok(Self(value as u64))
    }
}
```



--- slide ---

# What the Tests Expect



## Reading the Assertions

```rust
assert_eq!(
    PositiveNonzeroInteger::new(10),
    Ok(PositiveNonzeroInteger(10)),  // Positive: Ok
);
assert_eq!(
    PositiveNonzeroInteger::new(-10),
    Err(CreationError::Negative),    // Negative: Error
);
assert_eq!(
    PositiveNonzeroInteger::new(0),
    Err(CreationError::Zero),        // Zero: Error
);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  THREE CASES:                           │
│                                         │
│  value > 0  → Ok(PositiveNonzeroInteger)│
│  value < 0  → Err(CreationError::Negative)│
│  value == 0 → Err(CreationError::Zero)  │
│                                         │
│  The function currently ALWAYS returns  │
│  Ok, even for invalid values!           │
│                                         │
│  Add the validation checks.             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Logic to Add



## Checking Conditions

```
┌─────────────────────────────────────────┐
│                                         │
│  CURRENT CODE:                          │
│                                         │
│  fn new(value: i64) -> Result<...> {    │
│      Ok(Self(value as u64))             │
│  }                                      │
│                                         │
│  Always Ok - no validation!             │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  WHAT YOU NEED:                         │
│                                         │
│  fn new(value: i64) -> Result<...> {    │
│      // Check for negative              │
│      // Check for zero                  │
│      // Otherwise return Ok             │
│  }                                      │
│                                         │
│  Add if/else checks before Ok!          │
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
│  1. What are the three possible outcomes?│
│     (Look at the test assertions!)      │
│                                         │
│  2. How do you check if value is        │
│     negative? (What comparison?)        │
│                                         │
│  3. How do you check if value is zero?  │
│                                         │
│  4. What error variant for negative?    │
│     (CreationError::???)                │
│                                         │
│  5. What error variant for zero?        │
│     (CreationError::???)                │
│                                         │
│  6. In what order should you check?     │
│     (negative first? zero first?)       │
│                                         │
│  7. When do you return Ok?              │
│     (After checking both conditions!)   │
│                                         │
└─────────────────────────────────────────┘
```



Add validation before the Ok return!

(Now go to the Editor and try it!)
