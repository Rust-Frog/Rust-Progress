# Mutable Parameters



## A Different Approach

In move_semantics1, you made the variable mutable

INSIDE the function by creating a new binding.

But there's another way!



```
┌─────────────────────────────────────────┐
│                                         │
│  Two ways to make a parameter mutable:  │
│                                         │
│  1. Create a mutable binding inside     │
│     (What you did in move_semantics1)   │
│                                         │
│  2. Declare the parameter as mutable    │
│     in the function signature itself!   │
│                                         │
└─────────────────────────────────────────┘
```



This exercise explores the second way.



--- slide ---

# Remember the Previous Solution?



## What You Did Before

```rust
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;   // Create mutable binding
    vec.push(88);
    vec
}
```

You added a line to make a mutable copy of `vec`.



But the TODO for THIS exercise says:

**"Fix the compiler error WITHOUT adding any new line."**



```
┌─────────────────────────────────────────┐
│                                         │
│  You can't add `let mut vec = vec;`     │
│  this time!                             │
│                                         │
│  You have to find another way.          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Mutable Parameters



## Declaring Mutability in the Signature

You can mark a parameter as mutable directly

in the function definition!



```
┌─────────────────────────────────────────┐
│                                         │
│  Regular parameter (immutable):         │
│                                         │
│    fn example(x: i32) { ... }           │
│               ↑                         │
│               Can't change x            │
│                                         │
│                                         │
│  Mutable parameter:                     │
│                                         │
│    fn example(mut x: i32) { ... }       │
│               ↑↑↑                       │
│               CAN change x!             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Does This Work?



## Let's Think About It Deeply

When someone GIVES you something, it's now YOURS.

You can do whatever you want with it!



```
┌─────────────────────────────────────────┐
│                                         │
│  🎁 Someone gives you a gift box.       │
│                                         │
│  It's YOUR box now.                     │
│                                         │
│  Can you open it?  YES - it's yours!    │
│  Can you paint it? YES - it's yours!    │
│  Can you throw it? YES - it's yours!    │
│                                         │
│  The previous owner has NO SAY anymore. │
│  You have FULL CONTROL.                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Applying This to Functions



## When a Function Receives Ownership

```rust
fn fill_vec(vec: Vec<i32>) { ... }
```

When you call `fill_vec(some_vector)`:

1. The calling code GIVES UP ownership
2. The function RECEIVES ownership
3. The function now OWNS the data



```
┌─────────────────────────────────────────┐
│                                         │
│  BEFORE CALL:                           │
│    caller ───owns───▶ [1, 2, 3]         │
│                                         │
│  AFTER CALL:                            │
│    caller ───X───▶ (gave it away)       │
│    fill_vec ───owns───▶ [1, 2, 3]       │
│                                         │
│  fill_vec is the NEW OWNER!             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# So Why Can We Add mut?



## The Owner Decides

Since the function OWNS the data,

the function can decide how to treat it.



```
┌─────────────────────────────────────────┐
│                                         │
│  Think about it:                        │
│                                         │
│  The CALLER gave up ownership.          │
│  They can't use it anymore anyway!      │
│                                         │
│  So if the function wants to:           │
│    • Read the data? GO AHEAD.           │
│    • Modify the data? GO AHEAD.         │
│    • Destroy the data? GO AHEAD.        │
│                                         │
│  It's the function's property now!      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The mut Declaration



## Announcing Your Intentions

When you write:

```rust
fn fill_vec(mut vec: Vec<i32>) { ... }
             ↑↑↑
```

You're saying:

"I'm taking this, AND I plan to change it."



```
┌─────────────────────────────────────────┐
│                                         │
│  Without mut:                           │
│    "I'll take it but just look at it."  │
│                                         │
│  With mut:                              │
│    "I'll take it AND modify it."        │
│                                         │
│  Both are valid! You own it either way. │
│  The mut just declares your INTENT.     │
│                                         │
└─────────────────────────────────────────┘
```



The `mut` in the parameter says:

"I'm taking ownership AND I want to modify this."



--- slide ---

# Comparing the Two Approaches



## Same Result, Different Syntax

```
┌─────────────────────────────────────────┐
│                                         │
│  APPROACH 1: Mutable binding inside     │
│                                         │
│    fn func(param: Type) {               │
│        let mut param = param;  // rebind│
│        // now param is mutable          │
│    }                                    │
│                                         │
│                                         │
│  APPROACH 2: Mutable parameter          │
│                                         │
│    fn func(mut param: Type) {           │
│             ↑↑↑                         │
│        // param is already mutable!     │
│    }                                    │
│                                         │
│  Both achieve the same thing!           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Important: NOT a Reference!



## Don't Confuse These Two

```
┌─────────────────────────────────────────┐
│                                         │
│  THIS EXERCISE:                         │
│                                         │
│    fn func(mut vec: Vec<i32>)           │
│             ↑↑↑                         │
│    Takes OWNERSHIP, mutably             │
│    (No & symbol!)                       │
│                                         │
│                                         │
│  DIFFERENT THING (later lessons):       │
│                                         │
│    fn func(vec: &mut Vec<i32>)          │
│                 ↑↑↑↑                    │
│    BORROWS mutably                      │
│    (Has & symbol)                       │
│                                         │
│  For THIS exercise, just add `mut`      │
│  BEFORE the parameter name.             │
│  No & needed!                           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Look at Your Exercise



## The Current Code

```rust
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);  // Tries to mutate!
    vec
}
```

There's no `let mut` line this time.

Just the direct push.



```
┌─────────────────────────────────────────┐
│                                         │
│  The constraint says:                   │
│  "Fix WITHOUT adding any new line"      │
│                                         │
│  So you can CHANGE an existing line,    │
│  but not ADD one.                       │
│                                         │
│  Which line needs to change?            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Think It Through



## Where Can You Put mut?



```
┌─────────────────────────────────────────┐
│  QUESTIONS:                             │
│                                         │
│  1. The error is "cannot borrow `vec`   │
│     as mutable" - what needs to be      │
│     mutable?                            │
│                                         │
│  2. Look at the function signature:     │
│     fn fill_vec(vec: Vec<i32>)          │
│                                         │
│     Where can you add `mut` to make     │
│     the parameter mutable?              │
│                                         │
│  3. Remember: You're not adding a line, │
│     just modifying the existing one.    │
│                                         │
└─────────────────────────────────────────┘
```



(Go try it in the Editor!)
