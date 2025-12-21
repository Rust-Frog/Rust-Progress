# The Super Glue Problem

Remember: Rust variables have super glue by default.

```
┌─────────────┐
│      3      │  ← GLUED IN
└─────────────┘
       x
```

The code tries to do this:

```rust
let x = 3;     // Put 3 in the box (glued!)
println!("Number {x}");

x = 5;         // Try to swap it for 5
println!("Number {x}");
```

Rust says: "Nope. That 3 is glued in forever."

```
error: cannot assign twice to immutable variable
```

--- slide ---

# Why The Super Glue?

Imagine you're building a house with 50 people:

```
You set room_count = 10
Someone else changes it to 0
Later you calculate: 10 rooms × $1000 = ???

CRASH! You expected 10, got 0!
```

This bug is EVIL because:
- It works sometimes
- Fails randomly
- Hard to reproduce
- Makes you question your sanity

Rust's solution:

```
┌────────────────────────────────────────────────┐
│  "Want to change something?                    │
│   Say so UP FRONT with 'mut'."                 │
│                                                │
│  No surprises. No secret modifications.        │
│  If you don't say 'mut', it doesn't change.    │
└────────────────────────────────────────────────┘
```

--- slide ---

# Removing the Super Glue

Use `mut` (short for "mutable" = changeable):

```rust
let mut x = 3;
    ^^^
    "I might change this later"
```

Now you can swap values:

```
BEFORE:               AFTER x = 5:
┌─────────────┐       ┌─────────────┐
│      3      │   →   │      5      │
└─────────────┘       └─────────────┘
   x (changeable)        x (new value!)
```

The full working code:

```rust
let mut x = 3;
println!("Number {x}");  // Prints 3

x = 5;
println!("Number {x}");  // Prints 5
```

--- slide ---

# The Fix

Original code (broken):
```rust
let x = 3;     // Glued!
x = 5;         // Can't change!
```

Fixed code:
```rust
let mut x = 3; // Not glued!
x = 5;         // Now this works!
    ^^^
    Add mut here
```

Just add those 3 letters: `m-u-t`

The comment in the code says "Don't change this line"
for `x = 5;`, so you need to fix the FIRST line,
not the second one.

--- slide ---

# When to Use `mut`

```
┌────────────────────────────────────────────────┐
│  USE mut WHEN:                                 │
├────────────────────────────────────────────────┤
│  • Counting things (counter += 1)              │
│  • Building up a total (sum += value)          │
│  • Loop variables that update                  │
│  • The value genuinely needs to change         │
└────────────────────────────────────────────────┘

┌────────────────────────────────────────────────┐
│  DON'T USE mut WHEN:                           │
├────────────────────────────────────────────────┤
│  • The value should stay the same              │
│  • You're just reading data                    │
│  • You're not sure (default to no mut)         │
└────────────────────────────────────────────────┘
```

Rule of thumb: Start WITHOUT mut.
Add it only when the compiler complains.
