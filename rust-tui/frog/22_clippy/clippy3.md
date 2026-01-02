# Clippy: Multiple Fixes



## Several Lints to Fix

This exercise has MULTIPLE Clippy issues!

Let's find them all.



```
┌─────────────────────────────────────────┐
│                                         │
│  Types of issues:                       │
│                                         │
│  • Logic errors (wrong condition)       │
│  • Syntax mistakes (missing punctuation)│
│  • Wrong method usage                   │
│  • Incorrect algorithm                  │
│                                         │
│  Clippy catches all of these!           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Issue 1: Wrong Condition



## is_none() vs is_some()

```rust
let my_option: Option<&str> = None;

if my_option.is_none() {
    println!("{}", my_option.unwrap());  // PANIC!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The code says:                         │
│  "If there's NO value, unwrap it!"      │
│                                         │
│  But unwrap on None PANICS!             │
│                                         │
│  The condition is backwards:            │
│  is_none() should be is_some()          │
│                                         │
│  (Or better: use if let Some(x) = ...)  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Issue 2: Missing Comma



## Array Literal Problem

```rust
let my_arr = &[
    -1, -2, -3
    -4, -5, -6
];
```

Looks like 6 numbers? Rust sees 2!



```
┌─────────────────────────────────────────┐
│                                         │
│  Without comma after -3:                │
│                                         │
│  -3                                     │
│  -4   ← Rust sees this as: -3 - 4 = -7  │
│                                         │
│  The array becomes: [-1, -2, -7, -5, -6]│
│  That's 5 elements, not 6!              │
│                                         │
│  Fix: Add comma after -3                │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Issue 3: resize() Returns ()



## Wrong Method for Creating Vec

```rust
let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
```

`resize()` returns `()`, not a new Vec!



```
┌─────────────────────────────────────────┐
│                                         │
│  .resize() modifies IN PLACE            │
│  It returns (), not the resized vec!    │
│                                         │
│  For an empty vec, just use:            │
│  let my_empty_vec = Vec::<i32>::new();  │
│  OR                                     │
│  let my_empty_vec: Vec<i32> = vec![];   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Issue 4: Broken Swap



## Wrong Algorithm

```rust
let mut value_a = 45;
let mut value_b = 66;

// Trying to swap:
value_a = value_b;  // value_a is now 66
value_b = value_a;  // value_b is now 66 too!
```

Both end up with the same value!



```
┌─────────────────────────────────────────┐
│                                         │
│  Step by step:                          │
│                                         │
│  Start:    a=45, b=66                   │
│  a = b:    a=66, b=66  (45 is lost!)    │
│  b = a:    a=66, b=66  (still 66!)      │
│                                         │
│  Fix: Use std::mem::swap(&mut a, &mut b)│
│  OR the tuple swap pattern              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Fixes



## Summary

```
┌─────────────────────────────────────────┐
│                                         │
│  1. is_none() → is_some()               │
│     (or use if let)                     │
│                                         │
│  2. Add comma: -3, (not just -3)        │
│                                         │
│  3. Replace resize with Vec::new()      │
│     or vec![]                           │
│                                         │
│  4. Use std::mem::swap for swapping     │
│     OR: (value_a, value_b) = (value_b, value_a);│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Think It Through



## Before You Type



```
┌─────────────────────────────────────────┐
│  FOR EACH ISSUE:                        │
│                                         │
│  1. Condition: What method checks if    │
│     Option has a value?                 │
│     (is_some(), not is_none())          │
│                                         │
│  2. Array: What's missing between       │
│     -3 and -4?                          │
│     (A comma!)                          │
│                                         │
│  3. Empty Vec: What's the simplest way  │
│     to create one?                      │
│     (vec![] or Vec::new())              │
│                                         │
│  4. Swap: What function safely swaps    │
│     two values?                         │
│     (std::mem::swap)                    │
│                                         │
└─────────────────────────────────────────┘
```



Read Clippy's suggestions carefully!

(Go try it in the Editor!)
