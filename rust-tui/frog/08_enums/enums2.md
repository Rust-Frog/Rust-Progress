# Enums With Data



## Beyond Simple Labels

In the previous exercise, variants were just labels:

`Resize`, `Move`, `Echo`... names without data.

But what if a variant needs to CARRY information?



```
┌─────────────────────────────────────────┐
│                                         │
│  Think about a "Move" message:          │
│                                         │
│  Just saying "Move" isn't enough.       │
│  Move WHERE? To what position?          │
│                                         │
│  The variant needs to include:          │
│    • The destination coordinates        │
│                                         │
│  "Resize" needs:                        │
│    • The new width and height           │
│                                         │
│  "Echo" needs:                          │
│    • The message to echo                │
│                                         │
└─────────────────────────────────────────┘
```



Enum variants can hold data!



--- slide ---

# The Power of Enum Data



## Why This Is Huge

In many languages, enums are just named integers.

Rust enums are FAR more powerful:

Each variant can hold DIFFERENT types of data!



```
┌───────────────────────────────────────────┐
│                                           │
│  enum Message {                           │
│      Quit,                     // no data │
│      Move { x: i32, y: i32 },  // struct  │
│      Write(String),            // 1 value │
│      ChangeColor(u8, u8, u8),  // 3 values│
│  }                                        │
│                                           │
│  ONE type, FOUR different data shapes!    │
│                                           │
│  This is called a "tagged union" or       │
│  "sum type" in computer science.          │
│                                           │
└───────────────────────────────────────────┘
```



--- slide ---

# Three Kinds of Variants



## The Full Picture

Rust enum variants come in three flavors:



```
┌─────────────────────────────────────────┐
│                                         │
│  1. UNIT VARIANT                        │
│     No data at all.                     │
│                                         │
│     Quit                                │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  2. TUPLE VARIANT                       │
│     Unnamed fields in parentheses.      │
│                                         │
│     Move(i32, i32)                      │
│     Echo(String)                        │
│     ChangeColor(u8, u8, u8)             │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  3. STRUCT VARIANT                      │
│     Named fields in curly braces.       │
│                                         │
│     Resize { width: u64, height: u64 }  │
│                                         │
└─────────────────────────────────────────┘
```



Let's examine each one!



--- slide ---

# Unit Variants (Review)



## The Simplest Kind

You already know these:

```rust
enum Status {
    Active,
    Inactive,
    Pending,
}

let current = Status::Active;
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Definition:  Active                    │
│  Creation:    Status::Active            │
│                                         │
│  No parentheses.                        │
│  No curly braces.                       │
│  No data attached.                      │
│                                         │
│  Just a label that says "I'm this one." │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Tuple Variants



## Data Without Names

Tuple variants hold data in parentheses,

accessed by position (like tuple structs):

```rust
enum Message {
    Echo(String),
    Move(i32, i32),
    Color(u8, u8, u8),
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  DEFINITION SYNTAX:                     │
│                                         │
│    VariantName(Type)                    │
│    VariantName(Type, Type)              │
│    VariantName(Type, Type, Type)        │
│                                         │
│  Put the types inside parentheses,      │
│  separated by commas.                   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Creating Tuple Variants



## Providing the Data

When you create a tuple variant, provide values:

```rust
// One value
let msg = Message::Echo(String::from("Hello!"));

// Two values
let movement = Message::Move(10, 20);

// Three values
let color = Message::Color(255, 128, 0);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Message::Echo(String::from("Hello!"))  │
│              ↑                          │
│              Provide the String value   │
│                                         │
│  Message::Move(10, 20)                  │
│               ↑   ↑                     │
│               Two i32 values            │
│                                         │
│  The values go inside parentheses,      │
│  matching the types you defined.        │
│                                         │
└─────────────────────────────────────────┘



-
```



--- slide ---

# Using Structs in Tuple Variants



## Composing Types

A tuple variant can hold ANY type, including structs:

```rust
struct Point {
    x: u64,
    y: u64,
}

enum Message {
    Move(Point),  // holds a Point struct!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  CREATING THIS VARIANT:                 │
│                                         │
│  Message::Move(Point { x: 10, y: 20 })  │
│               ↑                         │
│               A full Point struct       │
│               goes here!                │
│                                         │
│  The variant wraps the struct.          │
│  You're putting a Point inside a Move.  │
│                                         │
└─────────────────────────────────────────┘



-
```



--- slide ---

# Struct Variants



## Data With Names

Struct variants have named fields,

just like regular structs:

```rust
enum Message {
    Resize { width: u64, height: u64 },
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  DEFINITION SYNTAX:                     │
│                                         │
│    VariantName { field: Type, ... }     │
│                                         │
│  Looks like a struct inside an enum!    │
│                                         │
│  Use this when field names make the     │
│  code clearer:                          │
│                                         │
│    Resize { width: 10, height: 20 }     │
│             ↑           ↑               │
│             Clear what each value means │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Creating Struct Variants



## Named Field Syntax

When creating, use the field names:

```rust
let resize = Message::Resize {
    width: 800,
    height: 600,
};
```



```
┌─────────────────────────────────────────────┐
│                                             │
│  Message::Resize {                          │
│      width: 800,    ← named field           │
│      height: 600,   ← named field           │
│  }                                          │
│                                             │
│  Just like creating a regular struct,       │
│  but prefixed with the enum and variant.    │
│                                             │
│  Field ORDER doesn't matter:                │
│  Message::Resize { height: 600, width: 800 }│
│  Works the same!                            │
│                                             │
└─────────────────────────────────────────────┘
```



--- slide ---

# Tuple vs Struct Variants



## When to Use Which?

```
┌─────────────────────────────────────────┐
│                                         │
│  USE TUPLE VARIANTS when:               │
│                                         │
│  • Meaning is obvious from context      │
│    Echo(String)  ← clearly a message    │
│                                         │
│  • You're wrapping a single value       │
│    Move(Point)   ← clearly a position   │
│                                         │
│  • Order is natural/conventional        │
│    Color(u8, u8, u8)  ← RGB order       │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  USE STRUCT VARIANTS when:              │
│                                         │
│  • Fields could be confused             │
│    Resize { width, height }             │
│    (which is which without names?)      │
│                                         │
│  • Self-documenting code matters        │
│    Window { x, y, width, height }       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Mixing Variant Types



## All In One Enum

A single enum can mix ALL variant types:

```rust
enum Message {
    // Unit variant - no data
    Quit,

    // Struct variant - named fields
    Resize { width: u64, height: u64 },

    // Tuple variant - single struct
    Move(Point),

    // Tuple variant - single String
    Echo(String),

    // Tuple variant - three values
    ChangeColor(u8, u8, u8),
}
```



Each variant carries exactly what IT needs.



--- slide ---

# Why This Matters



## The Flexibility

```
┌─────────────────────────────────────────┐
│                                         │
│  WITHOUT enums with data:               │
│                                         │
│  You'd need separate structs:           │
│    struct QuitMessage;                  │
│    struct ResizeMessage { w, h }        │
│    struct MoveMessage { point }         │
│    struct EchoMessage { text }          │
│                                         │
│  And no way to handle them uniformly!   │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  WITH enums:                            │
│                                         │
│  ONE type: Message                      │
│  Handles ALL cases                      │
│  Type-safe switching (match)            │
│  Compiler ensures completeness          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# A Complete Example



## Putting It Together

```rust
struct Point { x: u64, y: u64 }

enum Message {
    Quit,
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
}

fn main() {
    // Create different message types
    let m1 = Message::Quit;
    let m2 = Message::Resize { width: 10, height: 20 };
    let m3 = Message::Move(Point { x: 5, y: 10 });
    let m4 = Message::Echo(String::from("Hi"));
    let m5 = Message::ChangeColor(255, 0, 128);
}
```



All five are type `Message`, but hold different data!



--- slide ---

# Enums Can Have Methods



## Just Like Structs

You can add methods to enums with `impl`:

```rust
impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

let msg = Message::Echo(String::from("hello"));
msg.call();  // prints the message
```



```
┌─────────────────────────────────────────┐
│                                         │
│  impl EnumName {                        │
│      fn method_name(&self) {            │
│          // method body                 │
│      }                                  │
│  }                                      │
│                                         │
│  Same syntax as struct methods!         │
│  The method works on ANY variant.       │
│                                         │
└─────────────────────────────────────────┘


-
```



--- slide ---

# Your Exercise



## Defining Complex Variants

Look at how Message is USED in main():

```rust
Message::Resize { width: 10, height: 30 }
Message::Move(Point { x: 10, y: 15 })
Message::Echo(String::from("hello world"))
Message::ChangeColor(200, 255, 255)
Message::Quit
```



```
┌─────────────────────────────────────────┐
│                                         │
│  DEDUCING THE VARIANT TYPES:            │
│                                         │
│  Resize { width: 10, height: 30 }       │
│    → Curly braces with names            │
│    → STRUCT VARIANT                     │
│    → Fields: width and height           │
│    → What type fits 10 and 30?          │
│                                         │
│  Move(Point { x: 10, y: 15 })           │
│    → Parentheses with a Point           │
│    → TUPLE VARIANT                      │
│    → Contains a Point struct            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Analyzing Each Variant



## Read the Usage Carefully

```
┌─────────────────────────────────────────┐
│                                         │
│  Echo(String::from("hello world"))      │
│    → Parentheses with a String          │
│    → TUPLE VARIANT with String          │
│                                         │
│  ChangeColor(200, 255, 255)             │
│    → Parentheses with THREE values      │
│    → TUPLE VARIANT with 3 values        │
│    → Values are 0-255 (RGB colors)      │
│    → What type fits 0-255?              │
│                                         │
│  Quit                                   │
│    → No parentheses, no braces          │
│    → UNIT VARIANT                       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Type Hints



## Choosing the Right Types

```
┌─────────────────────────────────────────┐
│                                         │
│  For Resize width/height:               │
│                                         │
│  Look at the Point struct in the code:  │
│    struct Point { x: u64, y: u64 }      │
│                                         │
│  Dimensions usually match coordinates.  │
│  What type does Point use?              │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  For ChangeColor values:                │
│                                         │
│  RGB colors go from 0 to 255.           │
│  What unsigned type fits exactly that?  │
│  (Hint: you used it in structs!)        │
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
│  1. Which variants use { } vs ( )?      │
│     (Look at how they're created!)      │
│                                         │
│  2. For struct variant Resize:          │
│     What are the field NAMES?           │
│     What TYPE should they be?           │
│                                         │
│  3. For tuple variant Move:             │
│     What TYPE goes in the parentheses?  │
│     (Hint: it's already defined!)       │
│                                         │
│  4. For tuple variant Echo:             │
│     What TYPE is being passed?          │
│                                         │
│  5. For tuple variant ChangeColor:      │
│     How many values? What TYPE each?    │
│                                         │
│  6. Which variant has NO data at all?   │
│                                         │
└─────────────────────────────────────────┘
```



The main() function shows you exactly

how each variant is constructed!

(Now go to the Editor and try it!)
