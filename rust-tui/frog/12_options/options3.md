# Pattern Matching and Ownership



## match Can Move Values

When you pattern match, you might MOVE values:

```rust
let optional = Some(String::from("hello"));

match optional {
    Some(s) => println!("{}", s),  // s MOVED here!
    None => {},
}

// Can we use `optional` again?
```



```
┌─────────────────────────────────────────┐
│                                         │
│  When you match Some(s), the value      │
│  inside is MOVED into s.                │
│                                         │
│  After the match:                       │
│  • s owned the String (in that arm)     │
│  • optional no longer owns it           │
│  • optional can't be used again!        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Problem



## Using After match

```rust
let optional_point = Some(Point { x: 100, y: 200 });

match optional_point {
    Some(p) => println!("{}, {}", p.x, p.y),
    None => {},
}

println!("{:?}", optional_point);  // ERROR!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  COMPILER ERROR:                        │
│                                         │
│  "value borrowed after move"            │
│  or                                     │
│  "use of moved value"                   │
│                                         │
│  The match MOVED the Point out of       │
│  optional_point into p.                 │
│                                         │
│  After the match, optional_point is     │
│  no longer valid!                       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Does match Move?



## Default Binding Mode

By default, pattern matching BINDS BY VALUE:

```rust
Some(p)
     ↑
     p takes ownership of the value
```



```
┌─────────────────────────────────────────┐
│                                         │
│  BINDING BY VALUE (default):            │
│                                         │
│  Some(p) moves the inner value to p     │
│                                         │
│  This is often what you want:           │
│  • Extract the value                    │
│  • Work with it                         │
│  • Maybe return it                      │
│                                         │
│  But sometimes you just want to LOOK    │
│  at the value without taking it!        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The ref Keyword



## Borrowing in Patterns

Use `ref` to BORROW instead of MOVE:

```rust
let optional = Some(String::from("hello"));

match optional {
    Some(ref s) => println!("{}", s),  // s is &String!
    None => {},
}

// optional is still valid!
println!("{:?}", optional);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Some(ref s)                            │
│       ↑↑↑                               │
│       Creates a REFERENCE to the value  │
│                                         │
│  s is now &String, not String           │
│  The original stays in place!           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# ref vs &



## Different Positions, Same Idea

```rust
// In expressions: & creates a reference
let r = &some_value;

// In patterns: ref creates a reference
match optional {
    Some(ref r) => { /* r is a reference */ }
    None => {}
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  & goes BEFORE the value (expression)   │
│    &value → creates reference TO value  │
│                                         │
│  ref goes BEFORE the binding (pattern)  │
│    ref x → x becomes reference to value │
│                                         │
│  PATTERN SIDE vs EXPRESSION SIDE        │
│                                         │
│  In patterns, you're describing         │
│  how to BIND, not what to reference.    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Visualizing ref



## What Happens in Memory

```rust
let opt = Some(Point { x: 1, y: 2 });
```

Without ref:

```
┌─────────────────────────────────────────┐
│                                         │
│  match opt {                            │
│      Some(p) => { }  // p owns Point    │
│  }                                      │
│                                         │
│  opt: [MOVED] ─────────┐                │
│                        ↓                │
│  p: Point { x: 1, y: 2 }                │
│                                         │
│  opt is now empty/invalid!              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Visualizing ref (continued)



## With ref

```rust
let opt = Some(Point { x: 1, y: 2 });
```

With ref:

```
┌─────────────────────────────────────────┐
│                                         │
│  match opt {                            │
│      Some(ref p) => { }  // p borrows   │
│  }                                      │
│                                         │
│  opt: Some(Point { x: 1, y: 2 })        │
│             ↑                           │
│  p: &───────┘  (reference to the Point) │
│                                         │
│  opt still owns the Point!              │
│  p just borrows it.                     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# When to Use ref



## Common Scenarios

```
┌─────────────────────────────────────────┐
│                                         │
│  USE ref WHEN:                          │
│                                         │
│  1. You need to use the Option AFTER    │
│     the match                           │
│                                         │
│  2. You only want to READ the value,    │
│     not take ownership                  │
│                                         │
│  3. You want to PRINT or DEBUG the      │
│     original Option after matching      │
│                                         │
│  4. Multiple matches on same Option     │
│     (first match shouldn't consume it)  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# ref mut



## Mutable Borrowing in Patterns

Need to MODIFY the inner value?

```rust
let mut optional = Some(42);

match optional {
    Some(ref mut n) => *n += 1,  // Modify in place!
    None => {},
}

// optional is now Some(43)
```



```
┌─────────────────────────────────────────┐
│                                         │
│  ref mut creates a mutable reference    │
│                                         │
│  Some(ref mut n)                        │
│       ↑↑↑ ↑↑↑                           │
│       │   └─ mutable                    │
│       └─ reference                      │
│                                         │
│  n is &mut i32, can modify via *n       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Alternative: Match on Reference



## Another Approach

Instead of ref in pattern, match on a reference:

```rust
let optional = Some(Point { x: 1, y: 2 });

match &optional {  // Match on REFERENCE to optional
    Some(p) => println!("{}, {}", p.x, p.y),
    None => {},
}

// optional still valid!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  match &optional                        │
│        ↑                                │
│        Match on a reference!            │
│                                         │
│  When you match on &Option<T>,          │
│  the patterns automatically borrow.     │
│                                         │
│  p becomes &Point (not Point)           │
│  No explicit ref needed!                │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Two Solutions



## ref vs &optional

Both achieve the same goal:

```rust
// Solution 1: ref in pattern
match optional {
    Some(ref p) => { /* p is &Point */ }
    None => {}
}

// Solution 2: match on reference
match &optional {
    Some(p) => { /* p is &Point */ }
    None => {}
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Both keep optional valid after match!  │
│                                         │
│  Modern Rust often prefers &optional    │
│  but ref is still useful and valid.     │
│                                         │
│  The exercise teaches ref explicitly.   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Copy Types



## When Move Doesn't Apply

Some types implement Copy (integers, bools, etc.):

```rust
let optional = Some(42);  // i32 is Copy

match optional {
    Some(n) => println!("{}", n),
    None => {},
}

println!("{:?}", optional);  // Works! i32 was copied
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Copy types are duplicated, not moved.  │
│                                         │
│  • Integers (i32, u64, etc.)            │
│  • bool                                 │
│  • char                                 │
│  • Tuples of Copy types                 │
│                                         │
│  For these, ref isn't needed.           │
│  The value is automatically copied.     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Non-Copy Types



## When ref Matters

Structs, Strings, Vecs are NOT Copy by default:

```rust
let optional = Some(String::from("hello"));

match optional {
    Some(s) => println!("{}", s),  // String MOVED
    None => {},
}

println!("{:?}", optional);  // ERROR! String moved
```



```
┌─────────────────────────────────────────┐
│                                         │
│  For non-Copy types:                    │
│                                         │
│  • String                               │
│  • Vec<T>                               │
│  • Custom structs (by default)          │
│  • Box<T>                               │
│                                         │
│  Use ref (or match on reference) to     │
│  keep the original Option usable.       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Keep the Option Alive

Look at the code:

```rust
let optional_point = Some(Point { x: 100, y: 200 });

match optional_point {
    Some(p) => println!("Coordinates are {},{}", p.x, p.y),
    _ => panic!("No match!"),
}

println!("{optional_point:?}");  // Uses optional_point!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  THE PROBLEM:                           │
│                                         │
│  1. match extracts p from optional_point│
│     This MOVES the Point!               │
│                                         │
│  2. Later, println! tries to use        │
│     optional_point                      │
│                                         │
│  3. But it was moved! Compiler error!   │
│                                         │
│  You need to BORROW in the pattern      │
│  instead of MOVE.                       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Fix Location



## Where to Add What

```rust
match optional_point {
    Some(p) => println!("Coordinates are {},{}", p.x, p.y),
    //   ↑
    //   Something needs to change here!
    _ => panic!("No match!"),
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The pattern Some(p) needs modification.│
│                                         │
│  Currently: Some(p) moves the Point     │
│  Needed: Some(???) borrows the Point    │
│                                         │
│  What keyword creates a reference       │
│  in a pattern?                          │
│                                         │
│  Hint: It's 3 letters and goes          │
│  BEFORE the variable name.              │
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
│  1. Why does the code fail to compile?  │
│     (What got moved? What needs it?)    │
│                                         │
│  2. Is Point a Copy type?               │
│     (Custom structs usually aren't!)    │
│                                         │
│  3. What keyword in a pattern creates   │
│     a reference instead of moving?      │
│                                         │
│  4. Where exactly in the pattern do     │
│     you add this keyword?               │
│     Some(___) → Some(___ p)             │
│                                         │
│  5. After the change, what type is p?   │
│     (Value type or reference type?)     │
│                                         │
│  6. Can you still access p.x and p.y?   │
│     (Hint: yes! Rust auto-derefs)       │
│                                         │
└─────────────────────────────────────────┘
```



A small addition prevents the move!

(Now go to the Editor and try it!)
