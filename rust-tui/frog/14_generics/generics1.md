# Generics: The Power of Abstraction



## The Duplication Problem

Imagine you need functions that work with different types:

```rust
fn print_i32(value: i32) {
    println!("{}", value);
}

fn print_string(value: String) {
    println!("{}", value);
}

fn print_bool(value: bool) {
    println!("{}", value);
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  THREE functions doing the SAME thing!  │
│                                         │
│  Just the TYPE is different.            │
│                                         │
│  What if we could write ONE function    │
│  that works with ANY type?              │
│                                         │
│  That's what generics are for!          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# What Are Generics?



## Parameterized Types

Generics let you write code that works with many types:

```rust
fn print_value<T>(value: T) {
    // Works with ANY type T
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  <T> is a TYPE PARAMETER                │
│                                         │
│  It's like a placeholder that says:     │
│  "Some type will go here, but I don't   │
│   know which one yet."                  │
│                                         │
│  T is just a name - convention uses:    │
│  • T for "Type"                         │
│  • E for "Element"                      │
│  • K, V for "Key, Value"                │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Generic Syntax



## Breaking It Down

```rust
fn function_name<T>(param: T) -> T { ... }
   │            │    │          │
   │            │    │          └─ Return type uses T
   │            │    └─ Parameter uses T
   │            └─ Declare type parameter
   └─ Function name
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The <T> comes AFTER the function name  │
│  but BEFORE the parameters.             │
│                                         │
│  This DECLARES that T exists.           │
│  Then you can USE T in:                 │
│                                         │
│  • Parameter types                      │
│  • Return type                          │
│  • Function body                        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# You've Already Used Generics!



## Vec<T> and Option<T>

```rust
let numbers: Vec<i32> = Vec::new();
let strings: Vec<String> = Vec::new();

let maybe_int: Option<i32> = Some(42);
let maybe_str: Option<&str> = Some("hello");
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Vec<i32>    Vec<String>                │
│      ↑           ↑                      │
│      └─ T is i32 └─ T is String         │
│                                         │
│  Vec is GENERIC over T.                 │
│  You choose what T is when you use it!  │
│                                         │
│  Same Vec code works for ANY type.      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Type Inference with Generics



## The Compiler Is Smart

Usually, Rust can figure out T:

```rust
let mut numbers = Vec::new();
numbers.push(42);  // Compiler sees i32 here
// Now it knows: Vec<i32>

let mut words = Vec::new();
words.push("hello");  // Compiler sees &str
// Now it knows: Vec<&str>
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Type inference works by looking at     │
│  HOW you use the generic type.          │
│                                         │
│  Push an i32? Must be Vec<i32>.         │
│  Push a &str? Must be Vec<&str>.        │
│                                         │
│  The compiler is good at this!          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# When Inference Fails



## The Compiler Needs Help

Sometimes there's not enough information:

```rust
let numbers = Vec::new();
// Error: cannot infer type
// What is T supposed to be?
```



```
┌─────────────────────────────────────────┐
│                                         │
│  COMPILER ERROR:                        │
│                                         │
│  "type annotations needed"              │
│  "cannot infer type for type parameter" │
│                                         │
│  The compiler is saying:                │
│  "I don't know what T is! Tell me!"     │
│                                         │
│  This happens when:                     │
│  • Nothing is pushed yet                │
│  • Multiple types could work            │
│  • Conversions make it ambiguous        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Type Annotations



## Telling the Compiler What T Is

Two ways to specify the type:

```rust
// Method 1: Annotate the variable
let numbers: Vec<i32> = Vec::new();
//         ^^^^^^^^ T is i32

// Method 2: Turbofish syntax
let numbers = Vec::<i32>::new();
//               ^^^^^^^ T is i32
```



```
┌─────────────────────────────────────────┐
│                                         │
│  VARIABLE ANNOTATION:                   │
│  let x: Type = ...                      │
│                                         │
│  TURBOFISH SYNTAX:                      │
│  function::<Type>(...)                  │
│           ^^^^^^                        │
│  Looks like a fish! ::<>                │
│                                         │
│  Both tell compiler what T should be.   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The .into() Method



## Type Conversions

`.into()` converts between compatible types:

```rust
let x: u8 = 42;
let y: i32 = x.into();  // u8 → i32

let a: i8 = -5;
let b: i32 = a.into();  // i8 → i32
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .into() uses the From trait to         │
│  convert types automatically.           │
│                                         │
│  u8 can become i16, i32, i64, etc.      │
│  i8 can become i16, i32, i64, etc.      │
│                                         │
│  But what type does .into() produce?    │
│  It depends on what you NEED!           │
│                                         │
│  Sometimes the compiler can't tell      │
│  what the target type should be.        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Ambiguity Problem



## When Both Source Types Differ

```rust
let n1: u8 = 42;
let n2: i8 = -1;

let mut numbers = Vec::new();
numbers.push(n1.into());  // u8 → ???
numbers.push(n2.into());  // i8 → ???
```



```
┌─────────────────────────────────────────┐
│                                         │
│  PROBLEM:                               │
│                                         │
│  n1 is u8 (unsigned: 0 to 255)          │
│  n2 is i8 (signed: -128 to 127)         │
│                                         │
│  Both are being converted with .into()  │
│  They need to become the SAME type T    │
│                                         │
│  What type can hold:                    │
│  • Values from u8 (0-255)?              │
│  • Values from i8 (-128 to 127)?        │
│  • Including NEGATIVE numbers?          │
│                                         │
│  The vector needs ONE type for all!     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Finding a Common Type



## What Type Works?

```
┌─────────────────────────────────────────┐
│                                         │
│  u8 range:   0 to 255                   │
│  i8 range:   -128 to 127                │
│                                         │
│  Combined range: -128 to 255            │
│                                         │
│  Which integer types can hold this?     │
│                                         │
│  • i8?   NO  (max 127, can't hold 255)  │
│  • u8?   NO  (no negatives)             │
│  • i16?  YES (-32768 to 32767)          │
│  • i32?  YES (even bigger range)        │
│  • i64?  YES (huge range)               │
│                                         │
│  Any of i16, i32, i64 would work!       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Type Annotation Needed

You have:

```rust
let mut numbers = Vec::new();

let n1: u8 = 42;
numbers.push(n1.into());
let n2: i8 = -1;
numbers.push(n2.into());
```



```
┌─────────────────────────────────────────┐
│                                         │
│  THE TASK:                              │
│                                         │
│  The compiler doesn't know what type    │
│  the Vec should hold.                   │
│                                         │
│  Add a type annotation to numbers!      │
│                                         │
│  Vec<???> - what type works for both    │
│  u8 and i8 values (including negative)? │
│                                         │
│  Hint: You need a SIGNED integer type   │
│  that's larger than both u8 and i8.     │
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
│  1. What is the current error?          │
│     (Type cannot be inferred)           │
│                                         │
│  2. What needs to be annotated?         │
│     (The numbers variable)              │
│                                         │
│  3. What syntax adds a type to Vec?     │
│     (Vec<T> where T is the type)        │
│                                         │
│  4. What type can hold both:            │
│     • Positive values up to 255 (u8)    │
│     • Negative values down to -128 (i8) │
│                                         │
│  5. Is i8 big enough? Is u16?           │
│     What about i16 or i32?              │
│                                         │
│  6. Where do you put the annotation?    │
│     let numbers: Vec<???> = ...         │
│                                         │
└─────────────────────────────────────────┘
```



One type annotation makes it compile!

(Now go to the Editor and try it!)
