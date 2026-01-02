# FromStr: Parsing with Error Handling



## The parse() Method

You've seen this pattern:

```rust
let num: i32 = "42".parse().unwrap();
```

`parse()` uses the `FromStr` trait under the hood!



```
┌─────────────────────────────────────────┐
│                                         │
│  "42".parse::<i32>()                    │
│       ↓                                 │
│  <i32 as FromStr>::from_str("42")       │
│       ↓                                 │
│  Result<i32, ParseIntError>             │
│                                         │
│  FromStr is what makes .parse() work!   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# FromStr vs From



## Errors Instead of Defaults

`From<&str>` (previous exercise):
- Returns the type directly
- Uses default on failure

`FromStr`:
- Returns `Result<Self, Error>`
- Proper error handling!



```
┌─────────────────────────────────────────┐
│                                         │
│  From<&str>:                            │
│  fn from(s: &str) -> Self               │
│  (Can't fail - use default)             │
│                                         │
│  FromStr:                               │
│  fn from_str(s: &str) -> Result<Self, E>│
│  (CAN fail - return error)              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Implementing FromStr



## The Trait Signature

```rust
use std::str::FromStr;

impl FromStr for Person {
    type Err = ParsePersonError;  // Your error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse and return Ok(person) or Err(error)
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  type Err = ... declares the error type │
│                                         │
│  Return:                                │
│  • Ok(Person { ... }) on success        │
│  • Err(ParsePersonError::...) on fail   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Error Types



## ParsePersonError

```rust
enum ParsePersonError {
    BadLen,              // Wrong number of fields
    NoName,              // Empty name
    ParseInt(ParseIntError),  // Age parse failed
}
```

Each error case has its own variant!



```
┌─────────────────────────────────────────┐
│                                         │
│  "Mark"     → Err(BadLen)               │
│  ",20"      → Err(NoName)               │
│  "Mark,abc" → Err(ParseInt(...))        │
│  "Mark,20"  → Ok(Person { ... })        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Parsing Steps



## Same Logic, Different Returns

```
┌─────────────────────────────────────────┐
│                                         │
│  1. Split on comma                      │
│     if parts.len() != 2:                │
│        return Err(BadLen)               │
│                                         │
│  2. Get name                            │
│     if name.is_empty():                 │
│        return Err(NoName)               │
│                                         │
│  3. Parse age                           │
│     if parse fails:                     │
│        return Err(ParseInt(e))          │
│                                         │
│  4. Return Ok(Person { name, age })     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Wrapping Parse Errors



## The ? Operator with map_err

```rust
// Parse age, converting error type
let age = parts[1]
    .parse::<u8>()
    .map_err(ParsePersonError::ParseInt)?;
//  ↑                                   ↑
//  Convert ParseIntError to our type   Early return on Err
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .parse::<u8>() returns:                │
│  Result<u8, ParseIntError>              │
│                                         │
│  But we need:                           │
│  Result<_, ParsePersonError>            │
│                                         │
│  .map_err() converts the error type!    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Using Your Implementation



## The parse() Method Works!

```rust
// After implementing FromStr:
let p = "Mark,20".parse::<Person>();
// Returns: Result<Person, ParsePersonError>

let p = p.unwrap();
// Person { name: "Mark", age: 20 }
```



--- slide ---

# Think It Through



## Before You Type



```
┌─────────────────────────────────────────┐
│  ASK YOURSELF:                          │
│                                         │
│  1. What error for wrong field count?   │
│     (ParsePersonError::BadLen)          │
│                                         │
│  2. What error for empty name?          │
│     (ParsePersonError::NoName)          │
│                                         │
│  3. How to wrap parse error?            │
│     (.map_err(ParsePersonError::ParseInt))│
│                                         │
│  4. What's the success return?          │
│     (Ok(Person { name, age }))          │
│                                         │
└─────────────────────────────────────────┘
```



FromStr enables .parse() with proper errors!

(Go try it in the Editor!)
