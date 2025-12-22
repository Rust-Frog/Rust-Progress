# Working with Vectors



## This Exercise Has TWO Parts

You'll learn two different ways to transform data:

1. **Loop + Push** - The imperative style
2. **Iterator + Map** - The functional style

Both achieve the same result!



```
┌─────────────────────────────────────────┐
│                                         │
│  Input:  [2, 4, 6, 8, 10]               │
│                                         │
│  Task:   Multiply each by 2             │
│                                         │
│  Output: [4, 8, 12, 16, 20]             │
│                                         │
│  Two ways to do it, same result!        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Part 1: The Loop Approach



## vec_loop Function

This is the classic, step-by-step approach.

You control every action explicitly.



```
┌─────────────────────────────────────────┐
│                                         │
│  ALGORITHM:                             │
│                                         │
│  1. Create an empty output vector       │
│  2. Loop through each input element     │
│  3. Transform the element               │
│  4. Push the result to output           │
│  5. Return the output                   │
│                                         │
└─────────────────────────────────────────┘
```



This is how you might do it in C, Java, or Python.



--- slide ---

# The Loop Pattern



## Step by Step

```rust
let mut output = Vec::new();  // 1. Empty vector

for element in input {         // 2. Loop
    // 3. Transform and 4. Push
    output.push(/* transformed value */);
}

output                         // 5. Return
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Why `mut`?                             │
│                                         │
│  Because we're CHANGING output!         │
│  We keep pushing new elements into it.  │
│                                         │
│  Without `mut`, Rust won't let you      │
│  modify the vector.                     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Understanding push()



## Adding to a Vector

The `.push()` method adds ONE element to the end.



```
┌───────────────────────────────────────────┐
│                                           │
│  let mut v = Vec::new();   // []          │
│                                           │
│  v.push(100);              // [100]       │
│  v.push(200);              // [100,200]   │
│  v.push(300);              // [100,200,300]
│                                           │
│  Each push APPENDS to the vector.         │
│                                           │
└───────────────────────────────────────────┘
```



```
┌─────────────────────────────────────────┐
│                                         │
│  VISUAL:                                │
│                                         │
│  push(100):  [ 100 |     |     ]        │
│                 ↑                       │
│                                         │
│  push(200):  [ 100 | 200 |     ]        │
│                       ↑                 │
│                                         │
│  push(300):  [ 100 | 200 | 300 ]        │
│                             ↑           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Tracing Through the Loop



## What Happens Step by Step

If input is `[2, 4, 6]` and we multiply by 2:



```
┌─────────────────────────────────────────┐
│                                         │
│  START:  output = []                    │
│                                         │
│  Loop iteration 1:                      │
│    element = 2                          │
│    push(2 * 2) → push(4)                │
│    output = [4]                         │
│                                         │
│  Loop iteration 2:                      │
│    element = 4                          │
│    push(4 * 2) → push(8)                │
│    output = [4, 8]                      │
│                                         │
│  Loop iteration 3:                      │
│    element = 6                          │
│    push(6 * 2) → push(12)               │
│    output = [4, 8, 12]                  │
│                                         │
│  END: return [4, 8, 12]                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Think About vec_loop



Look at the code:

```rust
for element in input {
    // TODO: Multiply each element by 2
    // and push to output
}
```



```
┌─────────────────────────────────────────┐
│  QUESTIONS:                             │
│                                         │
│  1. `element` is each number from input │
│     How do you double it?               │
│                                         │
│  2. Once you have the doubled value,    │
│     how do you add it to output?        │
│                                         │
│  3. What goes inside the loop body?     │
│     (One line is enough!)               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Part 2: The Iterator Approach



## vec_map Function

Rust has a powerful FUNCTIONAL style too.

Instead of explicit loops, you chain methods.



```
┌─────────────────────────────────────────┐
│                                         │
│  IMPERATIVE (loop):                     │
│  "Go through each item, do this, do     │
│   that, put it here..."                 │
│                                         │
│  FUNCTIONAL (iterator):                 │
│  "Transform all items like this."       │
│                                         │
│  Same result, different philosophy!     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Iterator Chain



## Three Steps

```rust
input.iter().map(|x| /* transform */).collect()
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .iter()                                │
│       ↓                                 │
│  Creates an iterator over the elements  │
│                                         │
│  .map(|x| ...)                          │
│       ↓                                 │
│  Transforms each element                │
│                                         │
│  .collect()                             │
│       ↓                                 │
│  Gathers results into a new Vec         │
│                                         │
└─────────────────────────────────────────┘
```



It's like a pipeline: data flows through!



--- slide ---

# Understanding .iter()



## Creating an Iterator

When you call `.iter()`, you get an iterator.

An iterator is a thing that can give you elements one by one.



```
┌─────────────────────────────────────────┐
│                                         │
│  [1, 2, 3].iter()                       │
│                                         │
│  Creates:  Iterator → 1 → 2 → 3 → done  │
│                                         │
│  The iterator yields each element       │
│  when asked.                            │
│                                         │
└─────────────────────────────────────────┘
```



By itself, `.iter()` doesn't DO anything yet.

It just sets up the data source.



--- slide ---

# Understanding .map()



## Transforming Each Element

`.map()` takes a CLOSURE (mini-function).

It applies that closure to EVERY element.



```rust
.map(|x| x + 1)
     ↑↑↑  ↑↑↑↑↑
     │    │
     │    The transformation expression
     │
     The parameter (each element)
```



```
┌─────────────────────────────────────────┐
│                                         │
│  |x| x + 1  is a CLOSURE                │
│                                         │
│  It's like a tiny inline function:      │
│  "For any x, return x + 1"              │
│                                         │
│  Input: 5  →  Output: 6                 │
│  Input: 10 →  Output: 11                │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Closure Syntax Deep Dive



## The Pipes Are Parameters

```rust
|x| x * 2
```



```
┌─────────────────────────────────────────┐
│                                         │
│  |x|           ← Parameter declaration  │
│                   (like function args)  │
│                                         │
│  x * 2         ← Body of the closure    │
│                   (what gets returned)  │
│                                         │
│  Equivalent function:                   │
│  fn double(x: i32) -> i32 {             │
│      x * 2                              │
│  }                                      │
│                                         │
│  But closures are inline and short!     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Understanding .collect()



## Gathering the Results

`.map()` produces an iterator of transformed values.

`.collect()` turns that iterator into an actual Vec.



```
┌─────────────────────────────────────────┐
│                                         │
│  Without .collect():                    │
│    You have an iterator (lazy)          │
│    Nothing is actually computed yet!    │
│                                         │
│  With .collect():                       │
│    Forces computation                   │
│    Produces a real Vec                  │
│                                         │
└─────────────────────────────────────────┘
```



```rust
[1, 2, 3].iter()
         .map(|x| x + 1)
         .collect()    // → Vec [2, 3, 4]
```



--- slide ---

# The Example in Your Code



## vec_map_example Shows How

```rust
input.iter().map(|element| element + 1).collect()
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Input: [1, 2, 3]                       │
│                                         │
│  Step 1: .iter()                        │
│    Iterator over 1, 2, 3                │
│                                         │
│  Step 2: .map(|element| element + 1)    │
│    1 → 2                                │
│    2 → 3                                │
│    3 → 4                                │
│                                         │
│  Step 3: .collect()                     │
│    Gather into Vec                      │
│                                         │
│  Output: [2, 3, 4]                      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Think About vec_map



The example adds 1 to each element.

YOU need to multiply by 2 instead!



```rust
input
    .iter()
    .map(|element| {
        // What goes here?
    })
    .collect()
```



```
┌─────────────────────────────────────────┐
│  QUESTIONS:                             │
│                                         │
│  1. The example uses: element + 1       │
│     What's YOUR expression?             │
│                                         │
│  2. If doubling means "times 2"...      │
│     How do you write that in Rust?      │
│                                         │
│  3. Look at the closure in the example  │
│     and modify it for your task.        │
│                                         │
└─────────────────────────────────────────┘
```



(Go solve BOTH functions in the Editor!)
