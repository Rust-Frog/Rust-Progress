# Working With Strings



## Useful String Methods

Strings aren't just for storing text.

You need to MANIPULATE them:

- Remove extra whitespace
- Combine strings together
- Replace parts of strings



```
┌─────────────────────────────────────────┐
│                                         │
│  Rust provides methods for all of this! │
│                                         │
│  But there's a catch:                   │
│  Some methods return &str               │
│  Some methods return String             │
│                                         │
│  Knowing which is which matters!        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Trimming Whitespace



## The trim() Method

`.trim()` removes whitespace from both ends:

```rust
let messy = "  hello world  ";
let clean = messy.trim();
// clean is "hello world"
```



```
┌─────────────────────────────────────────┐
│                                         │
│  BEFORE:  "  hello world  "             │
│            ↑↑            ↑↑             │
│            spaces        spaces         │
│                                         │
│  AFTER:   "hello world"                 │
│           No leading/trailing spaces    │
│                                         │
│  Note: spaces INSIDE are kept!          │
│  Only edges are trimmed.                │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# trim() Returns &str



## No New Allocation!

Here's the important part:

```rust
let input: &str = "  hello  ";
let output: &str = input.trim();
```



```
┌─────────────────────────────────────────┐
│                                         │
│  trim() doesn't create a new String!    │
│                                         │
│  It returns a &str that points to       │
│  a SUBSET of the original data.         │
│                                         │
│  Original: "  hello  "                  │
│             ↑↑     ↑↑                   │
│  Trimmed:    "hello"                    │
│             └──────┘                    │
│             Just a view into original!  │
│                                         │
│  Same memory, different start/end.      │
│  Super efficient!                       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# trim() Variants



## More Precise Control

```rust
// Trim both ends (most common)
"  hello  ".trim()       // "hello"

// Trim only the start (left)
"  hello  ".trim_start() // "hello  "

// Trim only the end (right)
"  hello  ".trim_end()   // "  hello"
```



```
┌─────────────────────────────────────────┐
│                                         │
│  trim()       → both ends               │
│  trim_start() → left side only          │
│  trim_end()   → right side only         │
│                                         │
│  All return &str (no allocation).       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Combining Strings



## String Concatenation

Often you need to join strings together:

"Hello" + " world!" = "Hello world!"

Rust gives you several ways to do this.



```
┌─────────────────────────────────────────┐
│                                         │
│  METHODS TO COMBINE:                    │
│                                         │
│  1. format!() macro                     │
│  2. + operator                          │
│  3. push_str() method                   │
│  4. String + &str                       │
│                                         │
│  Let's look at each!                    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The format! Macro



## Easiest and Most Flexible

```rust
let hello = "Hello";
let result = format!("{} world!", hello);
// result is "Hello world!"
```



```
┌─────────────────────────────────────────┐
│                                         │
│  format!("{} world!", hello)            │
│          ↑↑                             │
│          Placeholder for hello          │
│                                         │
│  Works like println! but returns        │
│  a String instead of printing.          │
│                                         │
│  ALWAYS returns String (owned).         │
│  Creates new heap allocation.           │
│                                         │
│  Very flexible - combine anything!      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The + Operator



## Concatenation With +

```rust
let s1 = String::from("Hello");
let s2 = " world!";
let result = s1 + s2;  // s1 is MOVED here!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  IMPORTANT RULES FOR +:                 │
│                                         │
│  • Left side must be String (owned)     │
│  • Right side must be &str (borrowed)   │
│  • Left String is CONSUMED (moved)!     │
│  • Returns a new String                 │
│                                         │
│  After s1 + s2:                         │
│    s1 is no longer usable (moved)       │
│    s2 is still usable (only borrowed)   │
│    result owns the combined string      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# push_str() Method



## Append to Existing String

```rust
let mut s = String::from("Hello");
s.push_str(" world!");
// s is now "Hello world!"
```



```
┌─────────────────────────────────────────┐
│                                         │
│  push_str() modifies IN PLACE.          │
│                                         │
│  • String must be mutable (mut)         │
│  • Appends &str to the end              │
│  • No return value (modifies self)      │
│  • May reallocate if capacity exceeded  │
│                                         │
│  Good when building a string gradually: │
│                                         │
│  let mut result = String::new();        │
│  result.push_str("Hello");              │
│  result.push_str(" ");                  │
│  result.push_str("world!");             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Replacing Text



## The replace() Method

```rust
let original = "I like cats";
let new = original.replace("cats", "dogs");
// new is "I like dogs"
```



```
┌─────────────────────────────────────────┐
│                                         │
│  replace(from, to)                      │
│          ↑      ↑                       │
│         find   replace with             │
│                                         │
│  Finds ALL occurrences of `from`        │
│  and replaces them with `to`.           │
│                                         │
│  "cats cats cats".replace("cats", "x")  │
│  → "x x x"                              │
│                                         │
│  Returns a NEW String (owned).          │
│  Original is unchanged!                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# replace() Returns String



## New Allocation

Unlike `trim()`, `replace()` returns a `String`:

```rust
let original: &str = "hello";
let replaced: String = original.replace("l", "L");
// replaced is "heLLo"
```



```
┌─────────────────────────────────────────┐
│                                         │
│  WHY does replace() return String?      │
│                                         │
│  The result might be BIGGER or SMALLER  │
│  than the original!                     │
│                                         │
│  "cat".replace("cat", "elephant")       │
│  → "elephant" (much bigger!)            │
│                                         │
│  Can't just point to original data.     │
│  Must create new storage.               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Return Type Summary



## &str vs String

```
┌─────────────────────────────────────────┐
│                                         │
│  RETURNS &str (no allocation):          │
│                                         │
│    .trim()                              │
│    .trim_start()                        │
│    .trim_end()                          │
│                                         │
│  These just adjust the view window.     │
│  Point to subset of existing data.      │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  RETURNS String (allocates):            │
│                                         │
│    .replace()                           │
│    .to_lowercase()                      │
│    .to_uppercase()                      │
│    format!()                            │
│                                         │
│  These create NEW string content.       │
│  Need fresh heap allocation.            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Three Functions to Implement

You need to implement three functions:

```rust
fn trim_me(input: &str) -> &str {
    // Remove whitespace from both ends
}

fn compose_me(input: &str) -> String {
    // Add " world!" to the string
}

fn replace_me(input: &str) -> String {
    // Replace "cars" with "balloons"
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Notice the return types!               │
│                                         │
│  trim_me returns &str                   │
│    → trim() returns &str, perfect!      │
│                                         │
│  compose_me returns String              │
│    → Need to create new String          │
│                                         │
│  replace_me returns String              │
│    → replace() returns String, great!   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Function 1: trim_me



## Removing Whitespace

```rust
fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  TEST CASES:                            │
│                                         │
│  "Hello!     " → "Hello!"               │
│  "  What's up!" → "What's up!"          │
│  "   Hola!  " → "Hola!"                 │
│                                         │
│  What method removes whitespace         │
│  from BOTH ends?                        │
│                                         │
│  (Hint: It's 4 letters!)                │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Function 2: compose_me



## Adding to a String

```rust
fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  TEST CASES:                            │
│                                         │
│  "Hello" → "Hello world!"               │
│  "Goodbye" → "Goodbye world!"           │
│                                         │
│  Multiple approaches work:              │
│                                         │
│  • format!("{} world!", input)          │
│  • input.to_string() + " world!"        │
│  • String::from(input) + " world!"      │
│  • And more!                            │
│                                         │
│  Pick whatever feels natural.           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Function 3: replace_me



## Substituting Text

```rust
fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" with "balloons"
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  TEST CASES:                            │
│                                         │
│  "I think cars are cool"                │
│  → "I think balloons are cool"          │
│                                         │
│  "I love to look at cars"               │
│  → "I love to look at balloons"         │
│                                         │
│  What method replaces text?             │
│  What two arguments does it take?       │
│                                         │
│  (Hint: method_name(from, to))          │
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
│  FOR trim_me:                           │
│  1. What method trims whitespace?       │
│  2. Does it return the right type?      │
│                                         │
│  FOR compose_me:                        │
│  1. What needs to be added?             │
│  2. How do I combine strings?           │
│  3. What should the return type be?     │
│                                         │
│  FOR replace_me:                        │
│  1. What's being replaced?              │
│  2. What's the replacement?             │
│  3. What method does this?              │
│                                         │
│  Check the tests to verify expected     │
│  inputs and outputs!                    │
│                                         │
└─────────────────────────────────────────┘
```



Each function is just ONE line of code!

(Now go to the Editor and try it!)
