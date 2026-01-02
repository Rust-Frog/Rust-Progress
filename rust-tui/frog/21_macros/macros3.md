# Macros in Modules



## The Visibility Problem

Macros defined in a module aren't visible outside!

```rust
mod macros {
    macro_rules! my_macro {
        () => { println!("Hi"); };
    }
}

fn main() {
    my_macro!();  // Error: macro not found!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Module boundary blocks macro:          │
│                                         │
│  ┌─────────────────────┐                │
│  │  mod macros         │                │
│  │  ┌───────────────┐  │                │
│  │  │ my_macro!     │  │                │
│  │  └───────────────┘  │                │
│  └─────────────────────┘                │
│           ↑                             │
│           │ Can't see out!              │
│           ✗                             │
│  main() wants my_macro...               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# #[macro_use]



## Exporting Macros

The `#[macro_use]` attribute exports macros

from a module to its parent scope:

```rust
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => { println!("Hi"); };
    }
}

fn main() {
    my_macro!();  // Works now!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  #[macro_use] says:                     │
│  "Export all macros from this module    │
│   to the parent scope!"                 │
│                                         │
│  It goes BEFORE the mod keyword,        │
│  not inside the module.                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Where to Put #[macro_use]



## On the Module Declaration

```rust
#[macro_use]    // ← Attribute goes here
mod macros {    // ← Before mod
    macro_rules! my_macro {
        () => { println!("Hi"); };
    }
}
```

NOT inside the module:

```rust
mod macros {
    #[macro_use]  // ← WRONG! This won't work.
    macro_rules! my_macro { ... }
}
```



--- slide ---

# Your Exercise



## Macro Hidden in Module

```rust
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();  // Can't find it!
}
```

The macro is trapped in the module!



--- slide ---

# What You Need to Do



## Add #[macro_use]

```
┌─────────────────────────────────────────┐
│                                         │
│  Add #[macro_use] before the module:    │
│                                         │
│  #[macro_use]                           │
│  mod macros {                           │
│      macro_rules! my_macro {            │
│          () => {                        │
│              println!("...");           │
│          };                             │
│      }                                  │
│  }                                      │
│                                         │
│  DO NOT move the macro out of the       │
│  module - the exercise asks you to      │
│  keep it inside!                        │
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
│  1. Why can't main() see the macro?     │
│     (It's inside a module)              │
│                                         │
│  2. What attribute exports macros       │
│     from a module?                      │
│     (Hint: #[macro_???])                │
│                                         │
│  3. Where does this attribute go?       │
│     (On the line BEFORE mod macros)     │
│                                         │
│  4. Can you move the macro instead?     │
│     (The exercise says: don't!)         │
│                                         │
└─────────────────────────────────────────┘
```



#[macro_use] exports macros from modules!

(Go try it in the Editor!)
