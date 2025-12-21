# The Semicolon Problem



## The Code

```
fn square(num: i32) -> i32 {
    num * num;    // <-- Look at this semicolon!
}
```



Hmm... sounds familiar right? Yeah, it's a semicolon.

But most programmers don't really know what this 
little semicolon actually DOES.



Java programmers might say: 
"Ah yes, it's the end of a line."

C programmers might say:
"Well, it terminates a statement."



But Rust is different. That tiny semicolon 
changes the MEANING of your code completely.



--- slide ---

# Expressions vs Statements



Rust has two types of lines:



```
┌────────────────────────────────────────────────┐
│                                                │
│  EXPRESSION:                                   │
│  → Calculates something                        │
│  → Has a result/value                          │
│  → Like asking "what is 2 + 2?"                │
│                                                │
│  STATEMENT:                                    │
│  → Does something                              │
│  → Has NO result                               │
│  → Like saying "remember this"                 │
│                                                │
└────────────────────────────────────────────────┘
```



Think of it like this:

- Expression = a question with an answer
- Statement = a command with no answer



--- slide ---

# The Semicolon Changes Everything



Here's the magic (or curse):

**Adding `;` turns an expression into a statement.**



```
num * num      ← Expression! Returns the result.

num * num;     ← Statement! Returns nothing.
          ^
          │
          └─────This one character changes everything.
```



The semicolon basically says:

"Yeah calculate that, but throw away the answer."



Weird right? But once you get it, you get it.



--- slide ---

# Why This Breaks The Function



Look at the function signature:

```
fn square(num: i32) -> i32
                       ^^^^
                       "I promise to return an i32"
```



Now look at the body:

```
{
    num * num;   ← Returns ()... nothing!
}
```



The function PROMISED to return an i32.

But it's returning () which is "nothing" in Rust.



```
┌────────────────────────────────────────────────┐
│                                                │
│  Promise:  -> i32 (a number)                   │
│  Reality:  -> ()  (nothing)                    │
│                                                │
│  Result:   ERROR - mismatched types!           │
│                                                │
└────────────────────────────────────────────────┘
```


--- slide ---

# Now Think About It



Go back to the code:

```
fn square(num: i32) -> i32 {
    num * num;
}
```



Ask yourself these questions:



```
┌────────────────────────────────────────────────┐
│                                                │
│  1. The function says `-> i32`                 │
│     What does this mean?                       │
│                                                │
│  2. The line says `num * num;`                 │
│     Is this an expression or a statement?      │
│                                                │
│  3. What does a statement return?              │
│     (Hint: we covered this earlier)            │
│                                                │
│  4. So what is the function actually returning?│
│                                                │
│  5. Is that what the function promised?        │
│                                                │
└────────────────────────────────────────────────┘
```



You're starting to see it now, right?



--- slide ---

# The Clue Is In The Character



Look at these two versions side by side:



```rust
VERSION A:            |    VERSION B:
                      |
fn square(num: i32)   |    fn square(num: i32)
  -> i32 {            |    -> i32 {
                      |
    num * num         |    num * num;
                      |            ^
}                     |       }    |
                      |         What's this?
```



One of these returns a number.

One of these returns nothing.



The ONLY difference is one character.

Which character? What does it do?



Think about what you learned on the previous slides.



--- slide ---

# Discovering The Answer



By now you should understand:



```
┌────────────────────────────────────────────────┐
│                                                │
│  num * num   = expression = has a value        │
│                                                │
│  num * num;  = statement  = returns ()         │
│           ^                                    │
│           └── This is the culprit              │
│                                                │
└────────────────────────────────────────────────┘
```



So if the function needs to return a value...

And the semicolon turns values into nothing...



What should you do?



(Go try it in the Editor! Please don't goto next slide without trying it!)

Remember Cheating means Fooling yourself.


--- slide ---

# How Rust Returns Values



## Method 1: Implicit (The Rusty Way)

Last line without semicolon = automatically returned

```rust
fn double(x: i32) -> i32 {
    x * 2     // Returned automatically
}
```



## Method 2: Explicit (When You Need It)

Use `return` keyword (for early returns)

```rust
fn double(x: i32) -> i32 {
    return x * 2;   // Explicit return
}
```



Most Rust code uses the implicit way.

It's cleaner and more "Rusty".



--- slide ---

# Remember This



```
┌────────────────────────────────────────────────┐
│                                                │
│  THE SEMICOLON RULE:                           │
│                                                │
│  num * num   ← Expression (has value)          │
│  num * num;  ← Statement (no value)            │
│                                                │
│  FOR RETURNING:                                │
│                                                │
│  Last line WITHOUT ; → returned                │
│  Last line WITH ;    → returns ()              │
│                                                │
│  THE FIX FOR THIS EXERCISE:                    │
│                                                │
│  Just remove the semicolon!                    │
│                                                │
└────────────────────────────────────────────────┘
```



The compiler even tells you:

```
help: remove this semicolon to return this value
```



Read error messages — Rust is trying to help!
