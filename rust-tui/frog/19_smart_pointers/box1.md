# Welcome to Smart Pointers



## A New Concept

We're entering the world of SMART POINTERS.

These are data structures that act like pointers

but have extra superpowers!



```
┌─────────────────────────────────────────┐
│                                         │
│  Regular reference: &T                  │
│  • Just borrows data                    │
│  • No ownership                         │
│                                         │
│  Smart pointer: Box<T>, Rc<T>, etc.     │
│  • OWNS the data                        │
│  • Extra capabilities                   │
│  • Automatic cleanup                    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# What is Box<T>?



## The Simplest Smart Pointer

`Box<T>` puts data on the HEAP instead of the stack.

```rust
let x = 5;           // On the stack
let y = Box::new(5); // On the heap!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  STACK              HEAP                │
│  ┌─────┐            ┌─────┐             │
│  │  5  │  x         │  5  │             │
│  ├─────┤            └──▲──┘             │
│  │  •──┼─────────────────              │
│  └─────┘  y (pointer)                   │
│                                         │
│  y is a pointer TO the heap value       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Use Box?



## The Size Problem

Rust needs to know the SIZE of every type at compile time.

But what about types that contain themselves?



```
┌─────────────────────────────────────────┐
│                                         │
│  enum List {                            │
│      Cons(i32, List),  // ← Problem!    │
│      Nil,                               │
│  }                                      │
│                                         │
│  How big is List?                       │
│  It contains ANOTHER List!              │
│  Which contains ANOTHER List!           │
│  Which contains... infinite!            │
│                                         │
│  Rust can't compute the size!           │
│                                         │
└─────────────────────────────────────────┘
```



This is called a RECURSIVE TYPE.



--- slide ---

# The Cons List



## A Classic Data Structure

The "cons list" comes from Lisp and functional programming.

Each node has a VALUE and a pointer to the NEXT node.



```
┌─────────────────────────────────────────┐
│                                         │
│  Cons list: (1, (2, (3, Nil)))          │
│                                         │
│  ┌─────┬─────┐   ┌─────┬─────┐          │
│  │  1  │  •──┼──→│  2  │  •──┼──→ ...   │
│  └─────┴─────┘   └─────┴─────┘          │
│   value  next     value  next           │
│                                         │
│  Like a linked list!                    │
│                                         │
└─────────────────────────────────────────┘
```



The last element is `Nil` (end of list).



--- slide ---

# The Compiler Error



## Try It Without Box

```rust
enum List {
    Cons(i32, List),  // Direct inclusion
    Nil,
}
```

Rust says:

```
error: recursive type has infinite size
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The problem:                           │
│                                         │
│  Size of List = size of i32             │
│               + size of List            │
│               = 4 + (4 + (4 + ...))     │
│               = INFINITY!               │
│                                         │
│  Rust needs a FIXED size!               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Solution: Indirection



## Box Gives Us a Fixed Size

Instead of containing List directly,

contain a POINTER to a List!



```
┌─────────────────────────────────────────┐
│                                         │
│  enum List {                            │
│      Cons(i32, Box<List>),              │
│      //        ↑↑↑↑↑↑↑↑↑                │
│      //        Fixed size pointer!      │
│      Nil,                               │
│  }                                      │
│                                         │
│  Size of List = size of i32             │
│               + size of Box (pointer)   │
│               = 4 + 8 = 12 bytes        │
│                                         │
│  Box<T> is always 8 bytes (on 64-bit)   │
│  regardless of what T is!               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Creating Box Values



## How to Use Box

```rust
// Create a Box
let boxed = Box::new(5);

// Use it like the inner value (auto-deref)
println!("{}", boxed);  // Prints: 5

// For our List:
let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Box::new(value)                        │
│  • Allocates space on the heap          │
│  • Puts the value there                 │
│  • Returns a pointer to it              │
│                                         │
│  When Box goes out of scope:            │
│  • Automatically frees the heap memory  │
│  • No memory leaks!                     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## What You Need to Do

You have this (broken) enum:

```rust
enum List {
    Cons(i32, List),  // Won't compile!
    Nil,
}
```

And two functions to implement:

```rust
fn create_empty_list() -> List { todo!() }
fn create_non_empty_list() -> List { todo!() }
```



--- slide ---

# Task Breakdown



## Three Things to Do

```
┌─────────────────────────────────────────┐
│                                         │
│  1. FIX THE ENUM                        │
│                                         │
│  Change: Cons(i32, List)                │
│  To:     Cons(i32, ???)                 │
│                                         │
│  Wrap List in a Box!                    │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  2. CREATE EMPTY LIST                   │
│                                         │
│  What represents "nothing" in our List? │
│  (Hint: It's one of the enum variants!) │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  3. CREATE NON-EMPTY LIST               │
│                                         │
│  Use Cons with a value and Box::new()   │
│  The simplest: one item pointing to Nil │
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
│  1. What type wraps another type and    │
│     puts it on the heap?                │
│     (Starts with B...)                  │
│                                         │
│  2. What's the "end" of a cons list?    │
│     (One of your enum variants)         │
│                                         │
│  3. How do you create a Cons with one   │
│     element that ends with Nil?         │
│     (Use Box::new to wrap the tail)     │
│                                         │
└─────────────────────────────────────────┘
```



Box lets us have recursive types by adding indirection!

(Go try it in the Editor!)
