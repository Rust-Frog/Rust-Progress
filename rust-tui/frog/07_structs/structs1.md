# What Are Structs?



## Grouping Related Data Together

So far, you've worked with single values:

A number. A string. A boolean.

But what if you need to describe something MORE complex?



```
┌─────────────────────────────────────────┐
│                                         │
│  Think about a PERSON:                  │
│                                         │
│    - name (String)                      │
│    - age (number)                       │
│    - is_student (boolean)               │
│                                         │
│  Three different pieces of data...      │
│  but they all describe ONE thing!       │
│                                         │
└─────────────────────────────────────────┘
```



How do you bundle these together?

Enter: **Structs**



--- slide ---

# The Core Idea



## A Struct Is a Custom Type

A struct lets you CREATE YOUR OWN type

by combining other types together.



```
┌─────────────────────────────────────────┐
│                                         │
│  PRIMITIVE TYPES (built-in):            │
│    i32, u8, bool, String, char...       │
│                                         │
│  YOUR STRUCT (custom):                  │
│    Person { name, age, is_student }     │
│    Car { brand, year, color }           │
│    Point { x, y }                       │
│                                         │
│  You're creating NEW types!             │
│                                         │
└─────────────────────────────────────────┘
```



This is HUGE. You're not limited to

what Rust gives you anymore.



--- slide ---

# Real World Examples



## Where Structs Shine

Almost everything in real software uses structs:



```
┌─────────────────────────────────────────┐
│                                         │
│  🎮 Game Character                      │
│     → health, position, inventory       │
│                                         │
│  📧 Email Message                       │
│     → sender, recipient, subject, body  │
│                                         │
│  🎨 Color                               │
│     → red, green, blue values           │
│                                         │
│  📍 GPS Coordinate                      │
│     → latitude, longitude               │
│                                         │
│  🛒 Shopping Cart Item                  │
│     → product_name, quantity, price     │
│                                         │
└─────────────────────────────────────────┘
```



Structs turn messy groups of variables

into organized, meaningful units.



--- slide ---

# The Syntax



## Defining a Struct

Here's how you tell Rust about your new type:

```rust
struct Person {
    name: String,
    age: u32,
    is_student: bool,
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  struct  ← keyword                      │
│  Person  ← your type's name (PascalCase)│
│  { }     ← curly braces                 │
│                                         │
│  Inside the braces:                     │
│    field_name: Type,                    │
│    field_name: Type,                    │
│                                         │
│  Each field has a name AND a type.      │
│                                         │
└─────────────────────────────────────────┘
```



This is called a **Regular Struct**.



--- slide ---

# Creating an Instance



## From Definition to Data

Defining a struct is like creating a blueprint.

But blueprints don't hold data!

You need to create an **instance**.

```rust
let alice = Person {
    name: String::from("Alice"),
    age: 25,
    is_student: true,
};
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Person { ... }  ← use the struct name  │
│                                         │
│  Inside:                                │
│    field_name: value,                   │
│    field_name: value,                   │
│                                         │
│  Every field MUST be filled in!         │
│  You can't skip any.                    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Accessing Fields



## The Dot Operator

Once you have an instance, use `.` to access fields:

```rust
println!("{}", alice.name);    // "Alice"
println!("{}", alice.age);     // 25
```



```
┌─────────────────────────────────────────┐
│                                         │
│  instance.field_name                    │
│          ↑                              │
│          dot!                           │
│                                         │
│  alice.name      → gets the name        │
│  alice.age       → gets the age         │
│  alice.is_student → gets the boolean    │
│                                         │
└─────────────────────────────────────────┘
```



Simple and intuitive!



--- slide ---

# Three Types of Structs



## Rust Has Variety

Rust actually has THREE kinds of structs:



```
┌─────────────────────────────────────────┐
│                                         │
│  1. REGULAR STRUCT (named fields)       │
│     struct Color {                      │
│         red: u8,                        │
│         green: u8,                      │
│         blue: u8,                       │
│     }                                   │
│                                         │
│  2. TUPLE STRUCT (unnamed fields)       │
│     struct Color(u8, u8, u8);           │
│                                         │
│  3. UNIT STRUCT (no fields at all)      │
│     struct Marker;                      │
│                                         │
└─────────────────────────────────────────┘
```



Let's look at each one!



--- slide ---

# Regular Structs



## Named Fields

This is what you've already seen.

Each field has a **name** and a **type**.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

let rect = Rectangle {
    width: 100,
    height: 50,
};

// Access by name:
println!("Width: {}", rect.width);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  ✓ Self-documenting                     │
│  ✓ Order doesn't matter when creating   │
│  ✓ Clear what each value means          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Tuple Structs



## Unnamed Fields

Sometimes names feel like overkill.

Tuple structs skip the names!

```rust
struct Point(i32, i32);
struct Color(u8, u8, u8);

let origin = Point(0, 0);
let red = Color(255, 0, 0);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Access by INDEX (like tuples):         │
│                                         │
│  origin.0  → first value (x)            │
│  origin.1  → second value (y)           │
│                                         │
│  red.0     → first value (red)          │
│  red.1     → second value (green)       │
│  red.2     → third value (blue)         │
│                                         │
└─────────────────────────────────────────┘
```



Use `.0`, `.1`, `.2` instead of `.name`



--- slide ---

# When to Use Tuple Structs?



## Simple, Obvious Cases

Tuple structs work great when the meaning is obvious:



```
┌─────────────────────────────────────────┐
│                                         │
│  struct Point(i32, i32);                │
│    → Obviously x and y coordinates      │
│                                         │
│  struct Color(u8, u8, u8);              │
│    → Obviously RGB values               │
│                                         │
│  struct Meters(f64);                    │
│    → A "newtype" wrapper around f64     │
│                                         │
└─────────────────────────────────────────┘
```



```
┌─────────────────────────────────────────┐
│                                         │
│  DON'T use for confusing cases:         │
│                                         │
│  struct Person(String, u32, bool);      │
│    → What's the u32? Age? ID? Height?   │
│    → Use regular struct instead!        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Unit Structs



## No Fields At All

Sometimes you just need a TYPE, not data.

```rust
struct AlwaysEqual;
struct Marker;
struct Placeholder;
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Wait, what's the point?                │
│                                         │
│  Unit structs are used for:             │
│                                         │
│  • Markers / flags in the type system   │
│  • Implementing traits (coming later)   │
│  • When you need a type, not data       │
│                                         │
│  They take up ZERO memory!              │
│                                         │
└─────────────────────────────────────────┘
```



Creating a unit struct instance:

```rust
let marker = AlwaysEqual;
```

No `{}`, no `()`. Just the name.



--- slide ---

# Side by Side Summary



## The Three Forms

```
┌─────────────────────────────────────────┐
│                                         │
│  REGULAR STRUCT:                        │
│    Definition:  struct Foo { a: T }     │
│    Instance:    Foo { a: value }        │
│    Access:      instance.a              │
│                                         │
│  TUPLE STRUCT:                          │
│    Definition:  struct Foo(T, T)        │
│    Instance:    Foo(val1, val2)         │
│    Access:      instance.0, instance.1  │
│                                         │
│  UNIT STRUCT:                           │
│    Definition:  struct Foo;             │
│    Instance:    Foo                     │
│    Access:      (nothing to access!)    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Creating All Three Types

The exercise asks you to work with a COLOR theme.

You'll define and create instances of:

1. A **regular struct** with named fields
2. A **tuple struct** with positional fields
3. A **unit struct** with no fields



```
┌─────────────────────────────────────────┐
│                                         │
│  HINTS:                                 │
│                                         │
│  • Look at the test assertions          │
│    They tell you what fields/values     │
│    are expected!                        │
│                                         │
│  • RGB colors use values 0-255          │
│    What type fits that range?           │
│                                         │
│  • Unit struct instances are simple     │
│    Just the struct name, nothing else   │
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
│  1. What fields does the regular struct │
│     need? (Check the test assertions!)  │
│                                         │
│  2. What TYPE holds values 0-255?       │
│                                         │
│  3. For the tuple struct, how do you    │
│     access fields? (.name or .0?)       │
│                                         │
│  4. How do you create a unit struct     │
│     instance? (Curly braces? Parens?)   │
│                                         │
└─────────────────────────────────────────┘
```



Read the tests carefully.

They're your roadmap!

(Now go to the Editor and try it!)
