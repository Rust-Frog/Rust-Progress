# Iterator Methods



## Transforming Data

Iterators have powerful methods for transformation:

```rust
let numbers = [1, 2, 3];
let doubled: Vec<i32> = numbers
    .iter()
    .map(|x| x * 2)
    .collect();
// [2, 4, 6]
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Iterator chains:                       │
│                                         │
│  .iter()     → create iterator          │
│  .map(...)   → transform each element   │
│  .collect()  → gather into collection   │
│                                         │
│  No loops needed!                       │
│  Declarative, functional style.         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The .chars() Iterator



## Iterating Over Characters

Strings have a `.chars()` method:

```rust
let word = "hello";
let mut chars = word.chars();

chars.next()  // Some('h')
chars.next()  // Some('e')
chars.next()  // Some('l')
// ...
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .chars() creates an iterator over      │
│  the characters in a string.            │
│                                         │
│  Returns char values, not references!   │
│                                         │
│  Useful for:                            │
│  • Examining each character             │
│  • Transforming text                    │
│  • Parsing strings                      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Character Case Conversion



## to_uppercase()

```rust
let c = 'h';
let upper = c.to_uppercase();
// Returns an iterator of chars!

// For simple ASCII:
let upper: String = c.to_uppercase().collect();
// "H"
```



```
┌─────────────────────────────────────────┐
│                                         │
│  char::to_uppercase() returns an        │
│  ITERATOR, not a single char!           │
│                                         │
│  Why? Some characters map to multiple:  │
│  'ß'.to_uppercase() → "SS"              │
│                                         │
│  For simple cases, collect or use       │
│  .to_ascii_uppercase() for a char.      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Collecting Iterators



## .collect() Method

```rust
let chars_iter = "abc".chars();
let collected: String = chars_iter.collect();
// "abc"

let numbers = [1, 2, 3];
let vec: Vec<i32> = numbers.iter().copied().collect();
// vec![1, 2, 3]
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .collect() gathers iterator elements   │
│  into a collection.                     │
│                                         │
│  The return type determines the         │
│  collection type:                       │
│                                         │
│  let s: String = iter.collect();        │
│  let v: Vec<T> = iter.collect();        │
│                                         │
│  Type annotation is often needed!       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Chaining Characters



## Building Strings

```rust
// Get first char uppercase, rest unchanged
let word = "hello";
let mut chars = word.chars();

match chars.next() {
    None => String::new(),
    Some(first) => {
        // first.to_uppercase() + remaining chars
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  To capitalize first letter:            │
│                                         │
│  1. Get the first char                  │
│  2. Convert to uppercase                │
│  3. Chain with remaining chars          │
│  4. Collect into String                 │
│                                         │
│  first.to_uppercase().chain(chars)      │
│                      .collect()         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The .chain() Method



## Connecting Iterators

```rust
let first = "AB".chars();
let second = "cd".chars();
let combined: String = first.chain(second).collect();
// "ABcd"
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .chain(other) connects two iterators:  │
│                                         │
│  [A, B].chain([c, d])                   │
│    →  [A, B, c, d]                      │
│                                         │
│  First exhausts the first iterator,     │
│  then continues with the second.        │
│                                         │
│  Perfect for prepending or appending!   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The .map() Method



## Transform Each Element

```rust
let words = ["hello", "world"];
let capitalized: Vec<String> = words
    .iter()
    .map(|word| capitalize_first(word))
    .collect();
// ["Hello", "World"]
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .map(|elem| transform(elem))           │
│       ↑      ↑                          │
│       │      └─ What to do with it      │
│       └─ Each element                   │
│                                         │
│  Applies a function to every element.   │
│  Creates a new iterator with results.   │
│                                         │
│  Original data unchanged!               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Three Functions

```rust
// 1. Capitalize first character
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => todo!(),  // Combine uppercase first + rest
    }
}

// 2. Apply to each word, return Vec
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // Use .iter().map().collect()
}

// 3. Apply to each word, return single String
fn capitalize_words_string(words: &[&str]) -> String {
    // Use .iter().map().collect()
}
```



--- slide ---

# Task 1: capitalize_first



## Uppercase + Rest

```
┌─────────────────────────────────────────┐
│                                         │
│  You have:                              │
│  • first: the first char                │
│  • chars: remaining characters iterator │
│                                         │
│  Need to:                               │
│  1. Convert first to uppercase          │
│     first.to_uppercase()                │
│                                         │
│  2. Chain with remaining chars          │
│     .chain(chars)                       │
│                                         │
│  3. Collect into String                 │
│     .collect()                          │
│                                         │
│  Put it together:                       │
│  first.to_uppercase().chain(chars).collect()│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Task 2 & 3: Applying to Collections



## map + collect

```
┌─────────────────────────────────────────┐
│                                         │
│  capitalize_words_vector:               │
│                                         │
│  words.iter()              // Iterator  │
│       .map(|w| capitalize_first(w))     │
│       .collect()           // Vec<String>│
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  capitalize_words_string:               │
│                                         │
│  Same pattern, but collect to String!   │
│                                         │
│  When you collect chars to String,      │
│  they concatenate together.             │
│                                         │
│  .collect::<String>() or annotate type  │
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
│  FOR capitalize_first:                  │
│                                         │
│  1. How do you uppercase a char?        │
│     (char.to_uppercase())               │
│                                         │
│  2. How do you combine two iterators?   │
│     (.chain())                          │
│                                         │
│  3. How do you make a String from iter? │
│     (.collect())                        │
│                                         │
│  FOR capitalize_words_vector:           │
│                                         │
│  4. How do you transform each element?  │
│     (.map())                            │
│                                         │
│  FOR capitalize_words_string:           │
│                                         │
│  5. What's different about collecting   │
│     into String vs Vec?                 │
│     (Just the return type annotation!)  │
│                                         │
└─────────────────────────────────────────┘
```



Chain iterators and map over collections!

(Now go to the Editor and try it!)
