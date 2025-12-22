# Booleans



## What is a Boolean?

A boolean is the simplest type in Rust.

It can only be one of two values:

```rust
true    or    false
```



That's it! Just two possibilities.



Booleans are used for decisions:
- Is the user logged in?
- Is the number greater than 10?
- Is the file ready?



--- slide ---

# Boolean Basics



## Declaring Booleans

```rust
let is_happy = true;
let is_sad = false;
```



The type is `bool`:

```rust
let is_ready: bool = true;
```



## Using in Conditions

Booleans work directly in `if`:

```rust
if is_happy {
    println!("Yay!");
}
```



--- slide ---

# Boolean Operators



## Negation: `!`

The `!` operator flips truth:

```rust
let a = true;
let b = !a;   // b is false

let x = false;
let y = !x;   // y is true
```



Think of it as "NOT":
- `!true` = NOT true = false
- `!false` = NOT false = true



## Other Operators

```
┌────────────────────────────────────────────────┐
│                                                │
│  !a       NOT (flip the value)                 │
│  a && b   AND (both must be true)              │
│  a || b   OR (at least one true)               │
│                                                │
└────────────────────────────────────────────────┘


```



--- slide ---

# Think About Your Exercise



Look at the code:

```rust
let is_morning = true;

if is_evening {
    println!("Good evening!");
}
```



You need to define `is_evening`.

The hint says it should be the OPPOSITE of `is_morning`.



```
┌────────────────────────────────────────────────┐
│                                                │
│  QUESTIONS:                                    │
│                                                │
│  1. What type should is_evening be?            │
│                                                │
│  2. If is_morning is true, what should         │
│     is_evening be?                             │
│                                                │
│  3. What operator gives you the opposite       │
│     of a boolean value?                        │
│                                                │
└────────────────────────────────────────────────┘
```



(Go try it in the Editor!)
