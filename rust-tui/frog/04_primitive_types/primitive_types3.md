# Arrays



## What is an Array?

An array is a collection of items of the SAME type.



```
┌───┬───┬───┬───┬───┐
│ 1 │ 2 │ 3 │ 4 │ 5 │
└───┴───┴───┴───┴───┘
  0   1   2   3   4   ← indexes
```



All elements must be the same type.

Arrays have a FIXED size (known at compile time).



--- slide ---

# Array Syntax



## Creating Arrays

List elements in square brackets:

```rust
let numbers = [1, 2, 3, 4, 5];
let names = ["Alice", "Bob", "Charlie"];
```



## With Type Annotation

```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
//          [type; length]
```



## Repeat Syntax

Create an array of repeated values:

```rust
let zeros = [0; 10];   // Ten zeros
let ones = [1; 100];   // A hundred ones
```



--- slide ---

# Array Properties



## Accessing Elements

Use index with square brackets (0-indexed):

```rust
let arr = [10, 20, 30];
let first = arr[0];   // 10
let second = arr[1];  // 20
```



## Getting Length

```rust
let arr = [1, 2, 3, 4, 5];
let len = arr.len();  // 5
```



## Fixed Size

Arrays can't grow or shrink!

```rust
let arr = [1, 2, 3];
// arr.push(4);  // ERROR! No push on arrays
```



--- slide ---

# Think About Your Exercise



The code checks if your array has 100+ elements.



```rust
if a.len() >= 100 {
    println!("Wow, that's a big array!");
}
```



You could type out 100 elements... but that's tedious!

Remember the repeat syntax?



```
┌────────────────────────────────────────────────┐
│                                                │
│  QUESTIONS:                                    │
│                                                │
│  1. How do you create an array with many       │
│     elements of the same value?                │
│                                                │
│  2. What syntax creates an array of 100        │
│     zeros? (or any value repeated 100 times?)  │
│                                                │
│  3. What type will this array have?            │
│                                                │
└────────────────────────────────────────────────┘
```



(Go try it in the Editor!)
