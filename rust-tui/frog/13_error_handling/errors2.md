# Propagating Errors



## The Tedious Pattern

When calling functions that return Result,

you often want to pass errors up:

```rust
fn do_something() -> Result<i32, Error> {
    let result = might_fail();

    match result {
        Ok(value) => {
            // continue with value...
        }
        Err(e) => return Err(e),  // Pass error up
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  This pattern is EXTREMELY common:      │
│                                         │
│  "If this fails, return the error.      │
│   Otherwise, continue with the value."  │
│                                         │
│  Writing match for every Result call    │
│  gets very tedious very fast!           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The match Explosion



## Real Code Gets Messy

```rust
fn process_order(input: &str) -> Result<Order, Error> {
    let qty = match input.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    let validated = match validate(qty) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let order = match create_order(validated) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    Ok(order)
}
```

Three operations = three verbose match blocks!



--- slide ---

# Enter: The ? Operator



## Rust's Error Propagation Magic

The `?` operator does the match for you:

```rust
fn process_order(input: &str) -> Result<Order, Error> {
    let qty = input.parse::<i32>()?;
    let validated = validate(qty)?;
    let order = create_order(validated)?;
    Ok(order)
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  expression?                            │
│            ↑                            │
│            Question mark operator       │
│                                         │
│  This single character replaces the     │
│  entire match block!                    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# What ? Does



## The Mechanics

```rust
let value = might_fail()?;
```

This is equivalent to:

```rust
let value = match might_fail() {
    Ok(v) => v,
    Err(e) => return Err(e),
};
```



```
┌─────────────────────────────────────────┐
│                                         │
│  THE ? OPERATOR:                        │
│                                         │
│  1. Evaluates the expression            │
│                                         │
│  2. If Ok(value):                       │
│     → Unwraps and gives you the value   │
│     → Continues execution               │
│                                         │
│  3. If Err(e):                          │
│     → Returns Err(e) from the function  │
│     → Execution stops here              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Visual Flow



## Success vs Error Path

```
┌─────────────────────────────────────────┐
│                                         │
│  let x = operation()?;                  │
│                                         │
│  SUCCESS PATH:         ERROR PATH:      │
│                                         │
│  operation()           operation()      │
│      ↓                     ↓            │
│  returns Ok(v)         returns Err(e)   │
│      ↓                     ↓            │
│  x = v                 return Err(e)    │
│      ↓                 (function exits!)│
│  (continues)                            │
│                                         │
│  ? extracts v OR early-returns the error│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Requirements for ?



## When You Can Use It

```
┌─────────────────────────────────────────┐
│                                         │
│  TO USE ? ON A RESULT:                  │
│                                         │
│  1. You must be in a function that      │
│     returns Result                      │
│                                         │
│  2. The error types must be compatible  │
│     (same type, or convertible)         │
│                                         │
│  CAN'T USE ? IN:                        │
│                                         │
│  • Functions returning () (nothing)     │
│  • Functions returning just T           │
│  • main() by default (but can change!)  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Error Type Must Match



## Compatibility Requirement

```rust
fn my_function() -> Result<i32, ParseIntError> {
    let x = "42".parse::<i32>()?;
    //                        ↑
    // parse() returns Result<i32, ParseIntError>
    // Function returns Result<i32, ParseIntError>
    // Error types MATCH! ✓

    Ok(x)
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The error from ?                       │
│  must match (or convert to)             │
│  the function's error type.             │
│                                         │
│  If parse() → Err(ParseIntError)        │
│  And function returns Result<_, ParseIntError>│
│  Then ? can propagate it directly!      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Using ? with parse()



## A Common Use Case

String parsing is a classic Result operation:

```rust
let input = "42";
let number: i32 = input.parse()?;
```



```
┌─────────────────────────────────────────┐
│                                         │
│  "42".parse::<i32>()                    │
│                                         │
│  Returns: Result<i32, ParseIntError>    │
│                                         │
│  On success: Ok(42)                     │
│  On failure: Err(ParseIntError)         │
│                                         │
│  With ?:                                │
│  • Success → number = 42                │
│  • Failure → function returns error     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Chaining Operations



## Multiple ? In Sequence

```rust
fn calculate(input: &str) -> Result<i32, ParseIntError> {
    let a = input.parse::<i32>()?;  // First ?
    let b = (a * 2).to_string().parse::<i32>()?;  // Second ?
    Ok(b + 10)
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Each ? is a potential exit point.      │
│                                         │
│  If ANY fails, the function returns     │
│  immediately with that error.           │
│                                         │
│  Only if ALL succeed does execution     │
│  reach Ok(b + 10).                      │
│                                         │
│  Think of ? as a "checkpoint":          │
│  "Pass this to continue, fail = exit"   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Two Ways to Handle Errors



## Match vs ?

```rust
// APPROACH 1: Explicit match
fn total_cost(qty: &str) -> Result<i32, ParseIntError> {
    let qty = match qty.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    Ok(qty * 5 + 1)
}

// APPROACH 2: The ? operator
fn total_cost(qty: &str) -> Result<i32, ParseIntError> {
    let qty = qty.parse::<i32>()?;
    Ok(qty * 5 + 1)
}
```

Both do the same thing. ? is just shorter!



--- slide ---

# When Match Is Better



## ? Isn't Always Right

Sometimes explicit match is clearer:

```rust
// When you want to HANDLE the error, not propagate
fn try_with_fallback(s: &str) -> i32 {
    match s.parse::<i32>() {
        Ok(n) => n,
        Err(_) => 0,  // Use default instead of failing
    }
}

// When you want to TRANSFORM the error
fn custom_error(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(e) => Err(format!("Parse failed: {}", e)),
    }
}
```



--- slide ---

# Your Exercise



## Using ? for Error Propagation

You have this code:

```rust
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  THE PROBLEM:                           │
│                                         │
│  qty is Result<i32, ParseIntError>      │
│  not i32!                               │
│                                         │
│  You can't multiply a Result!           │
│  qty * cost_per_item doesn't work.      │
│                                         │
│  You need to handle the Result first.   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Parse Problem



## What parse() Returns

```rust
let qty = item_quantity.parse::<i32>();
// qty is Result<i32, ParseIntError>, NOT i32!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  item_quantity could be:                │
│                                         │
│  "42"        → parse succeeds → Ok(42)  │
│  "abc"       → parse fails → Err(...)   │
│  "99999999999" → too big → Err(...)     │
│                                         │
│  parse() can fail, so it returns Result.│
│                                         │
│  You must extract the i32 from Ok       │
│  OR propagate the error.                │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Two Solutions



## Both Work

```
┌─────────────────────────────────────────┐
│                                         │
│  SOLUTION 1: Explicit match             │
│                                         │
│  let qty = match item_quantity.parse()  │
│  {                                      │
│      Ok(n) => n,                        │
│      Err(e) => return Err(e),           │
│  };                                     │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  SOLUTION 2: The ? operator             │
│                                         │
│  let qty = item_quantity.parse::<i32>()?;│
│                                         │
│  Much shorter! Same behavior.           │
│                                         │
└─────────────────────────────────────────┘
```

The exercise mentions "one is a lot shorter!"



--- slide ---

# Think It Through



## Before You Type



```
┌─────────────────────────────────────────┐
│  ASK YOURSELF:                          │
│                                         │
│  1. What does parse() return?           │
│     (Just i32? Or Result<i32, ...>?)    │
│                                         │
│  2. What does the function return?      │
│     (Check the signature!)              │
│                                         │
│  3. Do the error types match?           │
│     (parse error = function error?)     │
│                                         │
│  4. If parse fails, what should happen? │
│     (Continue? Or return error?)        │
│                                         │
│  5. What operator propagates errors     │
│     automatically?                      │
│                                         │
│  6. Where does ? go?                    │
│     (After the expression!)             │
│                                         │
└─────────────────────────────────────────┘
```



One small addition makes it work!

(Now go to the Editor and try it!)
