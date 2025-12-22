# The use Keyword



## Paths Get Long

As modules nest deeper, paths get tedious:

```rust
mod outer {
    mod inner {
        mod deep {
            pub fn my_function() { }
        }
    }
}

fn main() {
    outer::inner::deep::my_function();
    outer::inner::deep::my_function();  // Again!
    outer::inner::deep::my_function();  // And again!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Writing the full path every time is:   │
│                                         │
│  • Tedious to type                      │
│  • Hard to read                         │
│  • Repetitive                           │
│                                         │
│  There must be a better way!            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Bringing Paths Into Scope



## The use Statement

`use` brings a path into the current scope:

```rust
use outer::inner::deep::my_function;

fn main() {
    my_function();  // No path needed!
    my_function();  // Clean!
    my_function();  // Simple!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  use path::to::item;                    │
│                                         │
│  After this line, you can use `item`    │
│  directly without the full path.        │
│                                         │
│  It's like creating a shortcut.         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# How use Works



## Creating a Local Binding

`use` doesn't copy or move anything.

It creates a NAME in the current scope

that refers to the item at that path.

```rust
mod snacks {
    pub const APPLE: &str = "Apple";
}

use snacks::APPLE;

fn main() {
    println!("{}", APPLE);        // Works!
    println!("{}", snacks::APPLE); // Also works!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Both names refer to the same thing!    │
│  use just gives you a shorter alias.    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Renaming with as



## Custom Names

Sometimes you want a DIFFERENT name.

Use `as` to rename:

```rust
use snacks::APPLE as fruit;

fn main() {
    println!("{}", fruit);  // Prints "Apple"
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  use path::to::item as new_name;        │
│                        ↑↑               │
│                        Your chosen name │
│                                         │
│  Now `new_name` refers to `item`.       │
│  The original name still works too!     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Rename?



## Common Reasons

```
┌─────────────────────────────────────────┐
│                                         │
│  1. AVOID CONFLICTS                     │
│                                         │
│  use std::fmt::Result as FmtResult;     │
│  use std::io::Result as IoResult;       │
│                                         │
│  Both modules have `Result`!            │
│  Renaming avoids ambiguity.             │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  2. SIMPLER NAMES                       │
│                                         │
│  use very_long_module_name::Thing as T; │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  3. CLEARER CONTEXT                     │
│                                         │
│  use snacks::fruits::PEAR as fruit;     │
│                                         │
│  "fruit" might be clearer in context    │
│  than "PEAR".                           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Nested Modules



## Modules Inside Modules

Modules can contain other modules:

```rust
mod delicious_snacks {
    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  delicious_snacks                       │
│  ├── fruits                             │
│  │   ├── PEAR                           │
│  │   └── APPLE                          │
│  └── veggies                            │
│      ├── CUCUMBER                       │
│      └── CARROT                         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The self Keyword



## Relative Paths

Inside a module, `self` refers to the current module:

```rust
mod delicious_snacks {
    // Inside delicious_snacks:
    use self::fruits::PEAR;
    //  ↑↑↑↑
    //  "this module" (delicious_snacks)

    mod fruits {
        pub const PEAR: &str = "Pear";
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  self::fruits::PEAR                     │
│  ↑                                      │
│  "starting from THIS module"            │
│                                         │
│  Without self:                          │
│    fruits::PEAR                         │
│  Also works! (relative path)            │
│                                         │
│  self makes it explicit.                │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Re-exporting with pub use



## Making Shortcuts Public

`use` creates a private shortcut by default.

`pub use` makes it PUBLIC - others can use it!

```rust
mod delicious_snacks {
    pub use self::fruits::PEAR as fruit;
    //↑↑↑                       ↑↑
    //public!                   renamed!

    mod fruits {
        pub const PEAR: &str = "Pear";
    }
}

fn main() {
    // Can access through the shortcut!
    println!("{}", delicious_snacks::fruit);
}
```



--- slide ---

# pub use Explained



## The Magic of Re-exporting

```
┌─────────────────────────────────────────┐
│                                         │
│  WITHOUT pub use:                       │
│                                         │
│  mod snacks {                           │
│      use self::fruits::PEAR as fruit;   │
│      mod fruits { pub const PEAR... }   │
│  }                                      │
│                                         │
│  // From outside:                       │
│  snacks::fruit;  // ERROR! Private!     │
│  snacks::fruits::PEAR;  // ERROR! Private│
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  WITH pub use:                          │
│                                         │
│  mod snacks {                           │
│      pub use self::fruits::PEAR as fruit;│
│      mod fruits { pub const PEAR... }   │
│  }                                      │
│                                         │
│  // From outside:                       │
│  snacks::fruit;  // Works! Re-exported! │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Re-export?



## Cleaner Public APIs

Re-exporting lets you create a nice API:

```
┌─────────────────────────────────────────┐
│                                         │
│  INTERNAL STRUCTURE:                    │
│                                         │
│  my_crate                               │
│  ├── internal                           │
│  │   └── deeply                         │
│  │       └── nested                     │
│  │           └── ImportantThing         │
│                                         │
│  WITH RE-EXPORT:                        │
│                                         │
│  pub use internal::deeply::nested::ImportantThing;│
│                                         │
│  USERS SEE:                             │
│                                         │
│  my_crate::ImportantThing               │
│                                         │
│  Clean! They don't care about internals.│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Re-exporting with Aliases

Look at the code structure:

```rust
mod delicious_snacks {
    // TODO: Add use statements here

    mod fruits {
        pub const PEAR: &str = "Pear";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
    }
}

fn main() {
    println!("{}", delicious_snacks::fruit);  // Wants "fruit"
    println!("{}", delicious_snacks::veggie); // Wants "veggie"
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  main() wants to access:                │
│    delicious_snacks::fruit              │
│    delicious_snacks::veggie             │
│                                         │
│  But fruits/veggies are in sub-modules! │
│  You need to RE-EXPORT them with new    │
│  names at the delicious_snacks level.   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Breaking Down the Task



## What You Need to Do

```
┌─────────────────────────────────────────┐
│                                         │
│  STEP 1: Access the nested item         │
│                                         │
│  self::fruits::PEAR                     │
│  self::veggies::CUCUMBER                │
│                                         │
│  STEP 2: Rename with `as`               │
│                                         │
│  PEAR as fruit                          │
│  CUCUMBER as veggie                     │
│                                         │
│  STEP 3: Make it public with `pub use`  │
│                                         │
│  pub use self::fruits::PEAR as fruit;   │
│  pub use self::veggies::CUCUMBER as veggie;│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Pattern



## pub use ... as ...

```rust
pub use self::source_module::ITEM as new_name;
↑↑↑ ↑↑↑ ↑↑↑↑ ↑↑↑↑↑↑↑↑↑↑↑↑↑↑ ↑↑↑↑ ↑↑ ↑↑↑↑↑↑↑↑
 │   │    │        │          │   │     │
 │   │    │        │          │   │     └─ The new public name
 │   │    │        │          │   └─ Rename keyword
 │   │    │        │          └─ The item to export
 │   │    │        └─ The child module
 │   │    └─ "this module"
 │   └─ Bring into scope
 └─ Make it public
```



--- slide ---

# Think It Through



## Before You Type



```
┌─────────────────────────────────────────┐
│  ASK YOURSELF:                          │
│                                         │
│  1. What does main() try to access?     │
│     (delicious_snacks::fruit and ?)     │
│                                         │
│  2. Where does PEAR actually live?      │
│     (In which sub-module?)              │
│                                         │
│  3. What keyword brings a path into     │
│     scope? (Hint: 3 letters)            │
│                                         │
│  4. What keyword renames something?     │
│     (Hint: 2 letters)                   │
│                                         │
│  5. What keyword makes it public?       │
│     (Hint: 3 letters, goes first)       │
│                                         │
│  6. How do you refer to "this module"?  │
│     (Hint: 4 letters)                   │
│                                         │
└─────────────────────────────────────────┘
```



You need TWO pub use statements!

(Now go to the Editor and try it!)
