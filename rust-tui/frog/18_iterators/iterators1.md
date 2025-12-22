# Iterators: Sequential Access



## What Is an Iterator?

An iterator produces a sequence of values, one at a time:

```rust
let numbers = [1, 2, 3, 4, 5];
let mut iter = numbers.iter();

iter.next()  // Some(&1)
iter.next()  // Some(&2)
iter.next()  // Some(&3)
// ...
```



```
┌─────────────────────────────────────────┐
│                                         │
│  An iterator is like a cursor:          │
│                                         │
│  [1, 2, 3, 4, 5]                         │
│   ↑                                     │
│   Start here                            │
│                                         │
│  Each .next() advances the cursor       │
│  and returns the current element.       │
│                                         │
│  When done, returns None.               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Creating Iterators



## The .iter() Method

Most collections have an `.iter()` method:

```rust
let array = [1, 2, 3];
let iterator = array.iter();

let vector = vec!["a", "b", "c"];
let iterator = vector.iter();
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .iter() creates an iterator that       │
│  BORROWS elements.                      │
│                                         │
│  Returns references: &T                 │
│                                         │
│  Other ways to create iterators:        │
│  • .iter()     → borrows (&T)           │
│  • .iter_mut() → mutable borrows (&mut T)│
│  • .into_iter() → takes ownership (T)   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The next() Method



## Getting Values

```rust
let fruits = ["apple", "banana", "cherry"];
let mut iter = fruits.iter();

iter.next()  // Some(&"apple")
iter.next()  // Some(&"banana")
iter.next()  // Some(&"cherry")
iter.next()  // None - exhausted!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .next() returns Option<&T>:            │
│                                         │
│  Some(&element) → here's the next one   │
│  None → no more elements                │
│                                         │
│  The iterator tracks its position.      │
│  That's why it must be mut!             │
│                                         │
│  let mut iter = ...                     │
│      ↑                                  │
│      Mutable because next() advances it │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Option?



## Safe End Detection

```rust
// next() returns Option, not raw value
iter.next()  // Some(&value) or None

// You always know when you're done:
match iter.next() {
    Some(val) => println!("Got: {}", val),
    None => println!("Done!"),
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Option prevents common bugs:           │
│                                         │
│  • No index out of bounds               │
│  • No null pointer errors               │
│  • Clear "done" signal with None        │
│                                         │
│  The iterator itself knows when         │
│  it's exhausted!                        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# References from iter()



## Borrowing Elements

```rust
let fruits = ["apple", "banana"];
let mut iter = fruits.iter();

let first = iter.next();
// first is Some(&"apple")
//              ↑
//              Reference!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .iter() borrows the collection.        │
│  Each element is a REFERENCE.           │
│                                         │
│  For &str array:                        │
│  • Element type: &str                   │
│  • Iterator yields: &(&str) = &&str     │
│                                         │
│  That's why tests show Some(&"banana")  │
│  not just Some("banana")!               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Working with next()

You have:

```rust
let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];

// TODO: Create an iterator over the array.
let mut fav_fruits_iterator = todo!();

assert_eq!(fav_fruits_iterator.next(), Some(&"banana"));
assert_eq!(fav_fruits_iterator.next(), todo!());
assert_eq!(fav_fruits_iterator.next(), Some(&"avocado"));
// ...
```



--- slide ---

# Task Breakdown



## Three Things to Do

```
┌─────────────────────────────────────────┐
│                                         │
│  1. CREATE THE ITERATOR                 │
│                                         │
│  Replace: let mut fav_fruits_iterator = todo!();│
│  With:    let mut fav_fruits_iterator = ???.iter();│
│                                         │
│  Call .iter() on the array!             │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  2. FILL IN THE SEQUENCE                │
│                                         │
│  The assertions call .next() repeatedly.│
│  What does each call return?            │
│                                         │
│  "banana" → "custard apple" → "avocado" │
│  → "peach" → "raspberry" → None         │
│                                         │
│  Replace todo!() with the correct value!│
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  3. END OF ITERATOR                     │
│                                         │
│  After 5 elements, what does next()     │
│  return? (The iterator is exhausted!)   │
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
│  1. How do you create an iterator?      │
│     (Call .iter() on the collection)    │
│                                         │
│  2. What's the second fruit?            │
│     ("custard apple")                   │
│                                         │
│  3. What format does next() return?     │
│     (Some(&"value") for elements)       │
│                                         │
│  4. What's the fourth fruit?            │
│     ("peach")                           │
│                                         │
│  5. After all 5 fruits, what's next?    │
│     (None - iterator exhausted)         │
│                                         │
│  6. Why is the iterator mut?            │
│     (next() changes internal position)  │
│                                         │
└─────────────────────────────────────────┘
```



Create iterator and fill in the expected values!

(Now go to the Editor and try it!)
