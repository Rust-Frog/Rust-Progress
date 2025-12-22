# Lifetimes in Practice



## Scope and Validity

Understanding lifetimes means understanding scope:

```rust
{
    let x = 5;      // x comes into scope
    // x is valid here
}                   // x goes out of scope
// x is no longer valid!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Variables live within their SCOPE.     │
│                                         │
│  When a block { } ends:                 │
│  • Variables in that block are dropped  │
│  • References to them become invalid    │
│                                         │
│  This is automatic memory management!   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Nested Scopes



## Different Lifetimes

```rust
let outer = String::from("outer");
{
    let inner = String::from("inner");
    // Both outer and inner valid here
}   // inner dropped here
// Only outer valid here
```



```
┌─────────────────────────────────────────┐
│                                         │
│  TIMELINE:                              │
│                                         │
│  outer ─────────────────────────────►   │
│         │                         │     │
│         │  inner ──────►          │     │
│         │        │      │         │     │
│         │        └──────┘         │     │
│         │        inner's scope    │     │
│         └─────────────────────────┘     │
│                outer's scope            │
│                                         │
│  inner has a SHORTER lifetime!          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Dangling Reference Problem



## When References Outlive Data

```rust
let result;           // Declare result
{
    let s = String::from("hello");
    result = &s;      // Borrow s
}                     // s is DROPPED!
println!("{}", result); // result points to nothing!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  COMPILER ERROR:                        │
│                                         │
│  "`s` does not live long enough"        │
│                                         │
│  The reference result tries to live     │
│  longer than the data it refers to!     │
│                                         │
│  result: ─────────────────────────►     │
│  s:      ───────────►                   │
│                     ↑                   │
│                     s dropped here      │
│                                         │
│  After s is dropped, result is INVALID! │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The longest Function



## Lifetime Constraints

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Return lives as long as BOTH inputs
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The return type &'a str means:         │
│                                         │
│  "The returned reference is valid for   │
│   lifetime 'a"                          │
│                                         │
│  Since both x and y are &'a str:        │
│                                         │
│  'a = the SHORTER of x's and y's        │
│       actual lifetimes                  │
│                                         │
│  The return cannot outlive EITHER input!│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# A Problematic Usage



## Scope Mismatch

```rust
let string1 = String::from("long string");
let result;
{
    let string2 = String::from("xyz");
    result = longest(&string1, &string2);
}   // string2 dropped here!
println!("{result}");  // Is result still valid?
```



```
┌─────────────────────────────────────────┐
│                                         │
│  string1: ───────────────────────────►  │
│  string2:      ──────────►              │
│                          ↑              │
│                          dropped        │
│                                         │
│  result could be &string1 or &string2   │
│  Rust assumes worst case: &string2      │
│                                         │
│  So result can't be used after string2  │
│  is dropped!                            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Compiler's Logic



## Conservative Safety

```
┌─────────────────────────────────────────┐
│                                         │
│  RUST'S REASONING:                      │
│                                         │
│  longest() might return &string2.       │
│  string2 lives only in the inner block. │
│  result = longest(...) happens inside.  │
│  println uses result OUTSIDE the block. │
│                                         │
│  IF result held &string2:               │
│  → Printing would use freed memory!     │
│                                         │
│  Rust CANNOT prove result is safe,      │
│  so it rejects the code.                │
│                                         │
│  Even if string1 is always longer!      │
│  (Rust checks at compile time, not      │
│   runtime)                              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Solutions



## Two Ways to Fix

```
┌─────────────────────────────────────────┐
│                                         │
│  OPTION 1: Use result INSIDE the block  │
│                                         │
│  {                                      │
│      let string2 = ...;                 │
│      result = longest(&string1, &string2);│
│      println!("{result}");  // Move here!│
│  }                                      │
│                                         │
│  Result is used while string2 is alive. │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  OPTION 2: Extend string2's lifetime    │
│                                         │
│  let string2 = ...;  // Move outside!   │
│  let result = longest(&string1, &string2);│
│  println!("{result}");                  │
│                                         │
│  string2 lives as long as result needs. │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Moving Code for Validity

You have:

```rust
let string1 = String::from("long string is long");
let result;
{
    let string2 = String::from("xyz");
    result = longest(&string1, &string2);
}
println!("The longest string is '{result}'");
```

The compiler complains about lifetimes!



--- slide ---

# Understanding the Problem



## Why It Fails

```
┌─────────────────────────────────────────┐
│                                         │
│  LINE BY LINE:                          │
│                                         │
│  let string1 = ...;    // Lives to end  │
│  let result;           // Declared      │
│  {                                      │
│      let string2 = ...; // Inner scope  │
│      result = longest(...);             │
│  }  // string2 DROPPED!                 │
│  println!("{result}"); // Uses result   │
│                                         │
│  result might reference string2         │
│  but string2 is already gone!           │
│                                         │
│  The hint says: "move one line"         │
│  Which line fixes this?                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Fix



## Which Line to Move

```
┌─────────────────────────────────────────┐
│                                         │
│  TASK: Move the println! INSIDE the     │
│        inner block.                     │
│                                         │
│  BEFORE:                                │
│                                         │
│  {                                      │
│      let string2 = ...;                 │
│      result = longest(...);             │
│  }                                      │
│  println!("{result}");  // Outside!     │
│                                         │
│  AFTER:                                 │
│                                         │
│  {                                      │
│      let string2 = ...;                 │
│      result = longest(...);             │
│      println!("{result}");  // Inside!  │
│  }                                      │
│                                         │
│  Now result is used while string2 lives!│
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
│  1. What is the error about?            │
│     (string2 doesn't live long enough)  │
│                                         │
│  2. When does string2 get dropped?      │
│     (At the closing brace of inner block)│
│                                         │
│  3. Where is result used?               │
│     (In println, after the inner block) │
│                                         │
│  4. What does "move one line" mean?     │
│     (Move println inside the block)     │
│                                         │
│  5. Why does this fix the problem?      │
│     (result is used while string2 lives)│
│                                         │
│  6. Could you move string2 out instead? │
│     (Yes, but the hint says move one    │
│      line, and println is easier)       │
│                                         │
└─────────────────────────────────────────┘
```



Move the println inside the inner block!

(Now go to the Editor and try it!)
