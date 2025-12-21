# The If Statement



## The Problem

You need to complete a function that returns 
the bigger of two numbers.



```rust
fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
}
```



To do this, you'll need to make a DECISION.

That's what `if` is for.



--- slide ---

# What is If?



You use `if` to run code only when something is true.



Think of it like a fork in the road:

```
                   condition
                      │
           ┌────┐     │     ┌────┐
      NO ← │ ?? │ ←───┴───→ │ ?? │ → YES
           └────┘           └────┘
```



"IF this is true, do that."

"OTHERWISE, do something else."



Every programming language has this.

Rust is no different.



--- slide ---

# Basic If Syntax



```rust
if condition {
    // code runs when condition is TRUE
}
```



The condition must be a **boolean** - true or false.



How do you get a boolean? Comparisons!

```
┌────────────────────────────────────────────────┐
│                                                │
│  a > b     "Is a greater than b?"              │
│  a < b     "Is a less than b?"                 │
│  a >= b    "Is a greater or equal to b?"       │
│  a <= b    "Is a less or equal to b?"          │
│  a == b    "Is a equal to b?"                  │
│  a != b    "Is a NOT equal to b?"              │
│                                                │
└────────────────────────────────────────────────┘
```



--- slide ---

# Adding Else



What if the condition is false?

Use `else`:

```rust
if condition {
    // runs when true
} else {
    // runs when false
}
```



Now you have two paths:

```
        condition
            │
      ┌─────┴─────┐
      │           │
    FALSE       TRUE
      │           │
      ▼           ▼
  else block   if block
```



--- slide ---

# Multiple Conditions: Else If



What if you have MORE than two options?

Chain them with `else if`:

```rust
if condition1 {
    // first option
} else if condition2 {
    // second option
} else if condition3 {
    // third option
} else {
    // if nothing matched
}
```



It checks each condition in order.

First one that's true wins!



```
condition1 → true?  → run block 1, done
    │
   false
    │
    ▼
condition2 → true?  → run block 2, done
    │
   false
    │
    ▼
  else block
```



--- slide ---

# If is an Expression!




Here's what makes Rust special:

`if` is an EXPRESSION - it returns a value!



```rust
let result = if something {
    value_when_true
} else {
    value_when_false
};
```



The last value in each branch (no semicolon!) 
becomes the result.



This is VERY useful for functions that need 
to return different values based on conditions.



--- slide ---

# Think About Your Exercise



```
┌────────────────────────────────────────────────┐
│                                                │
│  QUESTIONS:                                    │
│                                                │
│  1. You have two numbers: a and b              │
│     What comparison tells you which is bigger? │
│                                                │
│  2. The function must return a value.          │
│     If the condition is true, return what?     │
│     If it's false, return what?                │
│                                                │
│  3. What about if they're equal?               │
│     (Hint: one comparison handles both cases)  │
│                                                │
└────────────────────────────────────────────────┘
```



Remember: This function has no body yet.

You need to write the ENTIRE if/else structure.



(Go try it in the Editor!)
