# Variables Need Values

Imagine declaring a box but leaving it empty:

```
┌─────────────┐
│     ???     │  ← Nothing inside!
└─────────────┘
       x
```

What happens when someone asks "what's in x?"

```
You: "Uh... I don't know."
Rust: "That's a problem. I refuse to run."
```

This isn't Rust being picky. This prevents 
REAL bugs that crash REAL programs.

--- slide ---

# The Garbage Problem

In older languages like C and C++, if you 
forget to put something in a box, it has 
whatever GARBAGE was left in memory:

```
┌─────────────┐
│  -4829173   │  ← Random leftover junk!
└─────────────┘
       x
```

This could be ANYTHING:
- Part of an old password
- Pieces of other programs
- Random bits that make no sense

This causes:
- Security vulnerabilities
- Crashes that happen "sometimes"
- Bugs that are impossible to reproduce

Rust says: **Nope. Not allowed. Ever.**

--- slide ---

# Rust's Rule

Every box MUST have something in it before 
you try to look inside.

```rust
let x;           // Box exists, but empty
println!("{x}"); // "What's in x?" ERROR!
```

The compiler catches this IMMEDIATELY:

```
error: used binding `x` isn't initialized
```

Not when your program is running.
Not when users are using it.
Not at 3am when you're asleep.
RIGHT NOW, before you can even run it.

This is called "compile-time safety."

--- slide ---

# Visualizing the Problem

Here's what the code is trying to do:

```rust
let x;

if x == 10 { 
    println!("x is ten!");
}
```

Let's trace through it:

```
Step 1: Make empty box called x
┌─────────────┐
│     ???     │
└─────────────┘
       x

Step 2: Is x equal to 10?
┌─────────────┐
│     ???     │  == 10?  → MAKES NO SENSE!
└─────────────┘
```

How can you compare ??? to 10? You can't!

--- slide ---

# The Fix

Put something in the box when you create it:

```rust
let x = 10;
        ^^
        Now x has a value!
```

Now it makes sense:

```
Step 1: Make box with 10 inside
┌─────────────┐
│     10      │
└─────────────┘
       x

Step 2: Is x equal to 10?
┌─────────────┐
│     10      │  == 10?  → YES!
└─────────────┘
```

The code checks `if x == 10`, so putting `10` in 
the box makes the condition true!

(But any value works - just put SOMETHING in there.)
