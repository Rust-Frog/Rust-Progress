# Slices



## What is a Slice?

A slice is a VIEW into a portion of an array.



```
Array:  [10, 20, 30, 40, 50]
              ↓   ↓
Slice:       [20, 30]
```



A slice doesn't own the data - it just references part of it.

Think of it like looking through a window at part of an array.



--- slide ---

# Slice Syntax



## Creating a Slice

Use a RANGE inside square brackets:

```rust
let arr = [10, 20, 30, 40, 50];
let slice = &arr[1..3];
//               ↑  ↑
//             start end (exclusive)
```



The `&` means "reference to" - a slice borrows from the array.



## Range Rules

```
┌────────────────────────────────────────────────┐
│                                                │
│  [start..end]   From start to end-1            │
│  [start..]      From start to the end          │
│  [..end]        From beginning to end-1        │
│  [..]           The whole thing                │
│                                                │
│  Start is INCLUSIVE, end is EXCLUSIVE          │
│                                                │
└────────────────────────────────────────────────┘
```



--- slide ---

# Understanding Ranges



The tricky part: **the end is NOT included!**



```
┌────────────────────────────────────────────────┐
│                                                │
│  If you want elements at index 1, 2, 3...      │
│                                                │
│  You write: [1..4]                             │
│                  ↑                             │
│                  One MORE than the last index  │
│                                                │
│  Because 4 is EXCLUDED                         │
│                                                │
└────────────────────────────────────────────────┘
```



This is called a "half-open range" - common in programming!



## Quick Pattern

Want indexes from A to B (inclusive)?

Use `[A..B+1]`



--- slide ---

# Think About Your Exercise



Look at the test:

```rust
let a = [1, 2, 3, 4, 5];
//       0  1  2  3  4  ← indexes

// What slice gives you [2, 3, 4]?
```



```
┌────────────────────────────────────────────────┐
│                                                │
│  QUESTIONS:                                    │
│                                                │
│  1. What value do you WANT to start with?      │
│     What INDEX is that at?                     │
│                                                │
│  2. What value do you WANT to end with?        │
│     What INDEX is that at?                     │
│                                                │
│  3. If end is EXCLUSIVE, what number do you    │
│     need to use to INCLUDE that last value?    │
│                                                │
│  4. Don't forget the & for a reference!        │
│                                                │
└────────────────────────────────────────────────┘
```



Figure out the start and end indexes yourself!



(Go try it in the Editor!)
