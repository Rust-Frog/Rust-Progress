# Rust Has TWO String Types



## This Confuses Everyone

Most languages have one string type.

Rust has TWO. And they're DIFFERENT.



```
┌─────────────────────────────────────────┐
│                                         │
│  1. String   (capital S)                │
│     → Owned, growable, heap-allocated   │
│                                         │
│  2. &str     (pronounced "string slice")│
│     → Borrowed, fixed view into bytes   │
│                                         │
└─────────────────────────────────────────┘
```



This distinction is FUNDAMENTAL to Rust.

Understanding it unlocks so much!



--- slide ---

# Why Two Types?



## Ownership Again!

Remember ownership? It applies to strings too.

```
┌─────────────────────────────────────────┐
│                                         │
│  String = OWNS its data                 │
│                                         │
│    • Lives on the heap                  │
│    • Can grow, shrink, modify           │
│    • Gets dropped when owner goes away  │
│    • Only ONE owner at a time           │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  &str = BORROWS data from somewhere     │
│                                         │
│    • Just a view (pointer + length)     │
│    • Can't modify the content           │
│    • Doesn't own anything               │
│    • Can have MANY borrows at once      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# A Mental Model



## String vs &str Visualized

```
┌─────────────────────────────────────────┐
│                                         │
│  String s = String::from("hello")       │
│                                         │
│  STACK (s):        HEAP:                │
│  ┌──────────┐      ┌───┬───┬───┬───┬───┐│
│  │ ptr ─────┼─────▶│ h │ e │ l │ l │ o ││
│  │ len: 5   │      └───┴───┴───┴───┴───┘│
│  │ cap: 5   │                           │
│  └──────────┘                           │
│                                         │
│  String OWNS this heap data.            │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  &str slice = "hello"                   │
│                                         │
│  STACK:            READ-ONLY MEMORY:    │
│  ┌──────────┐      ┌───┬───┬───┬───┬───┐│
│  │ ptr ─────┼─────▶│ h │ e │ l │ l │ o ││
│  │ len: 5   │      └───┴───┴───┴───┴───┘│
│  └──────────┘                           │
│                                         │
│  &str just POINTS to data somewhere.    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# String Literals Are &str



## The Surprising Truth

When you write a string in quotes:

```rust
let s = "hello";
```

This is NOT a `String`. It's a `&str`!



```
┌─────────────────────────────────────────┐
│                                         │
│  "hello"  ← This is type &str           │
│                                         │
│  The text "hello" is baked into your    │
│  compiled program binary.               │
│                                         │
│  It exists in READ-ONLY memory.         │
│  You can't change it.                   │
│  You don't own it.                      │
│  You're just borrowing a view of it.    │
│                                         │
│  Technically: &'static str              │
│  (lives for entire program lifetime)    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Type Mismatch Problem



## When Return Types Don't Match

Consider this function:

```rust
fn get_greeting() -> String {
    "hello"  // This is &str, not String!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  COMPILER ERROR!                        │
│                                         │
│  expected `String`, found `&str`        │
│                                         │
│  The function signature says: -> String │
│  But you're returning: "hello" (&str)   │
│                                         │
│  These are DIFFERENT types.             │
│  Rust won't automatically convert.      │
│                                         │
└─────────────────────────────────────────┘
```



You need to EXPLICITLY convert!



--- slide ---

# Converting &str to String



## Three Ways to Do It

Rust gives you multiple ways to create a `String`

from a `&str`:

```rust
// Method 1: String::from()
let s = String::from("hello");

// Method 2: .to_string()
let s = "hello".to_string();

// Method 3: .to_owned()
let s = "hello".to_owned();
```



```
┌─────────────────────────────────────────┐
│                                         │
│  All three do the same thing:           │
│                                         │
│  1. Allocate space on the heap          │
│  2. Copy the bytes from &str            │
│  3. Return an owned String              │
│                                         │
│  Choose whichever reads best to you!    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# String::from()



## The Explicit Constructor

```rust
let s = String::from("hello");
```



```
┌─────────────────────────────────────────┐
│                                         │
│  String::from("hello")                  │
│  ↑       ↑      ↑                       │
│  Type   Method  Input (&str)            │
│                                         │
│  This says:                             │
│  "Create a String FROM this &str"       │
│                                         │
│  Very explicit about what's happening.  │
│  Common in Rust code.                   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# .to_string()



## The Method Call Way

```rust
let s = "hello".to_string();
```



```
┌─────────────────────────────────────────┐
│                                         │
│  "hello".to_string()                    │
│     ↑        ↑                          │
│   &str     Method                       │
│                                         │
│  This says:                             │
│  "Convert this TO a String"             │
│                                         │
│  Reads very naturally.                  │
│  Works on anything that implements      │
│  the Display trait.                     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# .to_owned()



## The Ownership Perspective

```rust
let s = "hello".to_owned();
```



```
┌─────────────────────────────────────────┐
│                                         │
│  "hello".to_owned()                     │
│     ↑        ↑                          │
│  borrowed   "give me an owned version"  │
│                                         │
│  This says:                             │
│  "I have a borrowed thing (&str)        │
│   and I want TO OWN it (String)"        │
│                                         │
│  Emphasizes the ownership transfer.     │
│  Very Rust-idiomatic thinking.          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Which Should You Use?



## It's Mostly Style

```
┌─────────────────────────────────────────┐
│                                         │
│  String::from("...")                    │
│    → Most explicit                      │
│    → Clear you're constructing a String │
│    → Common in examples/docs            │
│                                         │
│  "...".to_string()                      │
│    → Reads naturally                    │
│    → "Convert this to a String"         │
│    → Common in practice                 │
│                                         │
│  "...".to_owned()                       │
│    → Emphasizes ownership concept       │
│    → "I want to own this borrowed data" │
│    → Common in generic code             │
│                                         │
│  Pick one that makes sense for context. │
│  All are correct!                       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Not Auto-Convert?



## Rust Is Explicit

Many languages would just convert automatically.

Rust doesn't. Why?



```
┌─────────────────────────────────────────┐
│                                         │
│  CONVERSION HAS A COST:                 │
│                                         │
│  &str → String means:                   │
│    • Heap allocation (memory)           │
│    • Byte copying (time)                │
│                                         │
│  Rust wants you to SEE this cost.       │
│  No hidden allocations!                 │
│  No surprise performance hits!          │
│                                         │
│  When you write String::from("hello")   │
│  you KNOW you're allocating.            │
│                                         │
│  This explicitness prevents bugs        │
│  and helps you write efficient code.    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Real World Analogy



## Borrowing vs Owning

```
┌─────────────────────────────────────────┐
│                                         │
│  &str is like READING a library book    │
│                                         │
│    • You can look at the content        │
│    • You can't modify it                │
│    • You don't own it                   │
│    • The library owns it                │
│    • You must return it eventually      │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  String is like BUYING your own copy    │
│                                         │
│    • You own the book                   │
│    • You can write in the margins       │
│    • You can keep it forever            │
│    • You can throw it away when done    │
│    • It costs money (allocation)        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Fixing the Return Type Mismatch

You have a function that should return `String`:

```rust
fn current_favorite_color() -> String {
    "blue"
}
```

But `"blue"` is a `&str`, not a `String`!



```
┌─────────────────────────────────────────┐
│                                         │
│  The function signature says:           │
│    -> String                            │
│                                         │
│  But you're returning:                  │
│    "blue" (which is &str)               │
│                                         │
│  You need to CONVERT the &str           │
│  into a String before returning.        │
│                                         │
│  You know three ways to do this!        │
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
│  1. What type is "blue"?                │
│     (String or &str?)                   │
│                                         │
│  2. What type does the function need    │
│     to return? (Check the signature!)   │
│                                         │
│  3. What are the three ways to convert  │
│     &str to String?                     │
│     • String::from(...)                 │
│     • "...".to_string()                 │
│     • "...".to_owned()                  │
│                                         │
│  4. Which one will you use?             │
│     (Any of them will work!)            │
│                                         │
└─────────────────────────────────────────┘
```



Pick any conversion method you like!

(Now go to the Editor and try it!)
