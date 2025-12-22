# Composite Error Types



## Beyond Box<dyn Error>

Box<dyn Error> is convenient but has drawbacks:

- Callers can't match on specific errors
- Type information is lost
- Not ideal for library code



```
┌─────────────────────────────────────────┐
│                                         │
│  LIBRARY CODE NEEDS:                    │
│                                         │
│  • Specific, matchable error types      │
│  • Callers can handle each case         │
│  • Full type information preserved      │
│                                         │
│  Solution: Composite error enums!       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Wrapping Error Types



## One Enum, Multiple Sources

Create an enum that wraps all possible errors:

```rust
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Each variant WRAPS a different error:  │
│                                         │
│  Creation(CreationError)                │
│    → Holds a CreationError inside       │
│                                         │
│  ParseInt(ParseIntError)                │
│    → Holds a ParseIntError inside       │
│                                         │
│  Now ONE type can represent EITHER      │
│  error, and callers can match on it!    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Using Composite Errors



## In Function Signatures

```rust
fn parse_positive(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    // Can fail with ParseIntError (parsing)
    // Can fail with CreationError (validation)
    // Both wrapped in ParsePosNonzeroError
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Callers see ONE error type:            │
│  ParsePosNonzeroError                   │
│                                         │
│  But can match on variants:             │
│                                         │
│  match error {                          │
│      ParsePosNonzeroError::Creation(e) => ...│
│      ParsePosNonzeroError::ParseInt(e) => ...│
│  }                                      │
│                                         │
│  Best of both worlds!                   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Converting Errors



## The Problem With ?

```rust
fn parse(s: &str) -> Result<Positive, ParsePosNonzeroError> {
    let x: i64 = s.parse()?;  // Returns ParseIntError!
    // But we need ParsePosNonzeroError!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  s.parse() returns ParseIntError        │
│  But function returns ParsePosNonzeroError│
│                                         │
│  ? can't automatically convert!         │
│                                         │
│  We need to WRAP the error in our       │
│  composite type.                        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The map_err Method



## Transforming Errors

`map_err` transforms the error in a Result:

```rust
result.map_err(|e| transform(e))
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .map_err(|e| new_error)                │
│           ↑    ↑                        │
│           │    └─ What to transform to  │
│           └─ The original error         │
│                                         │
│  If Result is Ok: unchanged             │
│  If Result is Err(e): becomes Err(new)  │
│                                         │
│  Perfect for wrapping errors!           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Using map_err



## Wrapping in Composite Type

```rust
fn parse(s: &str) -> Result<Positive, ParsePosNonzeroError> {
    let x: i64 = s.parse()
        .map_err(|e| ParsePosNonzeroError::ParseInt(e))?;

    // Now x is i64, and any error is wrapped!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  s.parse()                              │
│    → Result<i64, ParseIntError>         │
│                                         │
│  .map_err(|e| ParsePosNonzeroError::ParseInt(e))│
│    → Result<i64, ParsePosNonzeroError>  │
│                                         │
│  ?                                      │
│    → i64 (or return the wrapped error)  │
│                                         │
│  The error is transformed before ?      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Conversion Helper Functions



## Cleaner Code

Instead of inline closures, use helper functions:

```rust
impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
    }

    fn from_parse_int(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  These functions convert specific       │
│  errors into the composite type.        │
│                                         │
│  Self::Creation(err)                    │
│    → Wraps CreationError                │
│                                         │
│  Self::ParseInt(err)                    │
│    → Wraps ParseIntError                │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Using Conversion Functions



## With map_err

```rust
// Instead of:
.map_err(|e| ParsePosNonzeroError::ParseInt(e))

// Use the helper:
.map_err(ParsePosNonzeroError::from_parse_int)
```



```
┌─────────────────────────────────────────┐
│                                         │
│  map_err takes a function:              │
│  fn(E) -> NewE                          │
│                                         │
│  ParsePosNonzeroError::from_parse_int   │
│  is exactly this: fn(ParseIntError) -> Self│
│                                         │
│  So you can pass it directly!           │
│  No closure needed.                     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Complete Example



## Putting It Together

```rust
impl PositiveNonzeroInteger {
    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        // Step 1: Parse string to i64
        let x: i64 = s.parse()
            .map_err(ParsePosNonzeroError::from_parse_int)?;

        // Step 2: Validate and create
        Self::new(x)
            .map_err(ParsePosNonzeroError::from_creation)
    }
}
```



--- slide ---

# Breaking It Down



## Step by Step

```
┌─────────────────────────────────────────┐
│                                         │
│  STEP 1: Parse the string               │
│                                         │
│  s.parse()                              │
│    → Result<i64, ParseIntError>         │
│                                         │
│  .map_err(from_parse_int)               │
│    → Result<i64, ParsePosNonzeroError>  │
│                                         │
│  ?                                      │
│    → i64 (or early return)              │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  STEP 2: Validate the number            │
│                                         │
│  Self::new(x)                           │
│    → Result<Self, CreationError>        │
│                                         │
│  .map_err(from_creation)                │
│    → Result<Self, ParsePosNonzeroError> │
│                                         │
│  (returned directly, no ?)              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why No ? on Last Line?



## Return Types Match

```rust
Self::new(x).map_err(ParsePosNonzeroError::from_creation)
```



```
┌─────────────────────────────────────────┐
│                                         │
│  This expression has type:              │
│  Result<Self, ParsePosNonzeroError>     │
│                                         │
│  The function returns:                  │
│  Result<Self, ParsePosNonzeroError>     │
│                                         │
│  They match! Just return directly.      │
│  No need for ? or Ok().                 │
│                                         │
│  The Result IS the return value.        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Adding the Conversion Function

You have this partial implementation:

```rust
impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
    }

    // TODO: Add another error conversion function here.
    // fn from_parse_int(???) -> Self { ??? }
}
```

And a parse function using unwrap (bad!):

```rust
fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
    let x: i64 = s.parse().unwrap();  // PANIC on error!
    Self::new(x).map_err(ParsePosNonzeroError::from_creation)
}
```



--- slide ---

# Two Tasks



## What You Need to Do

```
┌─────────────────────────────────────────┐
│                                         │
│  TASK 1: Add from_parse_int function    │
│                                         │
│  fn from_parse_int(err: ???) -> Self {  │
│      Self::ParseInt(???)                │
│  }                                      │
│                                         │
│  What type is the input parameter?      │
│  What goes inside Self::ParseInt()?     │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  TASK 2: Fix parse() to use map_err     │
│                                         │
│  Replace:                               │
│    s.parse().unwrap()                   │
│                                         │
│  With error handling using:             │
│    map_err and from_parse_int           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Look at from_creation



## Follow the Pattern

```rust
fn from_creation(err: CreationError) -> Self {
    Self::Creation(err)
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  PATTERN:                               │
│                                         │
│  fn from_XXX(err: XXXError) -> Self {   │
│      Self::XXX(err)                     │
│  }                                      │
│                                         │
│  For from_parse_int:                    │
│                                         │
│  • Input type: ParseIntError            │
│  • Variant: Self::ParseInt              │
│  • Wrap the error in the variant        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Fixing parse()



## Remove the unwrap

```rust
// BEFORE (panics on error!):
let x: i64 = s.parse().unwrap();

// AFTER (handles error properly):
let x: i64 = s.parse()
    .map_err(???)  // Convert error type
    ?;             // Propagate if error
```



```
┌─────────────────────────────────────────┐
│                                         │
│  What function converts ParseIntError   │
│  to ParsePosNonzeroError?               │
│                                         │
│  The one you're implementing!           │
│  ParsePosNonzeroError::from_parse_int   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Think It Through



## Before You Type



```
┌─────────────────────────────────────────┐
│  FOR from_parse_int:                    │
│                                         │
│  1. What type does parse() error with?  │
│     (Check std::num::ParseIntError)     │
│                                         │
│  2. What variant wraps ParseIntError?   │
│     (Look at the enum definition!)      │
│                                         │
│  3. Follow the from_creation pattern!   │
│                                         │
├─────────────────────────────────────────┤
│  FOR fixing parse():                    │
│                                         │
│  1. What does s.parse() return?         │
│                                         │
│  2. What converts ParseIntError to      │
│     ParsePosNonzeroError?               │
│                                         │
│  3. How do you call map_err with a      │
│     conversion function?                │
│                                         │
│  4. After map_err, what operator        │
│     propagates the error?               │
│                                         │
└─────────────────────────────────────────┘
```



Two changes: add function, fix parse()!

(Now go to the Editor and try it!)
