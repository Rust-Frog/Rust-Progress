# Lifetimes: References and Validity



## The Borrow Checker's Question

When a function returns a reference, Rust asks:

"How long is this reference valid?"

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  COMPILER ERROR:                        │
│                                         │
│  "missing lifetime specifier"           │
│  "this function's return type contains  │
│   a borrowed value, but the signature   │
│   does not say whether it is borrowed   │
│   from `x` or `y`"                      │
│                                         │
│  The compiler is confused!              │
│  Which input does the output come from? │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Lifetimes Matter



## References Don't Own Data

```rust
let result;
{
    let s = String::from("hello");
    result = &s;  // Borrow s
}  // s is dropped here!
// result now points to invalid memory!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  References are BORROWS, not owners.    │
│                                         │
│  The owner controls when data is freed. │
│  If the owner goes away, the reference  │
│  becomes DANGLING (points to nothing).  │
│                                         │
│  Rust's borrow checker prevents this    │
│  by tracking LIFETIMES of references.   │
│                                         │
│  A reference cannot outlive its data!   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Ambiguity Problem



## Which Reference Gets Returned?

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x    // Sometimes returns x
    } else {
        y    // Sometimes returns y
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  At compile time, Rust can't know       │
│  which branch will execute!             │
│                                         │
│  The return could be:                   │
│  • A reference from x                   │
│  • A reference from y                   │
│                                         │
│  Rust needs to know: how long is the    │
│  returned reference valid?              │
│                                         │
│  Answer: as long as BOTH inputs live!   │
│  (Because either could be returned)     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Lifetime Annotations



## Naming the Relationship

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //     ↑     ↑          ↑            ↑
    //     │     │          │            │
    //     │     └──────────┴────────────┘
    //     │     All three share lifetime 'a
    //     └─ Declare the lifetime parameter
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  'a is a LIFETIME PARAMETER             │
│  (Note: starts with apostrophe!)        │
│                                         │
│  It says:                               │
│  "x, y, and the return all live for     │
│   the same duration 'a"                 │
│                                         │
│  This tells Rust:                       │
│  The returned reference is valid as     │
│  long as BOTH x AND y are valid.        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Lifetime Syntax



## Breaking It Down

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
```



```
┌─────────────────────────────────────────┐
│                                         │
│  STEP 1: Declare lifetime parameter     │
│                                         │
│  fn longest<'a>(...) -> ...             │
│            ↑                            │
│            Angle brackets, like generics│
│                                         │
│  STEP 2: Annotate input references      │
│                                         │
│  x: &'a str                             │
│      ↑                                  │
│      Lifetime comes after &             │
│                                         │
│  STEP 3: Annotate return reference      │
│                                         │
│  -> &'a str                             │
│      ↑                                  │
│      Same lifetime as inputs            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# What Lifetimes Mean



## Not Changing Duration, Just Describing

```
┌─────────────────────────────────────────┐
│                                         │
│  IMPORTANT:                             │
│                                         │
│  Lifetime annotations DON'T change how  │
│  long references live!                  │
│                                         │
│  They DESCRIBE the relationship to the  │
│  compiler so it can check validity.     │
│                                         │
│  'a means: "some lifetime, call it 'a"  │
│                                         │
│  When x: &'a str and y: &'a str:        │
│  Rust uses the SHORTER of the two       │
│  actual lifetimes as 'a.                │
│                                         │
│  The return can't outlive either input! │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Naming Conventions



## Common Lifetime Names

```rust
'a  // Most common, generic lifetime
'b  // Second lifetime (if needed)
'static  // Special: lives for entire program
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Lifetime names are usually short:      │
│                                         │
│  'a, 'b, 'c ...                         │
│                                         │
│  By convention, start with 'a and go    │
│  alphabetically if you need more.       │
│                                         │
│  'static is special:                    │
│  • Built into the language              │
│  • Means "lives forever"                │
│  • String literals have 'static        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Lifetime Elision



## When You Don't Need Annotations

Sometimes Rust infers lifetimes:

```rust
// This works without annotation!
fn first_word(s: &str) -> &str {
    // ...
}

// Compiler understands: output comes from input
// No ambiguity = no annotation needed
```



```
┌─────────────────────────────────────────┐
│                                         │
│  ELISION RULES (compiler infers):       │
│                                         │
│  1. Each input reference gets its own   │
│     lifetime                            │
│                                         │
│  2. If exactly one input reference,     │
│     output gets that lifetime           │
│                                         │
│  3. If &self or &mut self is an input,  │
│     output gets self's lifetime         │
│                                         │
│  When ambiguous (like our case),        │
│  you MUST annotate!                     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why longest Needs Annotation



## Two Inputs = Ambiguity

```rust
fn longest(x: &str, y: &str) -> &str
//        ↑         ↑
//        Two different references!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  With TWO input references:             │
│                                         │
│  • Output could come from x             │
│  • Output could come from y             │
│  • Elision rules don't apply!           │
│                                         │
│  You must explicitly say:               │
│  "The output lifetime relates to        │
│   both inputs"                          │
│                                         │
│  That's what <'a> on all three does.    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Adding Lifetime Annotations

You have:

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  TASK:                                  │
│                                         │
│  Add lifetime annotations so that:      │
│                                         │
│  1. Declare a lifetime parameter 'a     │
│     (after function name, in <>)        │
│                                         │
│  2. Both inputs have lifetime 'a        │
│     (&'a str)                           │
│                                         │
│  3. Return type has lifetime 'a         │
│     (-> &'a str)                        │
│                                         │
│  The function body stays the same!      │
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
│  1. Where do you declare the lifetime?  │
│     (After fn name: fn longest<'a>)     │
│                                         │
│  2. What syntax annotates a reference?  │
│     (&'a Type)                          │
│                                         │
│  3. How many references need 'a?        │
│     (All three: x, y, and return)       │
│                                         │
│  4. Why the SAME lifetime for all?      │
│     (Return could be either input,      │
│      so it can't outlive either one)    │
│                                         │
│  5. Does the function body change?      │
│     (No! Only the signature)            │
│                                         │
│  6. What does 'a mean conceptually?     │
│     (The overlap of both input          │
│      lifetimes)                         │
│                                         │
└─────────────────────────────────────────┘
```



Add 'a to declare and to all three references!

(Now go to the Editor and try it!)
