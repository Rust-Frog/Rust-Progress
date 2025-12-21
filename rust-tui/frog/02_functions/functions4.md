# Return Types

## The Problem

```
fn sale_price(price: i64) -> ? {  // <-- Missing!
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}
```

Something is missing after the `->` arrow!


--- slide ---

# Return Values

## Why Return Values?

Functions can compute something and give it back:

```
┌────────────────────────────────────────────────┐
│  INPUT              PROCESS           OUTPUT   │
│    │                  │                 │      │
│    ▼                  ▼                 ▼      │
│   51   ──────►   calculate   ──────►   48      │
│                                                │
│  fn sale_price(price: i64) -> i64              │
│     │                          │               │
│     └── Input                  └── Output      │
└────────────────────────────────────────────────┘
```

## The Syntax

```
fn function_name() -> ReturnType {
                   ^^ ^^^^^^^^^^
                   │  │
                   │  └── What type is returned
                   └── Arrow indicates return
}
```

--- slide ---

# Return Type Syntax

## Examples

```
fn get_five() -> i32 {
    5
}

fn is_positive(n: i32) -> bool {
    n > 0
}

fn greet() -> String {
    String::from("Hello!")
}
```

## Key Pattern

```
fn name(params) -> Type {
                ^^ ^^^^
                │  │
                │  └── The type of value returned
                └── Arrow is required!
}
```

--- slide ---

# Understanding the Problem

Look at the function:

```
fn sale_price(price: i64) -> {
                          ^^ ^
                          │  │
                          │  └── Opening brace
                          └── Arrow but NO TYPE!
```

## What's Returned?

Look at the function body:

```
if is_even(price) {
    price - 10    // Returns this...
} else {
    price - 3     // ...or this
}
```

Both branches return `price - something`.
What type is `price`? It's `i64`!

## Think About It

```
┌────────────────────────────────────────────────┐
│  QUESTIONS TO CONSIDER:                        │
│                                                │
│  1. What comes after the arrow `->` ?          │
│                                                │
│  2. What type is being returned?               │
│     (Hint: look at the price parameter)        │
│                                                │
│  3. What type should go between `->` and `{`?  │
└────────────────────────────────────────────────┘
```

--- slide ---

# Common Return Types

```
┌──────────┬───────────────────────────────┐
│ Type     │ Example Return Value          │
├──────────┼───────────────────────────────┤
│ i32      │ 42, -1, 0                     │
│ i64      │ Large integers                │
│ bool     │ true, false                   │
│ String   │ String::from("hello")         │
│ ()       │ Nothing (unit type)           │
└──────────┴───────────────────────────────┘
```

## The Unit Type `()`

Functions without `->` implicitly return `()`:

```
fn say_hello() {     // Returns ()
    println!("Hi");
}

fn say_hello() -> () {  // Same thing, explicit
    println!("Hi");
}
```

--- slide ---

# Key Takeaways

## Return Type Checklist

```
┌────────────────────────────────────────────────┐
│  • Arrow `->` indicates a return type          │
│  • Type goes between `->` and `{`              │
│  • Type must match what's actually returned    │
│  • No arrow = returns () (unit)                │
└────────────────────────────────────────────────┘
```

## Template

```
fn calculate(input: i32) -> OutputType {
                         ^^^^^^^^^^^^
                         Add the type here
    // body
}
```

Read the error message — it tells you what's expected!
