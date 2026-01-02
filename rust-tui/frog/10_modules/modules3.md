# The Standard Library



## Rust's Built-in Toolkit

Rust comes with a powerful **standard library** (`std`)

full of useful modules, types, and functions.



```
┌─────────────────────────────────────────┐
│                                         │
│  std contains:                          │
│                                         │
│  std::collections  → HashMap, Vec, etc  │
│  std::io           → Input/Output       │
│  std::fs           → File system        │
│  std::time         → Time utilities     │
│  std::thread       → Threading          │
│  std::net          → Networking         │
│  ... and much more!                     │
│                                         │
│  You've already used some:              │
│    String (from std::string)            │
│    Vec (from std::vec)                  │
│    println! (from std::io)              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Importing from std



## Using use with the Standard Library

To use items from `std`, use the `use` keyword:

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key", "value");
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  WITHOUT use:                           │
│                                         │
│  std::collections::HashMap::new()       │
│  (Very long!)                           │
│                                         │
│  WITH use:                              │
│                                         │
│  use std::collections::HashMap;         │
│  HashMap::new()                         │
│  (Much cleaner!)                        │
│                                         │
└─────────────────────────────────────────┘

-
```



--- slide ---

# The Prelude



## Automatically Imported

Some things are used SO often that Rust

automatically imports them. This is the **prelude**.

```rust
// These are available without `use`:
String      // std::string::String
Vec         // std::vec::Vec
Option      // std::option::Option
Result      // std::result::Result
Some, None  // Option variants
Ok, Err     // Result variants
println!    // std::io macro
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The prelude is why you can write:      │
│                                         │
│  String::from("hello")                  │
│                                         │
│  Instead of:                            │
│                                         │
│  std::string::String::from("hello")     │
│                                         │
│  Rust imports common items for you!     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# What's NOT in the Prelude



## Must Import Manually

Many useful things require explicit `use`:

```rust
// Time utilities
use std::time::{SystemTime, Duration};

// Collections
use std::collections::{HashMap, HashSet};

// I/O
use std::io::{Read, Write, BufReader};

// File system
use std::fs::File;
```



```
┌─────────────────────────────────────────┐
│                                         │
│  If the compiler says:                  │
│  "cannot find type `X` in this scope"   │
│                                         │
│  You probably need to `use` it!         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Multiple Imports



## One use Per Item

You can write separate use statements:

```rust
use std::time::SystemTime;
use std::time::Duration;
use std::time::UNIX_EPOCH;
```



```
┌─────────────────────────────────────────┐
│                                         │
│  This works, but it's repetitive!       │
│                                         │
│  std::time::                            │
│  std::time::                            │
│  std::time::                            │
│                                         │
│  We're repeating the path prefix        │
│  three times. Can we do better?         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Nested Use Syntax



## Grouping Imports

Use curly braces to import multiple items at once:

```rust
use std::time::{SystemTime, Duration, UNIX_EPOCH};
```



```
┌─────────────────────────────────────────┐
│                                         │
│  use path::{Item1, Item2, Item3};       │
│            ↑                 ↑          │
│            Curly braces with comma-     │
│            separated list of items      │
│                                         │
│  This imports all three in ONE line!    │
│                                         │
│  BEFORE (3 lines):                      │
│    use std::time::SystemTime;           │
│    use std::time::Duration;             │
│    use std::time::UNIX_EPOCH;           │
│                                         │
│  AFTER (1 line):                        │
│    use std::time::{SystemTime, Duration, UNIX_EPOCH};│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Real Example: Time



## The std::time Module

```rust
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Get current time
    let now = SystemTime::now();

    // Calculate seconds since Unix epoch (Jan 1, 1970)
    let duration = now.duration_since(UNIX_EPOCH).unwrap();

    println!("Seconds since 1970: {}", duration.as_secs());
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  SystemTime = A point in time           │
│               (current system time)     │
│                                         │
│  UNIX_EPOCH = A constant representing   │
│               Jan 1, 1970 00:00:00 UTC  │
│               (the "zero" of Unix time) │
│                                         │
│  duration_since() = How much time       │
│                     between two points  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Understanding the Types



## SystemTime and UNIX_EPOCH

```
┌─────────────────────────────────────────┐
│                                         │
│  UNIX_EPOCH                             │
│  ↓                                      │
│  |←──── duration_since() ────→|         │
│  |                             |        │
│  Jan 1, 1970              SystemTime::now()│
│  00:00:00 UTC             (current time)│
│                                         │
│  The result is a Duration - how many    │
│  seconds (and nanoseconds) between them.│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Import Styles



## Different Ways to Import

```rust
// Style 1: Full path each time (no use)
std::time::SystemTime::now()
std::time::UNIX_EPOCH

// Style 2: Separate use statements
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

// Style 3: Nested use (recommended for multiple items)
use std::time::{SystemTime, UNIX_EPOCH};

// Style 4: Wildcard (imports everything public)
use std::time::*;  // Generally avoid this
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Style 3 (nested) is best practice      │
│  when importing multiple items from     │
│  the same module.                       │
│                                         │
│  Clean, explicit, one line.             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Wildcard Imports



## The * Syntax (Use Sparingly)

```rust
use std::collections::*;  // Import EVERYTHING
```



```
┌─────────────────────────────────────────┐
│                                         │
│  WHY AVOID *:                           │
│                                         │
│  • Unclear what's imported              │
│  • Name collisions possible             │
│  • Harder to trace where things come from│
│  • Pollutes your namespace              │
│                                         │
│  WHEN * IS OK:                          │
│                                         │
│  • Tests (use super::*)                 │
│  • Prelude modules designed for it      │
│  • Quick prototyping                    │
│                                         │
│  In general: be explicit!               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Importing from std::time

The code uses `SystemTime` and `UNIX_EPOCH`

but doesn't import them:

```rust
// TODO: Bring `SystemTime` and `UNIX_EPOCH` into scope

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("... {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  THE ERROR:                             │
│                                         │
│  "cannot find type `SystemTime`"        │
│  "cannot find value `UNIX_EPOCH`"       │
│                                         │
│  They exist in std::time - you just     │
│  need to import them!                   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Solution Path



## What to Import

```
┌─────────────────────────────────────────┐
│                                         │
│  You need:                              │
│                                         │
│  1. SystemTime (a struct)               │
│     → std::time::SystemTime             │
│                                         │
│  2. UNIX_EPOCH (a constant)             │
│     → std::time::UNIX_EPOCH             │
│                                         │
│  Both are in std::time!                 │
│                                         │
│  You can write two separate use         │
│  statements, or combine them with { }   │
│                                         │
│  BONUS: The exercise mentions "style    │
│  points" for doing it in ONE line!      │
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
│  1. What module are SystemTime and      │
│     UNIX_EPOCH in?                      │
│     (Hint: std::???)                    │
│                                         │
│  2. How do you import something         │
│     from the standard library?          │
│     (use std::...)                      │
│                                         │
│  3. How do you import MULTIPLE items    │
│     from the same module in ONE line?   │
│     (Hint: curly braces!)               │
│                                         │
│  4. What's the syntax?                  │
│     use path::{Item1, Item2};           │
│                                         │
└─────────────────────────────────────────┘
```



One line, two items, curly braces!

(Now go to the Editor and try it!)
