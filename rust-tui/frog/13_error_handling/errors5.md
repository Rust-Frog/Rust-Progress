# Multiple Error Types



## The Mixing Problem

What if your function can fail in different ways?

```rust
fn process(s: &str) -> Result<i64, ???> {
    let n: i64 = s.parse()?;        // ParseIntError
    let pos = Positive::new(n)?;    // CreationError
    Ok(pos.value())
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Two operations, two DIFFERENT errors:  │
│                                         │
│  parse() → Result<i64, ParseIntError>   │
│  new()   → Result<Positive, CreationError>│
│                                         │
│  What type should process() return?     │
│  ? needs ONE error type to propagate!   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Type Mismatch



## Why ? Fails

```rust
fn process(s: &str) -> Result<i64, ParseIntError> {
    let n: i64 = s.parse()?;        // OK - ParseIntError
    let pos = Positive::new(n)?;    // ERROR! CreationError != ParseIntError
    Ok(pos.value())
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  COMPILER ERROR:                        │
│                                         │
│  "? couldn't convert the error to       │
│   ParseIntError"                        │
│                                         │
│  CreationError is not ParseIntError!    │
│  ? can't magically unify them.          │
│                                         │
│  We need a type that can hold EITHER.   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Trait Objects



## A Type That Holds "Anything"

Rust has a concept: trait objects.

"I don't care what type, as long as it implements this trait."

```rust
Box<dyn SomeTrait>
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Box<dyn SomeTrait>                     │
│  ↑    ↑   ↑                             │
│  │    │   └─ The trait it must implement│
│  │    └─ "dynamic" - type erased        │
│  └─ Heap-allocated box                  │
│                                         │
│  This can hold ANY type that implements │
│  SomeTrait, decided at runtime.         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Error Trait



## What All Errors Share

Rust has a standard `Error` trait:

```rust
use std::error::Error;
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The Error trait means:                 │
│  "This type represents an error"        │
│                                         │
│  Many types implement it:               │
│                                         │
│  • ParseIntError    ✓ impl Error        │
│  • IoError          ✓ impl Error        │
│  • Custom errors    ✓ can impl Error    │
│                                         │
│  If we say "any Error", we can hold     │
│  ANY of these types!                    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Box<dyn Error>



## The "Any Error" Type

```rust
use std::error::Error;

fn process(s: &str) -> Result<i64, Box<dyn Error>> {
    let n: i64 = s.parse()?;        // ParseIntError → Box<dyn Error>
    let pos = Positive::new(n)?;    // CreationError → Box<dyn Error>
    Ok(pos.value())
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Box<dyn Error> means:                  │
│                                         │
│  "A boxed value of ANY type that        │
│   implements the Error trait"           │
│                                         │
│  Both ParseIntError and CreationError   │
│  implement Error, so both can be        │
│  converted to Box<dyn Error>!           │
│                                         │
│  ? automatically does this conversion.  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# How Conversion Works



## Automatic with ?

```rust
// ParseIntError implements Error
// CreationError implements Error

let n: i64 = s.parse()?;
// parse() returns Result<i64, ParseIntError>
// ? converts ParseIntError → Box<dyn Error>

let pos = Positive::new(n)?;
// new() returns Result<Positive, CreationError>
// ? converts CreationError → Box<dyn Error>
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The ? operator uses the From trait     │
│  to convert errors automatically.       │
│                                         │
│  Any type implementing Error can be     │
│  converted into Box<dyn Error>.         │
│                                         │
│  You don't have to do anything special! │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Making Custom Errors Work



## Implementing Error Trait

For your own errors to work with Box<dyn Error>:

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum CreationError {
    Negative,
    Zero,
}

// Required: Display trait
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CreationError::Negative => write!(f, "number is negative"),
            CreationError::Zero => write!(f, "number is zero"),
        }
    }
}

// Then: impl Error (can be empty!)
impl Error for CreationError {}
```



--- slide ---

# The Requirements



## What Error Needs

```
┌─────────────────────────────────────────┐
│                                         │
│  To implement std::error::Error:        │
│                                         │
│  1. Must implement Debug                │
│     #[derive(Debug)]                    │
│                                         │
│  2. Must implement Display              │
│     impl fmt::Display for MyError       │
│     (provides human-readable message)   │
│                                         │
│  3. Then impl Error can be empty        │
│     impl Error for MyError {}           │
│                                         │
│  The Error trait itself has default     │
│  implementations for its methods.       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Trade-offs of Box<dyn Error>



## Pros and Cons

```
┌─────────────────────────────────────────┐
│                                         │
│  ADVANTAGES:                            │
│                                         │
│  • Easy - just use Box<dyn Error>       │
│  • Flexible - accepts any error         │
│  • Good for applications/binaries       │
│  • Quick prototyping                    │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  DISADVANTAGES:                         │
│                                         │
│  • Type erased - can't match on variant │
│  • Heap allocation (Box)                │
│  • Not great for libraries              │
│  • Callers can't handle specific errors │
│                                         │
│  Best for: applications, main()         │
│  Not best for: library APIs             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Using in main()



## The Common Pattern

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = "42";
    let n: i64 = input.parse()?;           // ParseIntError OK
    let pos = Positive::new(n)?;           // CreationError OK
    println!("Result: {:?}", pos);
    Ok(())
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  main() -> Result<(), Box<dyn Error>>   │
│                                         │
│  This is the MOST FLEXIBLE main():      │
│                                         │
│  • Can use ? with ANY error type        │
│  • No need to specify exact error       │
│  • All errors print on failure          │
│                                         │
│  Perfect for command-line tools!        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Adding Return Type to main()

The code has:

```rust
fn main() {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;  // ParseIntError
    println!("{:?}", PositiveNonzeroInteger::new(x)?);  // CreationError
    Ok(())
}
```

Two different error types with ?!



--- slide ---

# The Problem



## Two Error Types

```
┌─────────────────────────────────────────┐
│                                         │
│  ERROR 1: parse() returns               │
│    Result<i64, ParseIntError>           │
│                                         │
│  ERROR 2: new() returns                 │
│    Result<PositiveNonzeroInteger, CreationError>│
│                                         │
│  Both use ? in main().                  │
│  main() needs a return type that        │
│  can hold EITHER error!                 │
│                                         │
│  What type works for both?              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Hint



## Read the Exercise Comment

```rust
// TODO: Add the correct return type `Result<(), Box<dyn ???>>`.
// What can we use to describe both errors?
// Is there a trait which both errors implement?
```



```
┌─────────────────────────────────────────┐
│                                         │
│  CLUES:                                 │
│                                         │
│  • Return type is Result<(), Box<dyn ?>>│
│  • Box<dyn ???> - what goes in ???      │
│  • "trait which both errors implement"  │
│                                         │
│  Both ParseIntError and CreationError   │
│  implement... what trait?               │
│                                         │
│  The trait for errors!                  │
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
│  1. What trait do all error types       │
│     implement? (5 letters!)             │
│                                         │
│  2. What's the syntax for a trait object│
│     that can be any Error?              │
│     Box<dyn ???>                        │
│                                         │
│  3. What's the full return type?        │
│     Result<(), Box<dyn ???>>            │
│                                         │
│  4. Is there anything else to change    │
│     in main's body?                     │
│     (Hint: Ok(()) is already there!)    │
│                                         │
│  5. Do you need to import anything?     │
│     (Check what's already imported!)    │
│                                         │
└─────────────────────────────────────────┘
```



Just add the return type to main()!

(Now go to the Editor and try it!)
