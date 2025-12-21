# Shadowing: New Box, Same Name

Shadowing is like throwing away the old box 
and making a NEW one with the same label.

```
First: let x = "hello";
┌─────────────┐
│   "hello"   │
└─────────────┘
       x

Then: let x = 5;  (note the second 'let')
┌─────────────┐    ┌─────────────┐
│   "hello"   │    │      5      │
└─────────────┘    └─────────────┘
   (forgotten)            x
```

The old box still exists somewhere, but you 
can't see it anymore. The new `x` "shadows" it.

--- slide ---

# Shadowing vs Mutability

These look similar but are COMPLETELY different:

## Mutability: Same box, different value
```rust
let mut x = 5;
x = 6;           // Same box, swap contents
```

## Shadowing: Brand new box
```rust
let x = 5;
let x = 6;       // THROW AWAY old box, make new!
^^^
Notice the 'let' again!
```

The key difference:

```
┌────────────────────────────────────────────────┐
│  MUTABILITY:                                   │
│  "Change what's IN the box"                    │
│  Box type stays the same                       │
│                                                │
│  SHADOWING:                                    │
│  "Replace the entire box"                      │
│  New box can be different type!                │
└────────────────────────────────────────────────┘
```

--- slide ---

# The Type-Change Superpower

Shadowing can change types. Mutability cannot.

```rust
// SHADOWING - This works!
let x = "three";  // x is text
let x = 3;        // x is now a number!
```

```rust
// MUTABILITY - This breaks!
let mut x = "three";  // x is text
x = 3;                // ERROR! Still a text box!
```

Why?

```
Shadowing: Two completely different boxes
┌───────────┐    ┌───────────┐
│  "three"  │    │     3     │
└───────────┘    └───────────┘
 (discarded)        new x

Mutability: One box, one type
┌───────────┐
│  "three"  │  ← This box is for TEXT only!
└───────────┘
     x           Can't put a number in TEXT box!
```

--- slide ---

# Your Exercise

The code tries to do:

```rust
let number = "T-H-R-E-E";  // Text box
number = 3;                 // Try to put number in
```

This is like trying to shove a basketball 
into a shoebox. Wrong shape!

```
┌───────────────┐
│  "T-H-R-E-E"  │  ← This is a TEXT box
└───────────────┘
     number

     3 ← "I'm a number, I don't fit in text box!"
```

Also, there's no `mut`, so you can't reassign anyway!

--- slide ---

# The Fix

You need a NEW box. Use `let` again:

```rust
let number = "T-H-R-E-E";  // Text box (old)
let number = 3;            // Number box (new!)
^^^
Add 'let' to make it shadowing!
```

Now you're not stuffing a number into a text box.
You're making a BRAND NEW box that holds numbers:

```
OLD (shadowed):      NEW (active):
┌───────────────┐    ┌─────────────┐
│  "T-H-R-E-E"  │    │      3      │
└───────────────┘    └─────────────┘
   (forgotten)          number
```

The old text box is forgotten.
The new number box is what `number` means now.
