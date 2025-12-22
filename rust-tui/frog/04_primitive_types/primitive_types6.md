# Tuple Indexing



## Accessing Tuple Elements

Besides destructuring, you can access by INDEX.

But tuples use DOT notation, not brackets!



```
┌─────────────────────────────────────────┐
│                                         │
│  let tup = (10, 20, 30);                │
│             ↓   ↓   ↓                   │
│            .0  .1  .2   ← indexes       │
│                                         │
│  tup.0 = 10                             │
│  tup.1 = 20                             │
│  tup.2 = 30                             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Dot vs Brackets



```
┌─────────────────────────────────────────┐
│                                         │
│  ARRAYS use BRACKETS:                   │
│                                         │
│    let arr = [1, 2, 3];                 │
│    arr[0]   arr[1]   arr[2]             │
│                                         │
│  TUPLES use DOT:                        │
│                                         │
│    let tup = (1, 2, 3);                 │
│    tup.0    tup.1    tup.2              │
│                                         │
└─────────────────────────────────────────┘
```



Why different?

Arrays can use variables: `arr[i]`

Tuples CANNOT: `tup.i` is invalid!

Tuple index must be a literal number.



--- slide ---

# Visual: Tuple Indexing



```
         numbers = (10, 20, 30)

    ┌─────────┬─────────┬─────────┐
    │   10    │   20    │   30    │
    └─────────┴─────────┴─────────┘
         ↑         ↑         ↑
       .0        .1        .2

    numbers.0  →  10
    numbers.1  →  20
    numbers.2  →  30
```



Remember: First element is `.0` (zero-based!)



--- slide ---

# Common Mistakes



```
┌─────────────────────────────────────────┐
│                                         │
│  MISTAKE #1: Using Brackets             │
│                                         │
│    let tup = (1, 2, 3);                 │
│    let first = tup[0];    ← BRACKETS    │
│                   ↑                     │
│                   ERROR!                │
│                                         │
│  RIGHT:                                 │
│    let first = tup.0;     ← DOT         │
│                   ↑                     │
│                   OK!                   │
└─────────────────────────────────────────┘
```



```
┌─────────────────────────────────────────┐
│  MISTAKE #2: Using Variables            │
│                                         │
│    let i = 0;                           │
│    tup.i                 ← VARIABLE     │
│       ↑                                 │
│       ERROR!                            │
│                                         │
│  Tuples need LITERALS:                  │
│    tup.0                 ← LITERAL      │
│       ↑                                 │
│       OK!                               │
└─────────────────────────────────────────┘
```



--- slide ---

# Think About Your Exercise



The code has:

```rust
let numbers = (1, 2, 3);

// Get the second element
assert_eq!(second, 2);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  numbers = (1, 2, 3)                    │
│             ↓  ↓  ↓                     │
│            .0 .1 .2                     │
│                                         │
│  You want the value 2...                │
│  What INDEX is that?                    │
│                                         │
└─────────────────────────────────────────┘
```



```
┌─────────────────────────────────────────┐
│  QUESTIONS:                             │
│                                         │
│  1. Is "second" at index 1 or 2?        │
│     (Remember: zero-based!)             │
│                                         │
│  2. Do you use brackets or dot?         │
│                                         │
└─────────────────────────────────────────┘
```



(Go try it in the Editor!)
