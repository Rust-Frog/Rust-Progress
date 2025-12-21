# Same Problem, Different Look

This is the same "empty box" problem, but the 
code looks slightly different:

```rust
let x: i32;
       ^^^
       This is a "type annotation"
```

## What's `i32`?

It's telling Rust what KIND of box this is:

```
┌─────────────┐
│     ???     │  ← Still empty!
└─────────────┘
  x (integers only)
```

You're saying: "Make a box for integers called x"

But you STILL didn't put anything IN it!
Knowing the box type doesn't fill the box.

--- slide ---

# Types Are Like Box Shapes

Different types = different shaped boxes:

```
INTEGER BOX:        STRING BOX:        BOOL BOX:
┌─────────────┐    ┌─────────────┐    ┌───────────┐
│     42      │    │   "hello"   │    │    true   │
└─────────────┘    └─────────────┘    └───────────┘
    i32                String             bool
```

You can ONLY put matching things in each box:

```
let age: i32 = 25;        // ✓ Number in number box
let name: i32 = "Bob";    // ✗ Text in number box!
```

Think of it like shapes:

```
○ goes in ○ hole    ✓
□ goes in ○ hole    ✗
```

--- slide ---

# Common Types

Here's a cheat sheet of common types:

```
┌─────────────┬──────────────────────────────────┐
│ Type        │ What it holds                    │
├─────────────┼──────────────────────────────────┤
│ i32         │ Whole numbers (can be negative)  │
│ u32         │ Whole numbers (positive only)    │
│ f64         │ Decimal numbers (3.14, 2.5)      │
│ bool        │ true or false                    │
│ char        │ Single character ('a', 'Z')      │
│ &str        │ Text ("hello", "world")          │
│ String      │ Text you can modify              │
└─────────────┴──────────────────────────────────┘
```

The most common is `i32` for numbers.
Rust usually picks this automatically.

--- slide ---

# The Error Explained

The code says:

```rust
let x: i32;      // Make an integer box called x
println!("{x}"); // Print what's in x
```

But the box is EMPTY!

```
┌─────────────┐
│     ???     │  ← Nothing here to print!
└─────────────┘
  x: i32

Rust: "You want me to print... what exactly?"
```

Knowing the box is for integers doesn't help
if there's no integer inside!

--- slide ---

# The Fix

Put a number in when you create the box:

```rust
let x: i32 = 5;
             ^
             Now there's something inside!
```

Visualized:

```
┌─────────────┐
│      5      │  ← Has a value!
└─────────────┘
  x: i32
```

Or even simpler - let Rust figure out the type:

```rust
let x = 5;
```

Rust sees `5` and thinks:
"That's a number, so x must be an i32 box."

Both work! The second is just less typing.
