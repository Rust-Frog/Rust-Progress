# The Problem: Handling Different Variants



## You Have an Enum Value... Now What?

You can CREATE enum values with different variants.

But how do you RESPOND differently to each one?



```
┌─────────────────────────────────────────┐
│                                         │
│  Imagine processing messages:           │
│                                         │
│  fn process(msg: Message) {             │
│      // msg could be ANY variant!       │
│      // How do I know which one?        │
│      // How do I get the data inside?   │
│  }                                      │
│                                         │
│  If it's Quit → do one thing            │
│  If it's Move → do something else       │
│  If it's Resize → yet another thing     │
│                                         │
│  We need to BRANCH based on the variant.│
│                                         │
└─────────────────────────────────────────┘
```



Enter: **Pattern Matching with match**



--- slide ---

# What is match?



## Rust's Powerful Control Flow

`match` lets you compare a value against patterns

and run code based on which pattern matches:

```rust
match some_value {
    pattern1 => do_something(),
    pattern2 => do_something_else(),
    pattern3 => do_another_thing(),
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  match value {                          │
│        ↑                                │
│        What you're examining            │
│                                         │
│      pattern => expression,             │
│      ↑           ↑                      │
│      What to     What to do             │
│      look for    if it matches          │
│                                         │
│  }                                      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# match With Enums



## The Perfect Pair

Match is DESIGNED for enums:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```



Each variant gets its own handler!



--- slide ---

# The Syntax Breakdown



## Understanding Each Part

```rust
match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  match coin {                           │
│                                         │
│      Coin::Penny => 1,                  │
│      ↑              ↑                   │
│      PATTERN        EXPRESSION          │
│      (what to       (what to return/do  │
│       match)         if matched)        │
│                                         │
│  Each line is called an "ARM".          │
│  Pattern on left, expression on right.  │
│  Arrow => connects them.                │
│  Comma separates arms.                  │
│                                         │
└─────────────────────────────────────────┘


-
```



--- slide ---

# Arms Can Have Code Blocks



## More Than One Line

If you need multiple statements, use curly braces:

```rust
match coin {
    Coin::Penny => {
        println!("Lucky penny!");
        1
    }
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Simple expression:  => value           │
│                                         │
│  Code block:         => {               │
│                          statement;     │
│                          statement;     │
│                          return_value   │
│                       }                 │
│                                         │
│  The last expression in the block       │
│  is the value returned.                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Exhaustiveness



## Match MUST Cover Everything

This is one of Rust's superpowers:

**You MUST handle every possible variant.**

```rust
enum Direction { North, South, East, West }

match direction {
    Direction::North => println!("Going up"),
    Direction::South => println!("Going down"),
    // COMPILE ERROR! East and West not covered!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The compiler FORCES completeness.      │
│                                         │
│  You can't forget a case!               │
│  You can't have unhandled variants!     │
│                                         │
│  If you add a new variant to an enum,   │
│  every match will fail to compile       │
│  until you handle the new case.         │
│                                         │
│  This prevents SO many bugs.            │
│                                         │
└─────────────────────────────────────────┘


-
```



--- slide ---

# Extracting Data From Variants



## The Real Power

When a variant holds data, you can EXTRACT it:

```rust
enum Message {
    Echo(String),
    Move { x: i32, y: i32 },
}

match message {
    Message::Echo(text) => {
        println!("Echo: {}", text);
    }
    Message::Move { x, y } => {
        println!("Move to ({}, {})", x, y);
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Message::Echo(text)                    │
│                ↑                        │
│                This BINDS the inner     │
│                String to 'text'!        │
│                                         │
│  Message::Move { x, y }                 │
│                  ↑  ↑                   │
│                  These BIND the fields  │
│                  to 'x' and 'y'!        │
│                                         │
│  You can then USE these variables       │
│  in the arm's expression.               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Destructuring Tuple Variants



## Pulling Out the Data

For tuple variants, use parentheses in the pattern:

```rust
enum Message {
    Echo(String),
    ChangeColor(u8, u8, u8),
    Move(Point),
}

match msg {
    Message::Echo(s) => {
        // s is the String
        println!("{}", s);
    }
    Message::ChangeColor(r, g, b) => {
        // r, g, b are the three u8 values
        println!("RGB: {}, {}, {}", r, g, b);
    }
    Message::Move(point) => {
        // point is the Point struct
        println!("x: {}, y: {}", point.x, point.y);
    }
}
```



--- slide ---

# Destructuring Struct Variants



## Named Fields in Patterns

For struct variants, use curly braces:

```rust
enum Message {
    Resize { width: u64, height: u64 },
}

match msg {
    Message::Resize { width, height } => {
        // width and height are now variables
        println!("New size: {}x{}", width, height);
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Message::Resize { width, height }      │
│                    ↑       ↑            │
│                    Field names become   │
│                    variable names!      │
│                                         │
│  This is called "destructuring" -       │
│  you're breaking the structure apart    │
│  and binding pieces to names.           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Unit Variants in match



## The Simplest Case

Unit variants have no data to extract:

```rust
enum Message {
    Quit,
}

match msg {
    Message::Quit => {
        // No data to extract
        // Just do whatever Quit means
        println!("Quitting!");
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Message::Quit => { ... }               │
│                                         │
│  No parentheses.                        │
│  No curly braces after variant name.    │
│  Just the variant name, then =>.        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# A Complete Example



## All Variant Types Together

```rust
enum Message {
    Quit,
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quit");
        }
        Message::Resize { width, height } => {
            println!("Resize to {}x{}", width, height);
        }
        Message::Move(point) => {
            println!("Move to ({}, {})", point.x, point.y);
        }
        Message::Echo(s) => {
            println!("Echo: {}", s);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Color: {}, {}, {}", r, g, b);
        }
    }
}
```



--- slide ---

# Using Extracted Data



## Calling Methods With It

The extracted data can be passed to functions:

```rust
impl State {
    fn resize(&mut self, width: u64, height: u64) { ... }
    fn echo(&mut self, s: String) { ... }
}

fn process(&mut self, msg: Message) {
    match msg {
        Message::Resize { width, height } => {
            self.resize(width, height);
        }
        Message::Echo(s) => {
            self.echo(s);
        }
        // ... other arms
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Extract data → Pass to method          │
│                                         │
│  Message::Resize { width, height }      │
│           ↓         ↓                   │
│  self.resize(width, height)             │
│                                         │
│  The match arm connects the enum data   │
│  to the method that needs it.           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Full Pattern



## match as a Router

Think of match as a MESSAGE ROUTER:

```
┌─────────────────────────────────────────┐
│                                         │
│           INCOMING MESSAGE              │
│                 │                       │
│                 ▼                       │
│           ┌─────────┐                   │
│           │  match  │                   │
│           └────┬────┘                   │
│                │                        │
│     ┌──────────┼──────────┐             │
│     │          │          │             │
│     ▼          ▼          ▼             │
│  ┌──────┐  ┌──────┐  ┌──────┐           │
│  │ Quit │  │Resize│  │ Move │  ...      │
│  └──┬───┘  └──┬───┘  └──┬───┘           │
│     │         │         │               │
│     ▼         ▼         ▼               │
│  quit()   resize()  move()              │
│                                         │
│  Each variant routes to its handler.    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Implementing process()

You have a `State` struct with methods:

- `resize(width, height)`
- `move_position(point)`
- `echo(s)`
- `change_color(r, g, b)`
- `quit()`

And a `Message` enum with variants that match!

Your task: write a `match` that routes each

message variant to its corresponding method.



--- slide ---

# The Method Signatures



## What You're Calling

```rust
impl State {
    fn resize(&mut self, width: u64, height: u64)
    fn move_position(&mut self, point: Point)
    fn echo(&mut self, s: String)
    fn change_color(&mut self, red: u8, green: u8, blue: u8)
    fn quit(&mut self)
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Each method expects specific arguments:│
│                                         │
│  resize needs:       width, height      │
│  move_position needs: a Point           │
│  echo needs:         a String           │
│  change_color needs: r, g, b values     │
│  quit needs:         nothing            │
│                                         │
│  Extract data from variants,            │
│  pass to the matching method!           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Message Variants



## What You're Matching

```rust
enum Message {
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  MATCHING VARIANTS TO METHODS:          │
│                                         │
│  Resize { width, height }               │
│    → call self.resize(width, height)    │
│                                         │
│  Move(point)                            │
│    → call self.move_position(point)     │
│                                         │
│  Echo(s)                                │
│    → call self.echo(s)                  │
│                                         │
│  ChangeColor(r, g, b)                   │
│    → call self.change_color(r, g, b)    │
│                                         │
│  Quit                                   │
│    → call self.quit()                   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Building the match



## Step by Step

```rust
fn process(&mut self, message: Message) {
    match message {
        // For each variant:
        // 1. Write the pattern (extract data)
        // 2. Write =>
        // 3. Call the appropriate method

        // Example structure:
        // Message::Variant => self.method(),
        // Message::Variant(x) => self.method(x),
        // Message::Variant { a, b } => self.method(a, b),
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Remember:                              │
│                                         │
│  • Struct variants use { field, field } │
│  • Tuple variants use (binding, ...)    │
│  • Unit variants have no extra syntax   │
│                                         │
│  • You MUST cover ALL five variants     │
│  • Compiler will error if you miss any  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Common Mistakes



## Watch Out For

```
┌─────────────────────────────────────────┐
│                                         │
│  MISTAKE 1: Wrong brackets              │
│                                         │
│  Message::Resize(w, h)    ← WRONG       │
│  Message::Resize { w, h } ← RIGHT       │
│                                         │
│  (Resize is a struct variant!)          │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  MISTAKE 2: Forgetting to call method   │
│                                         │
│  Message::Quit => quit()      ← WRONG   │
│  Message::Quit => self.quit() ← RIGHT   │
│                                         │
│  (Methods need self!)                   │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  MISTAKE 3: Missing a variant           │
│                                         │
│  The compiler will tell you!            │
│  "non-exhaustive patterns"              │
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
│  1. How many match arms do I need?      │
│     (One for each variant!)             │
│                                         │
│  2. For Message::Resize:                │
│     - What kind of variant is it?       │
│     - What fields does it have?         │
│     - What method should I call?        │
│                                         │
│  3. For Message::Move:                  │
│     - What does it contain?             │
│     - How do I extract a Point?         │
│     - What method takes a Point?        │
│                                         │
│  4. For Message::ChangeColor:           │
│     - How many values are inside?       │
│     - What names will I give them?      │
│     - What order does the method want?  │
│                                         │
│  5. For Message::Quit:                  │
│     - Any data to extract?              │
│     - What method handles quitting?     │
│                                         │
└─────────────────────────────────────────┘
```



Look at the tests to see what behavior is expected!

(Now go to the Editor and try it!)
