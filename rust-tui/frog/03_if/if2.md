# The Picky Eater



## The Problem

```rust
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } else {
        1          // <-- Something's wrong here
    }
}
```



The function returns `&str` (a string).

But one branch returns a number!



That's a type mismatch error.



--- slide ---

# All Branches Must Match



Remember from the last exercise: `if` is an expression.

It returns a value.



But what type does it return?

BOTH branches must return the SAME type!



```rust
if condition {
    "string"     // &str
} else {
    1            // i32  ← ERROR! Types don't match!
}
```



Rust can't decide: "Is this an `&str` or an `i32`?"



--- slide ---

# What The Tests Want



Look at the tests:

```rust
// "strawberry" → "Yummy!"
assert_eq!(picky_eater("strawberry"), "Yummy!");

// "potato" → "I guess I can eat that."
assert_eq!(picky_eater("potato"), "I guess I can eat that.");

// everything else → "No thanks!"
assert_eq!(picky_eater("broccoli"), "No thanks!");
```



So you need THREE outcomes, not two!



--- slide ---

# Why Else If?



Remember from Exercise 1?

You learned about `else if` for multiple conditions.



Now you NEED it!

```
┌────────────────────────────────────────────────┐
│                                                │
│  Plain if/else = TWO options                   │
│                                                │
│     if this → option A                         │
│     else    → option B                         │
│                                                │
│  But you need THREE options!                   │
│                                                │
│     if this → option A                         │
│     else if → option B                         │
│     else    → option C                         │
│                                                │
└────────────────────────────────────────────────┘
```



This is why we taught `else if` first!

Without it, you can't handle THREE different outcomes.



The key: ALL branches must return the SAME TYPE.

(In this case, they all need to return `&str`)



--- slide ---

# Think About It



```
┌────────────────────────────────────────────────┐
│                                                │
│  QUESTIONS:                                    │
│                                                │
│  1. The current else branch returns `1`.       │
│     What TYPE is `1`? What should it be?       │
│                                                │
│  2. How many different responses do the tests  │
│     expect? What are they?                     │
│                                                │
│  3. What foods get special treatment?          │
│     What about everything else?                │
│                                                │
└────────────────────────────────────────────────┘
```



Fix the else branch to return a string.

Then add the `else if` for the second case.



(Go try it in the Editor!)
