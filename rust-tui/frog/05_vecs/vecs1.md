# Wait, What's Wrong With Arrays?



## Let's Think About This

You just learned arrays. They hold multiple values.

So... why do we need something else?



```
┌─────────────────────────────────────────┐
│                                         │
│  Imagine you're making a TO-DO list.    │
│                                         │
│  Day 1: You have 3 tasks.               │
│  Day 2: You add 2 more tasks.           │
│  Day 3: You finish some, add others.    │
│                                         │
│  Your list CHANGES SIZE all the time!   │
│                                         │
└─────────────────────────────────────────┘
```



Arrays CAN'T do this. They're frozen in size.



--- slide ---

# The Array Problem



## When You Create an Array...

```rust
let tasks = ["Buy milk", "Call mom", "Study Rust"];
```

You've made a list of EXACTLY 3 items.

Forever. No changes. Done.



```
┌─────────────────────────────────────────┐
│                                         │
│  "But I want to add another task!"      │
│                                         │
│  Too bad. Array says NO.                │
│                                         │
│  "Can I at least remove one?"           │
│                                         │
│  Nope. Still 3 slots. Always 3 slots.   │
│                                         │
└─────────────────────────────────────────┘
```



This is frustrating when you don't know

ahead of time how many things you'll have!



--- slide ---

# Real World Examples



## Where Fixed Size Fails

Think about these situations:



```
┌─────────────────────────────────────────┐
│                                         │
│  📝 User types messages in a chat       │
│     → Don't know how many messages!     │
│                                         │
│  🎮 Enemies spawn in a game             │
│     → Don't know how many enemies!      │
│                                         │
│  📊 Reading lines from a file           │
│     → Don't know how many lines!        │
│                                         │
│  🛒 User adds items to shopping cart    │
│     → Don't know how many items!        │
│                                         │
└─────────────────────────────────────────┘
```



All of these need a list that can GROW.



--- slide ---

# Enter: The Vector



## A List That Can Change

A vector is exactly what it sounds like:

**An array that can grow and shrink.**



```
┌─────────────────────────────────────────┐
│                                         │
│  Think of it like this:                 │
│                                         │
│  ARRAY = Carved in stone                │
│          Fixed forever. Permanent.      │
│                                         │
│  VECTOR = Written on a whiteboard       │
│           Add more. Erase some. Flex!   │
│                                         │
└─────────────────────────────────────────┘
```



That's the core idea. Nothing more mysterious.

A vector is just a flexible array.



--- slide ---

# How Do You Make One?



## The vec! Trick

Rust gives you a shortcut called `vec!`

(Notice the exclamation mark - that's important)



Think of `vec!` as saying:

*"Hey Rust, I want a growable list with these items"*



```
┌─────────────────────────────────────────┐
│                                         │
│  You want a vector with 1, 2, 3?        │
│                                         │
│  You write:  vec![1, 2, 3]              │
│                                         │
│  That's it. That's the whole thing.     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Side by Side



## Array vs Vector Syntax

The syntax is almost identical!

Just add `vec!` in front.



```
┌─────────────────────────────────────────┐
│                                         │
│  ARRAY:                                 │
│                                         │
│    let a = [1, 2, 3];                   │
│             ↑                           │
│             Just brackets               │
│                                         │
│                                         │
│  VECTOR:                                │
│                                         │
│    let v = vec![1, 2, 3];               │
│            ↑↑↑↑                         │
│            vec! then brackets           │
│                                         │
└─────────────────────────────────────────┘
```



If you know how to write an array,

you already know how to write a vector!



--- slide ---

# Why The Exclamation Mark?



## Macros vs Functions

That `!` means it's a "macro" not a regular function.

What's the difference?



```
┌─────────────────────────────────────────┐
│                                         │
│  FUNCTION:                              │
│    Takes a FIXED number of arguments    │
│    add(1, 2)  ← always 2 args           │
│                                         │
│  MACRO:                                 │
│    Takes ANY number of arguments        │
│    vec![1]            ← 1 arg           │
│    vec![1, 2]         ← 2 args          │
│    vec![1, 2, 3, 4]   ← 4 args          │
│    All work!                            │
│                                         │
└─────────────────────────────────────────┘
```



Macros are flexible - they can handle

different amounts of input!



--- slide ---

# Spotting Macros



## The ! Is Your Clue

Whenever you see a `!` after a name in Rust,

you know it's a macro.



```
┌─────────────────────────────────────────┐
│                                         │
│  println!(...)    ← macro (prints)      │
│  vec![...]        ← macro (vector)      │
│  format!(...)     ← macro (strings)     │
│                                         │
│  String::new()    ← function (no !)     │
│  variable.push()  ← function (no !)     │
│                                         │
│  ! = macro                              │
│  no ! = function                        │
│                                         │
└─────────────────────────────────────────┘
```



For now, just remember: vec needs the `!`



--- slide ---

# Let's Practice Mentally



## The Thinking Process

Imagine I have an array with some values.

How do I turn that into a vector?



```
┌─────────────────────────────────────────┐
│                                         │
│  STEP 1: Look at the array values       │
│                                         │
│  STEP 2: Those SAME values go           │
│          inside vec![...]               │
│                                         │
│  STEP 3: Assign to a variable name      │
│                                         │
│  That's the whole process!              │
│                                         │
└─────────────────────────────────────────┘
```



You're not calculating anything new.

Just repackaging the same data.



--- slide ---

# Your Exercise



## What You're Being Asked

Look at the code. There's an array:

```rust
let a = [10, 20, 30, 40];
```

And a comment asking you to create a vector `v`

with the SAME elements.



```
┌─────────────────────────────────────────┐
│                                         │
│  You're NOT being asked to do           │
│  anything complicated.                  │
│                                         │
│  Just: "Make a vector with these        │
│         same four numbers."             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Think It Through



## Before You Type



```
┌─────────────────────────────────────────┐
│                                         │
│  The array has: 10, 20, 30, 40          │
│                                         │
│  You need a vector with: 10, 20, 30, 40 │
│                                         │
│  The magic word is: vec!                │
│                                         │
│  Put them together and you've got it!   │
│                                         │
└─────────────────────────────────────────┘
```



```
┌─────────────────────────────────────────┐
│  ASK YOURSELF:                          │
│                                         │
│  1. What values do I need inside?       │
│                                         │
│  2. What keyword creates a vector?      │
│                                         │
│  3. What's the variable name I need?    │
│     (Check what the function returns!)  │
│                                         │
└─────────────────────────────────────────┘
```



(Now go to the Editor and try it!)
