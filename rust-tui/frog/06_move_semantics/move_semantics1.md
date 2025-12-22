# Welcome to Move Semantics



## A New Section

We're entering a section called "Move Semantics."

This covers Rust's unique ownership system.

But let's start simple!



```
┌─────────────────────────────────────────┐
│                                         │
│  This first exercise is actually about  │
│  something you already know:            │
│                                         │
│  MUTABILITY                             │
│                                         │
│  Remember from variables?               │
│  You need `mut` to change things!       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Error You'll See



## Try to Compile First

When you try to compile, Rust complains:

```
cannot borrow `vec` as mutable
```



```
┌─────────────────────────────────────────┐
│                                         │
│  "borrow as mutable" - sounds fancy!    │
│                                         │
│  But it really just means:              │
│                                         │
│  "You're trying to CHANGE something     │
│   that wasn't marked as changeable!"    │
│                                         │
└─────────────────────────────────────────┘
```



This is the SAME concept from variables!



--- slide ---

# Remember Variables?



## Immutable by Default

```rust
let x = 5;
x = 10;  // ERROR! x is not mutable
```

```rust
let mut x = 5;
x = 10;  // OK! x is mutable
```



```
┌─────────────────────────────────────────┐
│                                         │
│  To CHANGE a variable, you need `mut`   │
│                                         │
│  Without mut: can only READ             │
│  With mut: can READ and WRITE           │
│                                         │
└─────────────────────────────────────────┘
```



This applies everywhere - including in functions!



--- slide ---

# Mutability in Functions



## The Same Rule Applies

When a function receives data, it's also immutable

by default - just like regular variables.



```
┌─────────────────────────────────────────┐
│                                         │
│  fn fill_vec(vec: Vec<i32>) {           │
│           // ↑                          │
│           // vec arrives IMMUTABLE      │
│                                         │
│      vec.push(88);                      │
│      // ↑ ERROR! Can't push to          │
│      //   an immutable vector!          │
│  }                                      │
│                                         │
└─────────────────────────────────────────┘
```



`.push()` CHANGES the vector (adds an element).

You can't change something immutable!



--- slide ---

# Look at Your Code



## What's Happening

```rust
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let vec = vec;    // Creates new binding

    vec.push(88);     // Tries to modify!

    vec
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Line 2: `let vec = vec;`               │
│                                         │
│  This creates a NEW variable called     │
│  `vec` that holds the received data.    │
│                                         │
│  But it's created with plain `let`      │
│  so it's IMMUTABLE.                     │
│                                         │
│  Line 4: `vec.push(88);` fails!         │
│  You can't push to an immutable vector! │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Fix



## Make It Mutable

If you want to MODIFY something,

you need to declare it as mutable.



```
┌─────────────────────────────────────────┐
│                                         │
│  Remember from variables?               │
│                                         │
│  let x = 5;       // Can't change x     │
│  let ??? x = 5;   // CAN change x       │
│      ↑↑↑                                │
│      What keyword goes here?            │
│                                         │
│  The SAME keyword works for any `let`   │
│  statement - including the one in your  │
│  function!                              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Think It Through



## You Know What to Do



```
┌─────────────────────────────────────────┐
│  QUESTIONS:                             │
│                                         │
│  1. What keyword allows a variable      │
│     to be changed?                      │
│                                         │
│  2. In the function, which line         │
│     creates the `vec` variable?         │
│                                         │
│  3. How do you modify that line to      │
│     make vec mutable?                   │
│                                         │
└─────────────────────────────────────────┘
```



This is just applying what you already know!



(Go try it in the Editor!)
