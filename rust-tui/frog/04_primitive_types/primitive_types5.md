# Tuple Destructuring



## What is a Tuple?

A tuple groups multiple values of DIFFERENT types.



```
┌─────────────────────────────────────────┐
│              TUPLE                      │
│                                         │
│    ( "Alice" ,   25   ,  true )         │
│        ↓         ↓        ↓             │
│      &str      i32      bool            │
│                                         │
│  Different types in one container!      │
└─────────────────────────────────────────┘
```



Unlike arrays (same type), tuples can mix types!



--- slide ---

# Creating Tuples



## Visual

```
┌─────────────────────────────────────────┐
│                                         │
│  Array:   [ 1, 2, 3 ]   ← same types    │
│                                         │
│  Tuple:   ( 1, "hi", true )  ← mixed!   │
│                                         │
│  Arrays use [ ]                         │
│  Tuples use ( )                         │
│                                         │
└─────────────────────────────────────────┘
```



## Syntax

```rust
let pair = (10, 20);
let info = ("Bob", 3.5, 'A');
```



## With Type Annotation

```rust
let pair: (i32, i32) = (10, 20);
```



--- slide ---

# Destructuring



## What is Destructuring?

Taking a tuple APART into separate pieces!



```
    BEFORE                    AFTER
┌─────────────┐
│ (100, 250)  │         x = 100
│             │  ───→
│   point     │         y = 250
└─────────────┘
   One tuple            Two variables!
```



## The Syntax

```rust
let point = (100, 250);

let (x, y) = point;
//   ↑  ↑
//   Pattern matches the shape
```



--- slide ---

# How Destructuring Works



```
                  let (x, y) = point;

       Pattern         =        Value
      ┌───┬───┐               ┌───┬───┐
      │ x │ y │      ←─────   │100│250│
      └───┴───┘               └───┴───┘
        │                       │
        └───────────────────────┘
              x gets 100
              y gets 250
```



The pattern must match the tuple's SHAPE:

```rust
let triple = (1, 2, 3);
let (a, b, c) = triple;  // ✓ 3 = 3
let (x, y) = triple;     // ✗ 2 ≠ 3 ERROR!
```



--- slide ---

# Ignoring Values



Use `_` to skip parts you don't need:



```
let data = ("Alice", 25, true);

let (first, _,  _ ) = data;
      ↓     ↓   ↓
    keep  skip skip

first = "Alice"
```



```rust
let (first, _, _) = data;  // Only keep first
```



--- slide ---

# Think About Your Exercise



Look at the code:

```rust
let cat = ("Furry McFurson", 3.5);

println!("{name} is {age} years old");
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The tuple has: (string, number)        │
│                                         │
│  println needs: name and age            │
│                                         │
│  Your job: create those variables!      │
│                                         │
└─────────────────────────────────────────┘
```



```
┌─────────────────────────────────────────┐
│  QUESTIONS:                             │
│                                         │
│  1. How many elements in the tuple?     │
│  2. What pattern shape do you need?     │
│  3. What variable names go in pattern?  │
└─────────────────────────────────────────┘
```



(Go try it in the Editor!)
