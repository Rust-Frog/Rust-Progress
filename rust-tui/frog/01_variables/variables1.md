# The Problem



Look at the code in the Editor:

```rust
fn main() {
    x = 5;
    println!("x has the value {x}");
}
```



The error says:

```
error: cannot find value `x` in this scope
```



This means Rust doesn't know what `x` is.



You're telling Rust "put 5 in x" but Rust is 
asking: "What is x? I've never heard of it!"



You need to INTRODUCE `x` to Rust first.

That's what the `let` keyword does.



--- slide ---

# What is `let`?



`let` is how you CREATE a new variable in Rust.



Think of it like making a labeled box:

```
Without let:           With let:
                       ┌─────────────┐
   5  → ???            │      5      │
                       └─────────────┘
   "Where do I go?"           x
                       "Now I have a home!"
```



The syntax is:

```rust
let name = value;
│    │       │
│    │       └── What you're storing
│    └────────── What you call it
└─────────────── "Create a new variable"
```



Without `let`, Rust doesn't create anything.

It just sees `x = 5` and asks "what is x?"



--- slide ---

# The Fix



Change this line:

```rust
x = 5;
```



To this:

```rust
let x = 5;
^^^
Add this keyword
```



That's it! Just add `let` before `x`.



The full fixed code:

```rust
fn main() {
    let x = 5;
    println!("x has the value {x}");
}
```



Now Rust knows:

1. Create a variable called `x`
2. Put `5` inside it
3. When you see `{x}` in the print, use that value



--- slide ---

# Why Does This Work?



When you write `let x = 5;`:



```
Step 1: Rust creates a "box" in memory
┌─────────────┐
│             │
│             │
└─────────────┘

Step 2: Labels it "x"
┌─────────────┐
│             │
└─────────────┘
       x

Step 3: Puts 5 inside
┌─────────────┐
│      5      │
└─────────────┘
       x

Step 4: Now {x} in println! means "get what's in x"
Output: "x has the value 5"
```



Without `let`, steps 1-3 never happen.

Rust has no box, no label, nothing to put 5 into.
