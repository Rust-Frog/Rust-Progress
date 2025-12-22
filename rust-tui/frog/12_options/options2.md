# When match Is Too Much



## Handling Just One Case

Often you only care about ONE variant:

```rust
let maybe_value = Some(42);

match maybe_value {
    Some(v) => println!("Got: {}", v),
    None => {},  // Do nothing... awkward!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  This works, but it's verbose!          │
│                                         │
│  You have to write:                     │
│  • The match keyword                    │
│  • Both arms (even if one is empty)     │
│  • Curly braces for empty arm           │
│                                         │
│  For just "do something if Some",       │
│  there's a cleaner way...               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Enter: if let



## Concise Pattern Matching

`if let` handles ONE pattern concisely:

```rust
let maybe_value = Some(42);

if let Some(v) = maybe_value {
    println!("Got: {}", v);
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  if let PATTERN = EXPRESSION {          │
│      // runs if pattern matches         │
│  }                                      │
│                                         │
│  This reads as:                         │
│  "If maybe_value matches Some(v),       │
│   then run this code with v bound"      │
│                                         │
│  No need to handle the None case!       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# if let Syntax Breakdown



## Understanding the Parts

```rust
if let Some(v) = maybe_value {
    // use v here
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  if let Some(v) = maybe_value { ... }   │
│  ↑  ↑   ↑    ↑  ↑ ↑                     │
│  │  │   │    │  │ └─ The value to match │
│  │  │   │    │  └─ Single equals sign!  │
│  │  │   │    └─ Variable to bind        │
│  │  │   └─ The pattern                  │
│  │  └─ let keyword                      │
│  └─ if keyword                          │
│                                         │
│  NOTE: It's = not ==                    │
│  This is pattern matching, not equality!│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# if let vs match



## Comparison

```rust
// Using match (verbose)
match optional {
    Some(x) => do_something(x),
    None => {},
}

// Using if let (concise)
if let Some(x) = optional {
    do_something(x);
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  USE match WHEN:                        │
│  • You need to handle ALL variants      │
│  • Both branches have meaningful code   │
│                                         │
│  USE if let WHEN:                       │
│  • You only care about ONE variant      │
│  • Other cases can be ignored           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# if let with else



## Handling the Other Case

You CAN add else for the non-matching case:

```rust
if let Some(v) = maybe_value {
    println!("Got: {}", v);
} else {
    println!("Got nothing");
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  if let PATTERN = EXPR {                │
│      // pattern matched                 │
│  } else {                               │
│      // pattern didn't match            │
│  }                                      │
│                                         │
│  The else runs when the pattern         │
│  does NOT match.                        │
│                                         │
│  At this point, match might be clearer! │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Looping: while let



## Pattern Matching in Loops

`while let` keeps looping while pattern matches:

```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("{}", top);
}
// Prints: 3, 2, 1
```



```
┌─────────────────────────────────────────┐
│                                         │
│  while let PATTERN = EXPRESSION {       │
│      // runs while pattern matches      │
│  }                                      │
│                                         │
│  Each iteration:                        │
│  1. Evaluate the expression             │
│  2. Try to match the pattern            │
│  3. If match: run body, repeat          │
│  4. If no match: exit loop              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Understanding Vec::pop()



## Returns Option

The `pop()` method removes and returns the last element:

```rust
let mut vec = vec![1, 2, 3];

vec.pop()  // Returns Some(3)
vec.pop()  // Returns Some(2)
vec.pop()  // Returns Some(1)
vec.pop()  // Returns None (empty!)
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .pop() returns Option<T>               │
│                                         │
│  • Some(value) if vec had elements      │
│  • None if vec was empty                │
│                                         │
│  This is why while let works perfectly: │
│  Loop while we get Some, stop at None.  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Nested Options



## Option Inside Option

What if you have `Vec<Option<T>>`?

```rust
let vec: Vec<Option<i32>> = vec![Some(1), Some(2), None];
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The vector contains Option values.     │
│                                         │
│  When you pop():                        │
│                                         │
│  vec.pop() returns Option<Option<i32>>! │
│                                         │
│  • Some(Some(1)) - popped Some(1)       │
│  • Some(None) - popped a None value     │
│  • None - vec was empty                 │
│                                         │
│  Two layers of Option!                  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Nested Pattern Matching



## Matching Multiple Layers

You can match nested patterns:

```rust
if let Some(Some(value)) = nested_option {
    // value is the inner value
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Some(Some(value))                      │
│  ↑    ↑    ↑                            │
│  │    │    └─ Inner value extracted     │
│  │    └─ Inner Some pattern             │
│  └─ Outer Some pattern                  │
│                                         │
│  This matches:                          │
│    Some(Some(42)) → value = 42          │
│                                         │
│  This does NOT match:                   │
│    Some(None) → pattern fails           │
│    None → pattern fails                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# while let with Nested Patterns



## Combining Concepts

```rust
let mut vec: Vec<Option<i32>> = vec![None, Some(1), Some(2)];

while let Some(Some(value)) = vec.pop() {
    println!("{}", value);
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  ITERATION 1:                           │
│    pop() → Some(Some(2))                │
│    Pattern Some(Some(value)) matches!   │
│    value = 2, body runs                 │
│                                         │
│  ITERATION 2:                           │
│    pop() → Some(Some(1))                │
│    Pattern matches! value = 1           │
│                                         │
│  ITERATION 3:                           │
│    pop() → Some(None)                   │
│    Pattern Some(Some(value)) fails!     │
│    Loop exits                           │
│                                         │
│  The None in the vec stopped the loop.  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Pattern Placement



## The Syntax Is Specific

```rust
// CORRECT:
if let Some(x) = value { ... }
while let Some(x) = value { ... }

// WRONG:
if Some(x) = value { ... }  // Missing 'let'!
if let Some(x) == value { ... }  // Wrong operator!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Common mistakes:                       │
│                                         │
│  1. Forgetting 'let' after if/while     │
│                                         │
│  2. Using == instead of =               │
│     (= is for pattern binding)          │
│                                         │
│  3. Swapping sides:                     │
│     if let value = Some(x)  ← WRONG!    │
│     if let Some(x) = value  ← CORRECT   │
│                                         │
│  Pattern on LEFT, value on RIGHT.       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise Part 1



## Fix the if let

The code has incomplete syntax:

```rust
word = optional_target {
    assert_eq!(word, target);
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  This is ALMOST an if let statement.    │
│                                         │
│  What's missing?                        │
│                                         │
│  • The keyword(s) at the start          │
│  • The pattern to match                 │
│                                         │
│  You need to turn this into:            │
│                                         │
│  [keywords] [pattern] = optional_target │
│                                         │
│  What keywords start an if let?         │
│  What pattern extracts from Some?       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise Part 2



## Fix the while let

```rust
integer = optional_integers.pop() {
    assert_eq!(integer, cursor);
    cursor -= 1;
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  This should be a while let loop.       │
│                                         │
│  CRITICAL INSIGHT:                      │
│                                         │
│  optional_integers is Vec<Option<i8>>   │
│  .pop() returns Option<Option<i8>>      │
│                                         │
│  You need to match BOTH layers:         │
│  • Outer Some (pop succeeded)           │
│  • Inner Some (actual integer value)    │
│                                         │
│  The pattern needs to be NESTED!        │
│                                         │
│  What pattern extracts the integer      │
│  from Option<Option<i8>>?               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Analyzing the Data



## What Gets Popped?

```rust
let mut optional_integers: Vec<Option<i8>> = vec![None];

for i in 1..=range {
    optional_integers.push(Some(i));
}
// Vec is: [None, Some(1), Some(2), ..., Some(10)]
```



```
┌─────────────────────────────────────────┐
│                                         │
│  pop() order (last to first):           │
│                                         │
│  pop() → Some(Some(10))  ← match!       │
│  pop() → Some(Some(9))   ← match!       │
│  ...                                    │
│  pop() → Some(Some(1))   ← match!       │
│  pop() → Some(None)      ← should stop! │
│                                         │
│  The pattern must:                      │
│  • Match Some(Some(integer))            │
│  • Fail on Some(None)                   │
│                                         │
│  This stops the loop at the right time! │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Think It Through



## Before You Type



```
┌─────────────────────────────────────────┐
│  FOR THE if let:                        │
│                                         │
│  1. What two keywords start an if let?  │
│                                         │
│  2. optional_target is Some(target).    │
│     What pattern extracts `word`?       │
│                                         │
│  3. Pattern goes on which side of =?    │
│                                         │
├─────────────────────────────────────────┤
│  FOR THE while let:                     │
│                                         │
│  1. What two keywords start while let?  │
│                                         │
│  2. .pop() on Vec<Option<i8>> returns   │
│     what type?                          │
│                                         │
│  3. What NESTED pattern extracts the    │
│     integer from two layers of Option?  │
│                                         │
│  4. Why does Some(None) stop the loop?  │
│                                         │
└─────────────────────────────────────────┘
```



Two incomplete statements need completing!

(Now go to the Editor and try it!)
