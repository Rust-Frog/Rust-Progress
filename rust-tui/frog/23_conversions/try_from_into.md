# TryFrom: Fallible Conversions



## When Conversion Can Fail

`From` is for infallible conversions.

`TryFrom` is for conversions that might fail!

```rust
// From: always succeeds
let big: i64 = i64::from(5i32);  // OK!

// TryFrom: might fail
let small: i8 = i8::try_from(500i32)?;  // Error!
// 500 doesn't fit in i8!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  From<T>:     fn from(T) -> Self        │
│  TryFrom<T>:  fn try_from(T) -> Result  │
│                                         │
│  Use TryFrom when conversion might fail!│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## RGB Color from Numbers

```rust
struct Color {
    red: u8,    // 0-255
    green: u8,  // 0-255
    blue: u8,   // 0-255
}
```

Input is `i16` which can be -32768 to 32767.

Only 0-255 is valid for RGB!



```
┌─────────────────────────────────────────┐
│                                         │
│  (183, 65, 14)   → Ok(Color)           │
│  (256, 0, 0)     → Err (too big!)       │
│  (-1, 0, 0)      → Err (negative!)      │
│                                         │
│  i16 range:  -32768 to 32767            │
│  u8 range:   0 to 255                   │
│                                         │
│  Must validate before converting!       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Implementing TryFrom



## The Trait Signature

```rust
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        // Validate and convert
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Three implementations needed:          │
│                                         │
│  1. From tuple: (i16, i16, i16)         │
│  2. From array: [i16; 3]                │
│  3. From slice: &[i16]                  │
│                                         │
│  Same validation logic, different input!│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Validation Logic



## Check Each Value

```rust
fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
    let (r, g, b) = tuple;

    // Check if values are in valid range
    if r < 0 || r > 255 || g < 0 || g > 255 || b < 0 || b > 255 {
        return Err(IntoColorError::IntConversion);
    }

    Ok(Color {
        red: r as u8,
        green: g as u8,
        blue: b as u8,
    })
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Valid range: 0 <= value <= 255         │
│                                         │
│  After validation, `as u8` is safe!     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Using u8::try_from



## A Cleaner Approach

```rust
// Instead of manual range check:
let red = u8::try_from(r)
    .map_err(|_| IntoColorError::IntConversion)?;
```



```
┌─────────────────────────────────────────┐
│                                         │
│  u8::try_from(i16) automatically:       │
│  • Returns Ok(u8) if 0-255              │
│  • Returns Err if out of range          │
│                                         │
│  We convert the error with map_err      │
│  and use ? to early return on error.    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Slice Implementation



## Extra Validation Needed

For slices, you must also check the LENGTH!

```rust
impl TryFrom<&[i16]> for Color {
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        // Check length first!
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }

        // Then validate values...
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Tuple/Array: length is known (3)       │
│  Slice: length could be anything!       │
│                                         │
│  [0, 0]          → Err(BadLen)          │
│  [0, 0, 0, 0]    → Err(BadLen)          │
│  [0, 0, 0]       → check values...      │
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
│  1. What's the valid range for RGB?     │
│     (0 to 255)                          │
│                                         │
│  2. What error for out-of-range values? │
│     (IntoColorError::IntConversion)     │
│                                         │
│  3. What error for wrong slice length?  │
│     (IntoColorError::BadLen)            │
│                                         │
│  4. After validation, how to convert    │
│     i16 to u8?                          │
│     (value as u8)                       │
│                                         │
└─────────────────────────────────────────┘
```



TryFrom: safe conversion with error handling!

(Go try it in the Editor!)
