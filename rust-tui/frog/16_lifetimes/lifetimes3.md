# Lifetimes in Structs



## Structs Holding References

When a struct holds a reference, Rust needs to know:

"How long must the referenced data live?"

```rust
struct Book {
    author: &str,  // Reference to borrowed data
    title: &str,   // Reference to borrowed data
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  COMPILER ERROR:                        │
│                                         │
│  "missing lifetime specifier"           │
│                                         │
│  The struct BORROWS data, it doesn't    │
│  own it.                                │
│                                         │
│  Rust needs to ensure the borrowed      │
│  data outlives the struct!              │
│                                         │
│  Otherwise, Book could point to freed   │
│  memory.                                │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Structs Need Lifetimes



## The Ownership Question

```rust
struct Book {
    author: &str,  // Who owns this string?
}

let book;
{
    let name = String::from("Orwell");
    book = Book { author: &name };
}  // name dropped!
// book.author is now invalid!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  WITHOUT lifetime annotations:          │
│                                         │
│  Rust can't track whether book.author   │
│  will become a dangling reference.      │
│                                         │
│  The struct could OUTLIVE the data      │
│  it references!                         │
│                                         │
│  Lifetime parameters solve this by      │
│  creating a CONTRACT:                   │
│                                         │
│  "The referenced data must live at      │
│   least as long as the struct"          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Adding Lifetimes to Structs



## The Syntax

```rust
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  struct Book<'a>                        │
│             ↑                           │
│             Lifetime parameter          │
│                                         │
│  author: &'a str                        │
│           ↑                             │
│           Uses that lifetime            │
│                                         │
│  This means:                            │
│  "Book<'a> contains references that     │
│   must be valid for lifetime 'a"        │
│                                         │
│  An instance of Book can't outlive      │
│  the strings it references!             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Struct vs Function Lifetimes



## Same Concept, Same Syntax

```rust
// Function with lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str

// Struct with lifetime
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The pattern is consistent:             │
│                                         │
│  1. Declare lifetime in angle brackets  │
│     <'a>                                │
│                                         │
│  2. Use it on references                │
│     &'a Type                            │
│                                         │
│  Whether it's a function or struct,     │
│  the syntax is the same!                │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Using Structs with Lifetimes



## Creating Instances

```rust
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

let book = Book {
    author: "George Orwell",
    title: "1984",
};
```



```
┌─────────────────────────────────────────┐
│                                         │
│  String literals have 'static lifetime  │
│  (they're baked into the program)       │
│                                         │
│  "George Orwell" lives forever          │
│  "1984" lives forever                   │
│                                         │
│  So Book can live as long as it needs!  │
│                                         │
│  The compiler infers the lifetime       │
│  when you create the instance.          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Multiple References, One Lifetime



## When Fields Share Lifetime

```rust
struct Book<'a> {
    author: &'a str,  // Both have 'a
    title: &'a str,   // Both have 'a
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  When all references share ONE lifetime:│
│                                         │
│  The struct can only live as long as    │
│  ALL referenced data lives.             │
│                                         │
│  'a = the SHORTEST lifetime among       │
│       all the references                │
│                                         │
│  This is conservative but simple!       │
│                                         │
│  For more complex cases, you can use    │
│  multiple lifetime parameters:          │
│  struct Complex<'a, 'b> { ... }         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# String Literals



## The 'static Lifetime

```rust
let s: &'static str = "I live forever";
```



```
┌─────────────────────────────────────────┐
│                                         │
│  String literals are &'static str       │
│                                         │
│  'static means:                         │
│  "Lives for the entire program duration"│
│                                         │
│  Why? String literals are stored in     │
│  the compiled binary itself.            │
│                                         │
│  They're never freed!                   │
│                                         │
│  So Book { author: "Orwell", ... }      │
│  works because "Orwell" is 'static,     │
│  which satisfies any lifetime 'a.       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Adding Lifetimes to a Struct

You have:

```rust
struct Book {
    author: &str,
    title: &str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };
    println!("{} by {}", book.title, book.author);
}
```

The struct needs lifetime annotations!



--- slide ---

# The Problem



## Missing Lifetime Parameter

```
┌─────────────────────────────────────────┐
│                                         │
│  COMPILER ERROR:                        │
│                                         │
│  "expected named lifetime parameter"    │
│                                         │
│  The struct has reference fields:       │
│  • author: &str                         │
│  • title: &str                          │
│                                         │
│  Rust needs to know how long these      │
│  references must be valid.              │
│                                         │
│  TASK:                                  │
│                                         │
│  1. Add <'a> to struct definition       │
│  2. Annotate both fields with 'a        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Transformation



## What to Change

```
┌─────────────────────────────────────────┐
│                                         │
│  BEFORE:                                │
│                                         │
│  struct Book {                          │
│      author: &str,                      │
│      title: &str,                       │
│  }                                      │
│                                         │
│  AFTER:                                 │
│                                         │
│  struct Book<'a> {                      │
│      author: &'a str,                   │
│      title: &'a str,                    │
│  }                                      │
│                                         │
│  Two additions:                         │
│  1. <'a> after struct name              │
│  2. 'a after each & in the fields       │
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
│  1. Why does the struct need lifetimes? │
│     (It holds references, not owned data)│
│                                         │
│  2. Where do you declare the lifetime?  │
│     (After struct name: struct Book<'a>)│
│                                         │
│  3. How do you annotate a reference?    │
│     (&'a str instead of &str)           │
│                                         │
│  4. How many fields need annotation?    │
│     (Both author and title)             │
│                                         │
│  5. Does main() need to change?         │
│     (No! The compiler infers the        │
│      concrete lifetime from usage)      │
│                                         │
│  6. Why do string literals work?        │
│     (They have 'static lifetime,        │
│      which satisfies any 'a)            │
│                                         │
└─────────────────────────────────────────┘
```



Add <'a> and annotate both fields!

(Now go to the Editor and try it!)
