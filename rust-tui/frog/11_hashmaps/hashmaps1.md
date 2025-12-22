# The Lookup Problem



## Finding Things Quickly

Imagine you have a list of 1 million names.

Someone asks: "Is 'Alice' in the list?"



```
┌─────────────────────────────────────────┐
│                                         │
│  WITH A VECTOR:                         │
│                                         │
│  You'd have to check EVERY element.     │
│  Start at index 0, check, move to 1...  │
│  Worst case: 1 million checks!          │
│                                         │
│  This is called O(n) - "linear time"    │
│  More items = proportionally slower     │
│                                         │
└─────────────────────────────────────────┘
```



What if you could find ANY item instantly,

regardless of how many items exist?



--- slide ---

# Enter: HashMap



## The Key-Value Store

A **HashMap** stores data in **key-value pairs**.

Given a key, it finds the value almost instantly.



```
┌─────────────────────────────────────────┐
│                                         │
│  HashMap                                │
│  ┌────────────┬─────────────┐           │
│  │    KEY     │    VALUE    │           │
│  ├────────────┼─────────────┤           │
│  │  "apple"   │      5      │           │
│  │  "banana"  │      3      │           │
│  │  "orange"  │      7      │           │
│  └────────────┴─────────────┘           │
│                                         │
│  "How many apples?" → Look up "apple"   │
│                     → Get 5 instantly!  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# How Is It So Fast?



## The Magic of Hashing

HashMap uses a **hash function** to convert

keys into array indices:



```
┌─────────────────────────────────────────┐
│                                         │
│  "apple" ──→ hash() ──→ 42              │
│                         ↓               │
│              storage[42] = 5            │
│                                         │
│  When you look up "apple":              │
│    1. Hash "apple" → 42                 │
│    2. Go directly to storage[42]        │
│    3. Return the value: 5               │
│                                         │
│  No searching! Direct access!           │
│  O(1) - "constant time"                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Real World Examples



## Where HashMaps Shine

```
┌─────────────────────────────────────────┐
│                                         │
│  📞 Phone Book                          │
│     Key: person's name                  │
│     Value: phone number                 │
│                                         │
│  🏪 Inventory System                    │
│     Key: product name                   │
│     Value: quantity in stock            │
│                                         │
│  📊 Word Counter                        │
│     Key: word                           │
│     Value: how many times it appears    │
│                                         │
│  👤 User Database                       │
│     Key: username                       │
│     Value: user data                    │
│                                         │
│  🎮 Game Scores                         │
│     Key: player name                    │
│     Value: score                        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# HashMap vs Vec



## Different Tools, Different Jobs

```
┌─────────────────────────────────────────┐
│                                         │
│  Vec<T>                                 │
│                                         │
│  • Ordered by index (0, 1, 2, 3...)     │
│  • Access by position: vec[0]           │
│  • Good for: lists, sequences           │
│  • Finding by value: slow (O(n))        │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  HashMap<K, V>                          │
│                                         │
│  • No inherent order                    │
│  • Access by key: map["apple"]          │
│  • Good for: lookups, associations      │
│  • Finding by key: fast (O(1))          │
│                                         │
└─────────────────────────────────────────┘
```



Use Vec when you care about ORDER.

Use HashMap when you care about LOOKUP.



--- slide ---

# Importing HashMap



## Not in the Prelude

Unlike Vec, HashMap must be imported:

```rust
use std::collections::HashMap;
```



```
┌─────────────────────────────────────────┐
│                                         │
│  std::collections contains:             │
│                                         │
│  • HashMap   - key-value pairs          │
│  • HashSet   - unique values only       │
│  • BTreeMap  - sorted key-value pairs   │
│  • BTreeSet  - sorted unique values     │
│  • VecDeque  - double-ended queue       │
│  • LinkedList - linked list             │
│                                         │
│  HashMap is the most commonly used.     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Creating a HashMap



## The new() Constructor

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
```



```
┌─────────────────────────────────────────┐
│                                         │
│  HashMap::new()                         │
│                                         │
│  Creates an EMPTY HashMap.              │
│                                         │
│  Note: You usually need `mut` because   │
│  you'll be inserting things into it.    │
│                                         │
│  The type is inferred from usage:       │
│  • What type of keys do you insert?     │
│  • What type of values do you insert?   │
│                                         │
│  Rust figures it out automatically!     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Type Annotation



## Being Explicit

Sometimes you want to specify the types:

```rust
let mut scores: HashMap<String, i32> = HashMap::new();
```



```
┌─────────────────────────────────────────┐
│                                         │
│  HashMap<K, V>                          │
│          ↑  ↑                           │
│          │  └─ Value type               │
│          └─ Key type                    │
│                                         │
│  HashMap<String, i32>                   │
│    Keys are Strings                     │
│    Values are i32 integers              │
│                                         │
│  HashMap<String, u32>                   │
│    Keys are Strings                     │
│    Values are u32 (unsigned)            │
│                                         │
│  The key type must implement:           │
│    Hash + Eq (for hashing to work)      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Inserting Values



## The insert() Method

```rust
let mut basket = HashMap::new();

basket.insert(key, value);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .insert(key, value)                    │
│          ↑     ↑                        │
│          │     └─ The value to store    │
│          └─ The key to identify it      │
│                                         │
│  After insert:                          │
│    basket[key] == value                 │
│                                         │
│  If the key already exists:             │
│    The OLD value is REPLACED            │
│    The new value takes over             │
│                                         │
│  insert() returns Option<V>:            │
│    Some(old_value) if key existed       │
│    None if key was new                  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Multiple Inserts



## Building Up Your HashMap

You can insert as many pairs as you need:

```rust
let mut basket = HashMap::new();

basket.insert(key1, value1);
basket.insert(key2, value2);
basket.insert(key3, value3);
// ... and so on
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Each insert adds a new key-value pair. │
│                                         │
│  Order of insertion doesn't matter      │
│  for HashMap functionality.             │
│                                         │
│  The HashMap grows automatically        │
│  as you add more items.                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Keys Must Be Unique



## One Value Per Key

A HashMap can only have ONE value per key:

```rust
let mut map = HashMap::new();

map.insert("apple", 5);
map.insert("apple", 10);  // Overwrites!

// map["apple"] is now 10, not 5
```



```
┌─────────────────────────────────────────┐
│                                         │
│  KEY UNIQUENESS RULE:                   │
│                                         │
│  • Each key can appear ONCE             │
│  • Inserting same key = REPLACEMENT     │
│  • No duplicate keys allowed            │
│                                         │
│  Different keys CAN have same value:    │
│                                         │
│  map.insert("apple", 5);                │
│  map.insert("orange", 5);  // OK!       │
│                                         │
│  Same value, different keys = fine.     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Reading Values



## The get() Method

```rust
let count = basket.get("apple");
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .get(&key) returns Option<&V>          │
│                                         │
│  • Some(&value) if key exists           │
│  • None if key doesn't exist            │
│                                         │
│  WHY Option?                            │
│                                         │
│  The key might not be in the map!       │
│  Rust makes you handle this possibility.│
│                                         │
│  Common patterns:                       │
│                                         │
│  if let Some(v) = map.get(&key) { }     │
│  map.get(&key).unwrap_or(&default)      │
│  match map.get(&key) { ... }            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# HashMap Length



## Counting Entries

```rust
let count = basket.len();
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .len() returns number of key-value     │
│  pairs in the HashMap.                  │
│                                         │
│  let mut map = HashMap::new();          │
│  map.len()  // 0                        │
│                                         │
│  map.insert("a", 1);                    │
│  map.len()  // 1                        │
│                                         │
│  map.insert("b", 2);                    │
│  map.len()  // 2                        │
│                                         │
│  map.insert("a", 99);  // Same key!     │
│  map.len()  // Still 2 (replacement)    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Summing Values



## Iterating Over Values

```rust
let total: u32 = basket.values().sum();
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .values() gives an iterator over       │
│  all VALUES in the HashMap.             │
│                                         │
│  HashMap { "a": 1, "b": 2, "c": 3 }     │
│  .values() → 1, 2, 3 (some order)       │
│                                         │
│  .sum() adds up all the values          │
│                                         │
│  Other useful iterators:                │
│                                         │
│  .keys()     → iterate over keys        │
│  .values()   → iterate over values      │
│  .iter()     → iterate over (key, value)│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Key Type Requirements



## What Can Be a Key?

Not everything can be a HashMap key:

```
┌─────────────────────────────────────────┐
│                                         │
│  VALID KEY TYPES:                       │
│                                         │
│  • String, &str                         │
│  • Integers (i32, u32, i64, etc.)       │
│  • bool                                 │
│  • char                                 │
│  • Tuples (if contents are hashable)    │
│  • Custom types with Hash + Eq derived  │
│                                         │
│  INVALID KEY TYPES:                     │
│                                         │
│  • f32, f64 (floating point)            │
│    (NaN != NaN breaks equality)         │
│  • Types without Hash implementation    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# String Keys



## Creating Owned Strings

When using String as keys, you need owned Strings:

```rust
let mut map = HashMap::new();

// This works - String::from creates owned String
map.insert(String::from("apple"), 5);

// String literals are &str, not String
// Type inference determines which you need
```



```
┌─────────────────────────────────────────┐
│                                         │
│  HashMap<String, u32>                   │
│          ↑                              │
│          Owned String keys              │
│                                         │
│  To create String from literal:         │
│                                         │
│  String::from("text")                   │
│  "text".to_string()                     │
│  "text".to_owned()                      │
│                                         │
│  All create an owned String.            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Building a Fruit Basket

You need to create a HashMap representing

a basket of fruits:

- Key: fruit name (String)
- Value: quantity (u32)



```
┌─────────────────────────────────────────┐
│                                         │
│  REQUIREMENTS:                          │
│                                         │
│  1. Declare a mutable HashMap           │
│                                         │
│  2. At least 3 different fruit types    │
│     (bananas already given to you)      │
│                                         │
│  3. Total fruit count >= 5              │
│     (2 bananas already there)           │
│                                         │
│  You need to:                           │
│  • Declare the HashMap variable         │
│  • Add more fruit types using insert()  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Declaring Variables



## The let Statement

When you see a TODO like:

```rust
// let mut basket =
```

You need to complete the declaration:

```
┌─────────────────────────────────────────┐
│                                         │
│  Variable declaration pattern:          │
│                                         │
│  let mut name = initial_value;          │
│      ↑↑↑                                │
│      Mutable because we'll insert       │
│                                         │
│  For a new empty HashMap:               │
│                                         │
│  What constructor creates an empty one? │
│  (Hint: you've seen it in this lesson)  │
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
│  1. What constructor creates a new      │
│     empty HashMap?                      │
│                                         │
│  2. The basket already has bananas.     │
│     How many MORE fruit types do you    │
│     need to add to reach at least 3?    │
│                                         │
│  3. There are 2 bananas already.        │
│     How many MORE fruits do you need    │
│     to reach at least 5 total?          │
│                                         │
│  4. What method adds a key-value pair   │
│     to a HashMap?                       │
│                                         │
│  5. What type should fruit names be?    │
│     (Look at the existing banana line)  │
│                                         │
└─────────────────────────────────────────┘
```



Look at the existing code for patterns to follow!

(Now go to the Editor and try it!)
