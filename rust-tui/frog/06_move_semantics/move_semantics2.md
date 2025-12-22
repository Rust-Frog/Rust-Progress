# Welcome to Ownership



## This Is the Big One

This exercise introduces Rust's most unique concept:

**OWNERSHIP**

This is what makes Rust different from every other language.



```
┌─────────────────────────────────────────┐
│                                         │
│  Don't worry if this takes time to      │
│  understand. Ownership is the hardest   │
│  part of learning Rust!                 │
│                                         │
│  We'll go step by step.                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# What Is Ownership?



## The Core Idea

In Rust, every piece of data has ONE owner.

Only ONE variable can "own" a piece of data at a time.



```
┌─────────────────────────────────────────┐
│                                         │
│  Think of it like a physical object:    │
│                                         │
│  🎸 You have a guitar.                  │
│     YOU own it.                         │
│     Only ONE person can own it.         │
│                                         │
│  You can GIVE it to someone else.       │
│  Then THEY own it. You don't anymore.   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# What Does "Own" Mean?



## Let's Look at Memory

Your computer has memory (RAM).

When you create data, it gets stored somewhere.



```
┌──────────────────────────────────────────┐
│                                          │
│  let name = String::from("Alice");       │
│                                          │
│  What happens:                           │
│                                          │
│  VARIABLE         MEMORY                 │
│  ┌──────┐        ┌───────────────┐       │
│  │ name ├───────▶│   "Alice"     │       │
│  └──────┘        └───────────────┘       │
│     ↑                   ↑                │
│  Variable          Actual data           │
│  (the name)        (in memory)           │
│                                          │
│  `name` POINTS TO where the data lives   │
│                                          │
└──────────────────────────────────────────┘
```



--- slide ---

# The Owner Is Responsible



## What Ownership Means

The owner is responsible for the data:



```
┌─────────────────────────────────────────┐
│                                         │
│  Being an OWNER means:                  │
│                                         │
│  1. You can ACCESS it (read it)         │
│                                         │
│  2. You can MODIFY it (if mutable)      │
│                                         │
│  3. When you're DONE, it gets cleaned   │
│     up automatically                    │
│                                         │
│  4. Only ONE owner at a time            │
│                                         │
└─────────────────────────────────────────┘
```



Like owning a house - YOU are responsible

for it, and it gets cleaned when you leave.



--- slide ---

# Now: The Key Concept



## Passing to a Function MOVES Ownership

When you give data to a function,

the ownership TRANSFERS to that function.



```
┌─────────────────────────────────────────┐
│                                         │
│  let vec0 = vec![1, 2, 3];              │
│  // vec0 owns the data                  │
│                                         │
│  some_function(vec0);                   │
│  // Ownership MOVES to the function     │
│                                         │
│  // vec0 is INVALID now!                │
│  // You gave it away!                   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Visualizing the Move



## Step by Step



```
┌─────────────────────────────────────────┐
│                                         │
│  BEFORE calling the function:           │
│                                         │
│  vec0 ────────────▶ [1, 2, 3]           │
│  (vec0 owns it)                         │
│                                         │
└─────────────────────────────────────────┘
```

```
┌─────────────────────────────────────────┐
│                                         │
│  AFTER calling the function:            │
│                                         │
│  vec0 ──────X─────▶ (nothing!)          │
│  (vec0 is dead, empty, invalid)         │
│                                         │
│  function's param ──────▶ [1, 2, 3]     │
│  (function now owns it)                 │
│                                         │
└─────────────────────────────────────────┘
```



The data didn't copy. The ownership just moved!



--- slide ---

# Why Can't You Use It After?



## The Variable Is "Dead"

After giving ownership away, the variable

points to nothing. It's like a broken link.



```
┌─────────────────────────────────────────┐
│                                         │
│  If you try:                            │
│                                         │
│    println!("{:?}", vec0);              │
│                                         │
│  Rust says:                             │
│                                         │
│    "ERROR! vec0 was MOVED. You can't    │
│     use it anymore!"                    │
│                                         │
│  This PREVENTS bugs at compile time!    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Problem In This Exercise



## Look at the Test

```rust
let vec0 = vec![22, 44, 66];

let vec1 = fill_vec(vec0);   // vec0 moves away

assert_eq!(vec0, [...]);     // Uses vec0!
assert_eq!(vec1, [...]);     // Uses vec1!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  THE CONFLICT:                          │
│                                         │
│  After fill_vec(vec0), ownership moved. │
│  vec0 is invalid.                       │
│                                         │
│  But the test needs vec0 AGAIN!         │
│                                         │
│  You gave away your guitar,             │
│  but you still need to play it!         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Solution: Make a Copy



## You Can't Give AND Keep...

...unless you DUPLICATE it first!



```
┌─────────────────────────────────────────┐
│                                         │
│  🎸 You have a guitar.                  │
│                                         │
│  Friend wants it. You still need it.    │
│                                         │
│  Solution:                              │
│                                         │
│  Make a COPY of the guitar!             │
│  Give them the copy.                    │
│  Keep your original.                    │
│                                         │
│  Now you BOTH have guitars!             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# In Rust: .clone()



## Makes an Independent Copy

```rust
let original = vec![1, 2, 3];
let copy = original.clone();
```



```
┌─────────────────────────────────────────┐
│                                         │
│  AFTER .clone():                        │
│                                         │
│  original ───▶ [1, 2, 3]  (still yours) │
│                                         │
│  copy ───────▶ [1, 2, 3]  (brand new!)  │
│                                         │
│  Two separate vectors!                  │
│  Both exist! Both are valid!            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Applying to Your Exercise



## The Current Code

```rust
let vec0 = vec![22, 44, 66];
let vec1 = fill_vec(vec0);     // vec0 moves
```



```
┌─────────────────────────────────────────┐
│                                         │
│  What if you passed a CLONE instead?    │
│                                         │
│    fill_vec( ??? )                      │
│              ↑                          │
│              Pass a copy of vec0!       │
│                                         │
│  Then the COPY moves to the function    │
│  But vec0 stays with you!               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Wait - Is Clone Always Good?



## Honest Talk About Clone

Clone WORKS, but it has tradeoffs:



```
┌─────────────────────────────────────────┐
│                                         │
│  THE COST OF CLONE:                     │
│                                         │
│  • Copies ALL the data in memory        │
│    (slow for big data!)                 │
│                                         │
│  • Uses MORE memory                     │
│    (now you have two copies)            │
│                                         │
│  • Can be a "lazy fix"                  │
│    (hiding better patterns)             │
│                                         │
└─────────────────────────────────────────┘
```



Clone isn't BAD, but it's not always the BEST option.



--- slide ---

# There's a Better Way (Coming Soon!)



## Borrowing: The Elegant Solution

Instead of copying, you can LEND the data

without giving up ownership. This uses `&`.



```
┌─────────────────────────────────────────┐
│                                         │
│  CLONE:                                 │
│    "Make a copy. Give away the copy."   │
│    → Two separate copies exist          │
│    → Uses more memory                   │
│                                         │
│  BORROWING (coming in later exercises): │
│    "Let them look at mine temporarily." │
│    → No copying!                        │
│    → You still own it                   │
│    → More efficient                     │
│                                         │
└─────────────────────────────────────────┘
```



For THIS exercise, clone is the expected solution.

But keep borrowing in mind - you'll learn it soon!



--- slide ---

# Think It Through



```
┌─────────────────────────────────────────┐
│  QUESTIONS:                             │
│                                         │
│  1. Which line passes vec0 to the       │
│     function?                           │
│                                         │
│  2. What method makes a copy of data?   │
│     (Hint: rhymes with "phone")         │
│                                         │
│  3. The syntax is: variable.______()    │
│     What fills in the blank?            │
│                                         │
└─────────────────────────────────────────┘
```



(Go try it in the Editor!)

