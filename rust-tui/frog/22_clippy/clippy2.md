# Clippy: Iterating Over Option



## The Pattern

Some code iterates over an Option:

```rust
let option = Some(12);
for x in option {
    res += x;
}
```

This WORKS, but Clippy says there's a better way!



```
┌─────────────────────────────────────────┐
│                                         │
│  Option implements IntoIterator:        │
│                                         │
│  Some(x) → yields x once                │
│  None    → yields nothing               │
│                                         │
│  So `for x in option` is valid...       │
│  but it's a weird pattern!              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Clippy Complains



## Misleading Code

A `for` loop suggests multiple iterations.

But Option has at most ONE value!

```rust
// This LOOKS like it might loop many times:
for x in option {
    res += x;
}

// But it runs 0 or 1 times - confusing!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  for loop = "do this for EACH item"     │
│                                         │
│  But Option has 0 or 1 items only!      │
│  This misleads readers of your code.    │
│                                         │
│  Clippy: "Use if let instead!"          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# if let: The Better Pattern



## Clear Intent

`if let` makes it obvious you're handling ONE value:

```rust
// Before (misleading):
for x in option {
    res += x;
}

// After (clear):
if let Some(x) = option {
    res += x;
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  if let Some(x) = option {              │
│      // runs if option is Some          │
│      // x holds the inner value         │
│  }                                      │
│                                         │
│  This clearly says:                     │
│  "If there's a value, use it"           │
│                                         │
│  NOT: "Loop over multiple things"       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Replace for with if let

```rust
fn main() {
    let mut res = 42;
    let option = Some(12);

    // TODO: Fix the Clippy lint
    for x in option {
        res += x;
    }

    println!("{res}");
}
```



--- slide ---

# What You Need to Do



## Rewrite the Loop

```
┌─────────────────────────────────────────┐
│                                         │
│  Change this:                           │
│                                         │
│  for x in option {                      │
│      res += x;                          │
│  }                                      │
│                                         │
│  To this:                               │
│                                         │
│  if let Some(x) = option {              │
│      res += x;                          │
│  }                                      │
│                                         │
│  Same behavior, clearer intent!         │
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
│  1. How many times can a for loop over  │
│     Option run?                         │
│     (0 or 1)                            │
│                                         │
│  2. What pattern handles "maybe one     │
│     value" more clearly?                │
│     (if let Some(x) = ...)              │
│                                         │
│  3. What goes inside the if let body?   │
│     (The same code: res += x)           │
│                                         │
└─────────────────────────────────────────┘
```



if let is clearer than for on Option!

(Go try it in the Editor!)
