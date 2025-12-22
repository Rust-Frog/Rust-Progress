# Iterators and Results



## Combining Iteration with Error Handling

When operations can fail during iteration:

```rust
let strings = ["1", "2", "three", "4"];
let results: Vec<Result<i32, _>> = strings
    .iter()
    .map(|s| s.parse::<i32>())
    .collect();
// [Ok(1), Ok(2), Err(...), Ok(4)]
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Each parse() can succeed or fail.      │
│                                         │
│  We get a Vec of Results:               │
│  • Ok(1), Ok(2) - parsed successfully   │
│  • Err(...) - "three" failed to parse   │
│  • Ok(4) - parsed successfully          │
│                                         │
│  But what if we want Result<Vec<_>>?    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Two Collection Patterns



## Vec<Result> vs Result<Vec>

```rust
// Pattern 1: Vec<Result<T, E>>
// Keep all results, successes AND failures
[Ok(1), Ok(2), Err(e), Ok(4)]

// Pattern 2: Result<Vec<T>, E>
// All succeed → Ok(vec)
// Any fails → Err(first_error)
Ok([1, 2, 4])  // or Err(e) if any failed
```



```
┌─────────────────────────────────────────┐
│                                         │
│  CHOOSE BASED ON NEED:                  │
│                                         │
│  Vec<Result>:                           │
│  • Process valid items, report errors   │
│  • "Show me everything that happened"   │
│                                         │
│  Result<Vec>:                           │
│  • All-or-nothing operation             │
│  • "Either all work, or give me error"  │
│                                         │
│  Both use .collect(), different types!  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Collecting into Result<Vec>



## The Magic of Type Inference

```rust
let numbers = ["1", "2", "3"];

// Collect into Result<Vec<i32>, ParseIntError>
let result: Result<Vec<i32>, _> = numbers
    .iter()
    .map(|s| s.parse::<i32>())
    .collect();

// Ok([1, 2, 3]) if all succeed
// Err(...) if ANY fail
```



```
┌─────────────────────────────────────────┐
│                                         │
│  When you collect Iterator<Item=Result> │
│  into Result<Vec<T>, E>:                │
│                                         │
│  • If all Ok → Result contains Vec      │
│  • If any Err → Returns first Err       │
│                                         │
│  Rust's collect() is smart enough to    │
│  handle this automatically!             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The divide Function



## Custom Error Types

```rust
enum DivisionError {
    DivideByZero,
    IntegerOverflow,
    NotDivisible,
}

fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    // Check for errors, return Ok or Err
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Three possible errors:                 │
│                                         │
│  DivideByZero: b == 0                   │
│  IntegerOverflow: i64::MIN / -1         │
│  NotDivisible: a % b != 0               │
│                                         │
│  Check each condition, return the       │
│  appropriate error or Ok(a / b).        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Implementing divide



## Order of Checks

```rust
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    // 1. Check divide by zero first
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }

    // 2. Check overflow (special case)
    if a == i64::MIN && b == -1 {
        return Err(DivisionError::IntegerOverflow);
    }

    // 3. Check divisibility
    if a % b != 0 {
        return Err(DivisionError::NotDivisible);
    }

    // 4. All good!
    Ok(a / b)
}
```



--- slide ---

# result_with_list



## Collecting into Result<Vec>

```rust
fn result_with_list() -> Result<Vec<i64>, DivisionError> {
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers
        .into_iter()
        .map(|n| divide(n, 27));

    // Collect: Iterator<Result> → Result<Vec>
    division_results.collect()
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Return type: Result<Vec<i64>, DivisionError>│
│                                         │
│  .collect() gathers the results:        │
│  • All Ok → Ok(vec![1, 11, 1426, 3])    │
│  • Any Err → Err(that_error)            │
│                                         │
│  Add return type and return the collect!│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# list_of_results



## Collecting into Vec<Result>

```rust
fn list_of_results() -> Vec<Result<i64, DivisionError>> {
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers
        .into_iter()
        .map(|n| divide(n, 27));

    // Collect: Iterator<Result> → Vec<Result>
    division_results.collect()
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Return type: Vec<Result<i64, DivisionError>>│
│                                         │
│  Each Result stays separate:            │
│  [Ok(1), Ok(11), Ok(1426), Ok(3)]       │
│                                         │
│  Just a different return type!          │
│  Same .collect() call, different result.│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Three Functions

```
┌─────────────────────────────────────────┐
│                                         │
│  1. IMPLEMENT divide()                  │
│                                         │
│  Check these conditions in order:       │
│  • b == 0 → DivideByZero                │
│  • a == i64::MIN && b == -1 → Overflow  │
│  • a % b != 0 → NotDivisible            │
│  • Otherwise → Ok(a / b)                │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  2. FIX result_with_list()              │
│                                         │
│  Add return type: Result<Vec<i64>, ...> │
│  Return the collected results           │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  3. FIX list_of_results()               │
│                                         │
│  Add return type: Vec<Result<i64, ...>> │
│  Return the collected results           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Think It Through



## Before You Type



```
┌─────────────────────────────────────────┐
│  FOR divide():                          │
│                                         │
│  1. What causes DivideByZero?           │
│     (b == 0)                            │
│                                         │
│  2. What causes IntegerOverflow?        │
│     (i64::MIN / -1 would exceed i64::MAX)│
│                                         │
│  3. What causes NotDivisible?           │
│     (a % b != 0, has remainder)         │
│                                         │
│  FOR result_with_list:                  │
│                                         │
│  4. What return type wraps Vec in Result?│
│     (Result<Vec<i64>, DivisionError>)   │
│                                         │
│  FOR list_of_results:                   │
│                                         │
│  5. What return type is Vec of Results? │
│     (Vec<Result<i64, DivisionError>>)   │
│                                         │
└─────────────────────────────────────────┘
```



Implement divide and add return types!

(Now go to the Editor and try it!)
