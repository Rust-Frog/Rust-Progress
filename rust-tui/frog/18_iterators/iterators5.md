# Filter and Count



## Converting Loops to Iterators

Imperative counting loop:

```rust
let mut count = 0;
for val in map.values() {
    if *val == target {
        count += 1;
    }
}
```

Can become:

```rust
map.values()
   .filter(|val| *val == target)
   .count()
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Iterator methods replace the loop:     │
│                                         │
│  .values() - iterate over map values    │
│  .filter() - keep only matching items   │
│  .count()  - count how many remain      │
│                                         │
│  Same result, declarative style!        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The .filter() Method



## Keeping Matching Elements

```rust
let numbers = [1, 2, 3, 4, 5, 6];
let evens: Vec<i32> = numbers
    .iter()
    .filter(|n| *n % 2 == 0)
    .copied()
    .collect();
// [2, 4, 6]
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .filter(|elem| condition)              │
│          ↑      ↑                       │
│          │      └─ Returns bool         │
│          └─ Each element (reference)    │
│                                         │
│  true  → keep element                   │
│  false → discard element                │
│                                         │
│  NOTE: filter receives REFERENCES!      │
│  So you often need *elem to dereference.│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The .count() Method



## Counting Elements

```rust
let numbers = [1, 2, 3, 4, 5];
let count = numbers.iter().count();
// 5

let even_count = numbers
    .iter()
    .filter(|n| *n % 2 == 0)
    .count();
// 2
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .count() consumes the iterator and     │
│  returns how many elements it had.      │
│                                         │
│  Combined with .filter():               │
│  "Count elements matching condition"    │
│                                         │
│  This replaces the counting loop!       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# HashMap Methods



## Iterating Over Maps

```rust
use std::collections::HashMap;

let map: HashMap<String, i32> = ...;

map.keys()    // Iterator over &String
map.values()  // Iterator over &i32
map.iter()    // Iterator over (&String, &i32)
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .values() is perfect when you only     │
│  care about the values, not keys.       │
│                                         │
│  For counting values matching criteria: │
│                                         │
│  map.values()                           │
│     .filter(|&val| val == target)       │
│     .count()                            │
│                                         │
│  Or: .filter(|val| **val == target)     │
│  (double deref: ref to ref to value)    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Dereferencing in Closures



## Understanding the References

```rust
// map.values() yields &V (reference to value)
// filter gets &&V (reference to that reference)

// Option 1: Dereference in pattern
.filter(|&v| v == target)

// Option 2: Dereference in body
.filter(|v| **v == target)

// Option 3: Use == with references
.filter(|v| *v == &target)
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Why so many references?                │
│                                         │
│  values() → &Progress                   │
│  filter(|x|) → x is &(&Progress)        │
│                                         │
│  To compare, you need the actual value. │
│  Dereference until types match!         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Nested Iteration



## flat_map for Collections of Collections

```rust
// Collection of HashMaps
let collection: &[HashMap<String, Progress>] = ...;

// Count across ALL maps
collection.iter()
    .flat_map(|map| map.values())
    .filter(|&v| *v == target)
    .count()
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .flat_map() flattens nested iterators: │
│                                         │
│  [[a, b], [c, d]] → [a, b, c, d]        │
│                                         │
│  For each map, get its values, then     │
│  flatten into single stream.            │
│                                         │
│  Then filter and count as usual!        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Two Functions to Implement

```rust
// Convert this loop to iterator:
fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if *val == value {
            count += 1;
        }
    }
    count
}

// Implement with iterators:
fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // Use .values().filter().count()
}
```



--- slide ---

# Function 1: count_iterator



## Single HashMap

```
┌─────────────────────────────────────────┐
│                                         │
│  The for loop does:                     │
│  1. Iterate over map.values()           │
│  2. Check if *val == value              │
│  3. Count matches                       │
│                                         │
│  Convert to:                            │
│                                         │
│  map.values()                           │
│     .filter(|v| *v == &value)           │
│     .count()                            │
│                                         │
│  Or use pattern matching in filter:     │
│  .filter(|&v| v == &value)              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Function 2: count_collection_iterator



## Slice of HashMaps

```
┌─────────────────────────────────────────┐
│                                         │
│  The nested loop does:                  │
│  1. Iterate over each map in collection │
│  2. For each map, iterate over values   │
│  3. Count all matches                   │
│                                         │
│  Convert to:                            │
│                                         │
│  collection.iter()                      │
│            .flat_map(|map| map.values())│
│            .filter(|v| **v == value)    │
│            .count()                     │
│                                         │
│  flat_map flattens the nested iteration!│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Think It Through



## Before You Type



```
┌─────────────────────────────────────────┐
│  FOR count_iterator:                    │
│                                         │
│  1. How do you iterate over values?     │
│     (map.values())                      │
│                                         │
│  2. How do you keep only matches?       │
│     (.filter(|v| condition))            │
│                                         │
│  3. How do you count the result?        │
│     (.count())                          │
│                                         │
│  FOR count_collection_iterator:         │
│                                         │
│  4. How do you iterate over a slice?    │
│     (collection.iter())                 │
│                                         │
│  5. How do you flatten nested iteration?│
│     (.flat_map(|map| map.values()))     │
│                                         │
│  6. Then filter and count same as above!│
│                                         │
└─────────────────────────────────────────┘
```



Chain .values(), .filter(), .count()!

(Now go to the Editor and try it!)
