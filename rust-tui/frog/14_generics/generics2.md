# Generic Structs



## Beyond Concrete Types

A struct with a fixed type:

```rust
struct Wrapper {
    value: u32,
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  This Wrapper can ONLY hold u32.        │
│                                         │
│  What if you want to wrap:              │
│  • A String?                            │
│  • A &str?                              │
│  • A custom type?                       │
│                                         │
│  You'd need WrapperString, WrapperStr...│
│  That's a lot of duplication!           │
│                                         │
│  Generics solve this.                   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Making a Struct Generic



## The Syntax

```rust
struct Wrapper<T> {
    value: T,
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  struct Wrapper<T>                      │
│                ↑                        │
│                Type parameter           │
│                                         │
│  Now T is a placeholder:                │
│                                         │
│  Wrapper<u32>    → value is u32         │
│  Wrapper<String> → value is String      │
│  Wrapper<&str>   → value is &str        │
│                                         │
│  ONE struct definition, MANY types!     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Using Generic Structs



## Creating Instances

```rust
let int_wrapper = Wrapper { value: 42 };
// Compiler infers: Wrapper<i32>

let str_wrapper = Wrapper { value: "hello" };
// Compiler infers: Wrapper<&str>

let explicit: Wrapper<f64> = Wrapper { value: 3.14 };
// Explicitly specified
```



```
┌─────────────────────────────────────────┐
│                                         │
│  When you create an instance:           │
│                                         │
│  • Compiler looks at the value type     │
│  • Fills in T with that type            │
│  • Creates a concrete Wrapper<T>        │
│                                         │
│  Each Wrapper<T> is a DIFFERENT type!   │
│  Wrapper<i32> ≠ Wrapper<String>         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Implementing Generic Structs



## The impl Block Challenge

For a non-generic struct:

```rust
struct Wrapper {
    value: u32,
}

impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}
```

Simple! But what about generic structs?



--- slide ---

# Generic impl Blocks



## The Syntax Changes

```rust
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  impl<T> Wrapper<T>                     │
│      ↑         ↑                        │
│      │         └─ Using T in type       │
│      └─ Declaring T exists              │
│                                         │
│  You must DECLARE <T> after impl        │
│  before you can USE it in Wrapper<T>    │
│                                         │
│  Think of it as:                        │
│  "For any type T, implement Wrapper<T>" │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Two Places?



## Declaration vs Usage

```rust
impl<T> Wrapper<T> { ... }
    ↑          ↑
    │          └─ USING T (the type we implement for)
    └─ DECLARING T (introducing the name)
```



```
┌─────────────────────────────────────────┐
│                                         │
│  ANALOGY:                               │
│                                         │
│  fn my_func<T>(param: T) { }            │
│            ↑        ↑                   │
│            │        └─ Using T          │
│            └─ Declaring T               │
│                                         │
│  Same pattern! Declare first, use after.│
│                                         │
│  The impl<T> declares "T exists"        │
│  Then Wrapper<T> uses that T            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Methods on Generic Types



## Using T in Methods

```rust
impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }

    fn get_value(&self) -> &T {
        &self.value
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Inside the impl block, T is known:     │
│                                         │
│  • Parameters can be type T             │
│  • Return types can use T               │
│  • self.value is type T                 │
│                                         │
│  The actual type is determined when     │
│  someone USES Wrapper with a specific   │
│  type like Wrapper<String>.             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Self in Generic Impls



## What Does Self Mean?

```rust
impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Self = Wrapper<T>                      │
│                                         │
│  When someone calls Wrapper::<i32>::new │
│  • T = i32                              │
│  • Self = Wrapper<i32>                  │
│  • Returns Wrapper<i32>                 │
│                                         │
│  Self adapts to whatever T is!          │
│                                         │
│  You could also write:                  │
│  fn new(value: T) -> Wrapper<T>         │
│  But Self is shorter and more flexible. │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Common Mistake



## Forgetting impl<T>

```rust
// WRONG - T is not declared
impl Wrapper<T> {
    fn new(value: T) -> Self { ... }
}

// RIGHT - T is declared
impl<T> Wrapper<T> {
    fn new(value: T) -> Self { ... }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  COMPILER ERROR without impl<T>:        │
│                                         │
│  "cannot find type `T` in this scope"   │
│                                         │
│  The compiler doesn't know what T is!   │
│  You must declare it with impl<T>.      │
│                                         │
│  This is the most common generic error. │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Multiple Type Parameters



## More Than One Generic

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Pair<i32, String>                      │
│       ↑     ↑                           │
│       T     U                           │
│                                         │
│  T and U can be different types!        │
│  Pair<i32, String> has:                 │
│    first: i32                           │
│    second: String                       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Making Wrapper Generic

You have:

```rust
struct Wrapper {
    value: u32,
}

impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}
```

Tests expect it to work with ANY type!



--- slide ---

# What the Tests Expect



## Multiple Types

```rust
// Test 1: Should work with u32
Wrapper::new(42).value  // 42

// Test 2: Should work with &str
Wrapper::new("Foo").value  // "Foo"
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Currently Wrapper only accepts u32.    │
│  But tests want to wrap "Foo" (a &str)! │
│                                         │
│  TASK:                                  │
│                                         │
│  1. Make the struct generic             │
│     struct Wrapper<T> { value: T }      │
│                                         │
│  2. Make the impl block generic         │
│     impl<T> Wrapper<T> { ... }          │
│                                         │
│  3. Update new() to accept T            │
│     fn new(value: T) -> Self            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Transformation



## Step by Step

```
┌─────────────────────────────────────────┐
│                                         │
│  STRUCT:                                │
│                                         │
│  struct Wrapper {        BEFORE         │
│      value: u32,                        │
│  }                                      │
│                                         │
│  struct Wrapper<T> {     AFTER          │
│      value: T,                          │
│  }                                      │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  IMPL:                                  │
│                                         │
│  impl Wrapper {          BEFORE         │
│                                         │
│  impl<T> Wrapper<T> {    AFTER          │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  METHOD:                                │
│                                         │
│  fn new(value: u32)      BEFORE         │
│                                         │
│  fn new(value: T)        AFTER          │
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
│  1. Where does <T> go on the struct?    │
│     (After the struct name)             │
│                                         │
│  2. What replaces u32 in the struct?    │
│     (The type parameter T)              │
│                                         │
│  3. Where does <T> go on impl?          │
│     (After impl, AND after Wrapper)     │
│                                         │
│  4. What's the syntax for impl?         │
│     impl<T> Wrapper<T> { ... }          │
│                                         │
│  5. What replaces u32 in new()?         │
│     (The type parameter T)              │
│                                         │
│  6. Does the return type need to change?│
│     (Self still works - it becomes      │
│      Wrapper<T> automatically!)         │
│                                         │
└─────────────────────────────────────────┘
```



Three changes: struct, impl line, parameter type!

(Now go to the Editor and try it!)
