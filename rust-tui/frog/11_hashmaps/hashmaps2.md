# The Conditional Insert Problem



## When insert() Is Too Aggressive

Remember: `insert()` ALWAYS overwrites:

```rust
map.insert("apple", 5);
map.insert("apple", 10);  // Overwrites! 5 is gone.
```



```
┌─────────────────────────────────────────┐
│                                         │
│  But what if you want to:               │
│                                         │
│  "Add this key ONLY IF it's not         │
│   already in the map"                   │
│                                         │
│  Examples:                              │
│                                         │
│  • Set a default, don't override        │
│  • First player to register wins        │
│  • Don't replace existing data          │
│                                         │
│  insert() would destroy existing data!  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Manual Approach



## Check, Then Insert

You COULD check first:

```rust
if !map.contains_key(&key) {
    map.insert(key, value);
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  This works, but has problems:          │
│                                         │
│  1. TWO lookups (contains + insert)     │
│     Inefficient!                        │
│                                         │
│  2. Verbose - lots of boilerplate       │
│                                         │
│  3. Race conditions in concurrent code  │
│     (not an issue in single-threaded)   │
│                                         │
│  There must be a better way...          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Entry API



## Rust's Elegant Solution

Rust provides the **Entry API** for this pattern:

```rust
map.entry(key)
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .entry(key) returns an Entry enum:     │
│                                         │
│  Entry::Occupied(entry)                 │
│    → Key exists, entry has the value    │
│                                         │
│  Entry::Vacant(entry)                   │
│    → Key doesn't exist, ready to insert │
│                                         │
│  ONE lookup, then decide what to do!    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Understanding Entry



## A Slot in the HashMap

Think of `entry()` as finding the SLOT

where a key would live:

```
┌─────────────────────────────────────────┐
│                                         │
│  HashMap:                               │
│  ┌────────┬─────────┐                   │
│  │ "apple"│    5    │  ← Occupied       │
│  ├────────┼─────────┤                   │
│  │ "pear" │    3    │  ← Occupied       │
│  ├────────┼─────────┤                   │
│  │   ???  │   ???   │  ← Vacant (empty) │
│  └────────┴─────────┘                   │
│                                         │
│  map.entry("apple")  → Occupied         │
│  map.entry("banana") → Vacant           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# or_insert()



## Insert Only If Vacant

The most common Entry pattern:

```rust
map.entry(key).or_insert(value);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .or_insert(value)                      │
│                                         │
│  "If the key is NOT in the map,         │
│   insert this value.                    │
│   Otherwise, do nothing."               │
│                                         │
│  Returns: &mut V (mutable ref to value) │
│                                         │
│  If Occupied → returns ref to existing  │
│  If Vacant   → inserts value, returns ref│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# or_insert() Examples



## See It In Action

```rust
let mut map = HashMap::new();
map.insert("apple", 5);

// Key exists - nothing changes
map.entry("apple").or_insert(100);
// map["apple"] is still 5

// Key doesn't exist - inserts
map.entry("banana").or_insert(3);
// map["banana"] is now 3
```



```
┌─────────────────────────────────────────┐
│                                         │
│  EXISTING KEY:                          │
│    or_insert() → does nothing           │
│    existing value preserved             │
│                                         │
│  NEW KEY:                               │
│    or_insert() → inserts the value      │
│    key-value pair added to map          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Comparison: insert vs or_insert



## Different Behaviors

```
┌─────────────────────────────────────────┐
│                                         │
│  map.insert(key, value)                 │
│                                         │
│  • ALWAYS puts value in map             │
│  • Overwrites if key exists             │
│  • Returns Option<V> (old value)        │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  map.entry(key).or_insert(value)        │
│                                         │
│  • ONLY inserts if key is absent        │
│  • Preserves existing values            │
│  • Returns &mut V (ref to value)        │
│                                         │
└─────────────────────────────────────────┘
```



Choose based on whether you want to

preserve or replace existing values!



--- slide ---

# or_insert_with()



## Lazy Value Creation

What if creating the value is expensive?

```rust
map.entry(key).or_insert_with(|| expensive_computation());
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .or_insert_with(|| value_creator)      │
│                  ↑↑                     │
│                  Closure (lazy)         │
│                                         │
│  The closure is ONLY called if the      │
│  key doesn't exist!                     │
│                                         │
│  or_insert(expensive())                 │
│    → expensive() ALWAYS runs            │
│                                         │
│  or_insert_with(|| expensive())         │
│    → expensive() only runs if needed    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# or_default()



## Insert Default Value

If the value type has a Default:

```rust
map.entry(key).or_default();
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .or_default()                          │
│                                         │
│  Inserts the DEFAULT value for the type │
│  if the key doesn't exist.              │
│                                         │
│  Common defaults:                       │
│                                         │
│  • Numbers: 0                           │
│  • String: "" (empty)                   │
│  • Vec: [] (empty)                      │
│  • bool: false                          │
│                                         │
│  Type must implement Default trait.     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Checking If Key Exists



## Before Entry API

You can check if a key exists:

```rust
map.contains_key(&key)  // returns bool
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .contains_key(&key)                    │
│                                         │
│  Returns true if key is in the map.     │
│  Returns false if key is not in map.    │
│                                         │
│  Useful for logic:                      │
│                                         │
│  if map.contains_key(&"apple") {        │
│      println!("We have apples!");       │
│  }                                       │
│                                         │
│  But for conditional insert,            │
│  prefer the Entry API!                  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Pattern For Loops



## Adding Multiple Items Conditionally

A common pattern with Entry API:

```rust
for item in items {
    map.entry(item).or_insert(some_value);
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  For each item:                         │
│                                         │
│  1. Check if it's already in the map    │
│  2. If not, add it with the value       │
│  3. If yes, leave it alone              │
│                                         │
│  This ensures:                          │
│  • All items end up in the map          │
│  • Existing items aren't modified       │
│  • New items get the specified value    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Enum Keys



## Using Custom Types as Keys

Enums can be HashMap keys if they derive

the right traits:

```rust
#[derive(Hash, PartialEq, Eq)]
enum Fruit {
    Apple,
    Banana,
    Orange,
}

let mut basket = HashMap::new();
basket.insert(Fruit::Apple, 5);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  #[derive(Hash, PartialEq, Eq)]         │
│                                         │
│  These three traits are required:       │
│                                         │
│  • Hash - for computing hash values     │
│  • PartialEq - for comparing keys       │
│  • Eq - for full equality               │
│                                         │
│  derive() auto-implements them!         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Conditional Fruit Insertion

You have a basket that already contains

some fruits (Apple, Mango, Lychee).

You must add NEW fruits without changing

the existing ones.



```
┌─────────────────────────────────────────┐
│                                         │
│  RULES:                                 │
│                                         │
│  • Apple, Mango, Lychee are PRESET      │
│    DO NOT modify their quantities!      │
│                                         │
│  • You must add Banana and Pineapple    │
│                                         │
│  • Total fruits must be > 11            │
│                                         │
│  • All 5 fruit types must be present    │
│                                         │
│  How do you INSERT without OVERWRITING? │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Challenge



## Insert Only New Items

```rust
for fruit in fruit_kinds {
    // TODO: Insert new fruits if they are not
    // already present in the basket.
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The loop gives you each fruit kind.    │
│                                         │
│  Some fruits (Apple, Mango, Lychee)     │
│  are ALREADY in the basket.             │
│                                         │
│  Some fruits (Banana, Pineapple)        │
│  are NOT yet in the basket.             │
│                                         │
│  You need to:                           │
│  • Check if the fruit is already there  │
│  • If NOT there, add it with a quantity │
│  • If already there, leave it alone     │
│                                         │
│  Sound familiar? Entry API!             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Counting Requirements



## Do The Math

```
┌─────────────────────────────────────────┐
│                                         │
│  EXISTING FRUITS:                       │
│                                         │
│  Apple:  4                              │
│  Mango:  2                              │
│  Lychee: 5                              │
│  ─────────                              │
│  Total: 11                              │
│                                         │
│  REQUIREMENT: Total must be > 11        │
│                                         │
│  You need to add at least 1 more fruit  │
│  through Banana and/or Pineapple.       │
│                                         │
│  The existing 11 are protected -        │
│  your code must NOT change them.        │
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
│  1. What method lets you insert ONLY    │
│     if the key doesn't exist?           │
│                                         │
│  2. What does map.entry(key) return?    │
│                                         │
│  3. What method on Entry inserts only   │
│     if the slot is vacant?              │
│                                         │
│  4. The loop variable is `fruit`.       │
│     How do you get the entry for it?    │
│                                         │
│  5. What value should new fruits have?  │
│     (Must make total > 11)              │
│                                         │
│  6. Why can't you just use insert()?    │
│     (What would happen to existing?)    │
│                                         │
└─────────────────────────────────────────┘
```



The Entry API is the key to this exercise!

(Now go to the Editor and try it!)
