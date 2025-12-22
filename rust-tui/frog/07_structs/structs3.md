# Structs With Behavior



## Data Alone Isn't Enough

So far, structs just hold data.

But what if you want them to DO things?



```
┌─────────────────────────────────────────┐
│                                         │
│  A Rectangle struct has width & height  │
│                                         │
│  Wouldn't it be nice to ask:            │
│    "Hey rectangle, what's your area?"   │
│                                         │
│  Instead of:                            │
│    calculate_area(rect.width, rect.height)│
│                                         │
│  We could write:                        │
│    rect.area()                          │
│                                         │
└─────────────────────────────────────────┘
```



This is what METHODS are for!



--- slide ---

# What Are Methods?



## Functions That Belong to a Struct

A method is just a function that:

1. Is defined inside an `impl` block
2. Takes `self` as its first parameter
3. Is called on an instance using dot notation



```
┌─────────────────────────────────────────┐
│                                         │
│  REGULAR FUNCTION:                      │
│    fn calculate_area(w: u32, h: u32)    │
│    calculate_area(rect.width, rect.height)│
│                                         │
│  METHOD:                                │
│    fn area(&self)                       │
│    rect.area()                          │
│                                         │
│  Methods KNOW about their struct!       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The impl Block



## Where Methods Live

You define methods in an `impl` block:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  impl Rectangle {                       │
│       ↑                                 │
│       The struct you're adding to       │
│                                         │
│  "I'm implementing behavior for         │
│   the Rectangle type"                   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Understanding &self



## The Magic First Parameter

Every method takes `self` as its first parameter.

This represents the instance the method is called on.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

let rect = Rectangle { width: 10, height: 5 };
rect.area();  // self = rect
```



```
┌─────────────────────────────────────────┐
│                                         │
│  When you call rect.area()              │
│                                         │
│  Rust automatically passes 'rect'       │
│  as 'self' to the method.               │
│                                         │
│  Inside the method:                     │
│    self.width  → rect.width  → 10       │
│    self.height → rect.height → 5        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why &self?



## Borrowing, Not Taking

Notice the `&` in `&self`:

```rust
fn area(&self) -> u32
        ↑
        Reference!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  &self  = "borrow the instance"         │
│           (can read, can't modify)      │
│                                         │
│  &mut self = "borrow mutably"           │
│              (can read AND modify)      │
│                                         │
│  self = "take ownership"                │
│         (rarely used)                   │
│                                         │
│  Most methods use &self                 │
│  They just need to READ the data.       │
│                                         │
└─────────────────────────────────────────┘
```



This connects to move semantics you learned!



--- slide ---

# Methods With Parameters



## Beyond Just &self

Methods can take additional parameters too:

```rust
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width &&
        self.height > other.height
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  rect1.can_hold(&rect2)                 │
│        ↑          ↑                     │
│        method     additional argument   │
│                                         │
│  self = rect1     other = rect2         │
│                                         │
│  The first param is always self.        │
│  Extra params come after.               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Return Types



## Methods Return Values

Just like regular functions!

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  -> u32   returns an unsigned integer   │
│  -> bool  returns true or false         │
│  -> String  returns a string            │
│                                         │
│  No arrow? Returns nothing (unit type)  │
│                                         │
│  The return type goes AFTER the         │
│  parameter list, before the body.       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Associated Functions



## Methods Without self

Not every function in `impl` needs `self`.

Functions without `self` are called **associated functions**:

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

// Called with :: not .
let rect = Rectangle::new(10, 5);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Associated functions:                  │
│    • Don't take self                    │
│    • Called with ::  (not dot)          │
│    • Often used as constructors         │
│                                         │
│  You've seen this before:               │
│    String::from("hello")                │
│    Vec::new()                           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Self vs self



## Capital Matters!

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  self (lowercase)                       │
│    The actual instance                  │
│    "this particular rectangle"          │
│                                         │
│  Self (uppercase)                       │
│    The TYPE itself                      │
│    Shorthand for "Rectangle"            │
│                                         │
│  In the above:                          │
│    -> Self  means  -> Rectangle         │
│    Self { }  means  Rectangle { }       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Putting It Together



## A Complete Example

```rust
struct Package {
    weight: u32,
    destination: String,
}

impl Package {
    // Associated function (constructor)
    fn new(weight: u32, destination: String) -> Self {
        Self { weight, destination }
    }

    // Method (uses &self)
    fn shipping_cost(&self) -> u32 {
        self.weight * 2  // $2 per gram
    }

    // Method with parameter
    fn is_heavier_than(&self, other: &Package) -> bool {
        self.weight > other.weight
    }
}
```



--- slide ---

# Your Exercise



## Implementing Methods

You have a `Package` struct with fields for

sender country, recipient country, and weight.

Your task: implement two methods.



```
┌─────────────────────────────────────────┐
│                                         │
│  METHOD 1: is_international             │
│                                         │
│  When is a package "international"?     │
│  Think about it:                        │
│    • Sender is in one country           │
│    • Recipient is in another country    │
│    • Are they the SAME or DIFFERENT?    │
│                                         │
│  What should this method RETURN?        │
│  (Look at how the tests USE it!)        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Second Method



## Calculating Fees

```
┌─────────────────────────────────────────┐
│                                         │
│  METHOD 2: get_fees                     │
│                                         │
│  Takes a cents_per_gram rate            │
│  Returns the total shipping cost        │
│                                         │
│  If something weighs 100 grams          │
│  And the rate is 3 cents per gram       │
│  What's the total? (Basic math!)        │
│                                         │
│  What type should it return?            │
│  (Look at the test assertions!)         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Reading the Tests



## Your Best Friend

The tests tell you EXACTLY what to do:

```rust
assert!(package.is_international());
assert!(!package.is_international());
assert_eq!(package.get_fees(3), 4500);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  assert!() expects a bool               │
│    → is_international returns bool      │
│                                         │
│  assert_eq!(x, 4500)                    │
│    → get_fees returns a number          │
│    → 1500 grams × 3 cents = 4500        │
│                                         │
│  The tests are your specification!      │
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
│  1. What does is_international need     │
│     to check? (Compare what fields?)    │
│                                         │
│  2. What does it return?                │
│     (true/false, a number, a string?)   │
│                                         │
│  3. For get_fees, what's the formula?   │
│     (weight times what?)                │
│                                         │
│  4. What does get_fees return?          │
│     (Look at assert_eq! in the tests)   │
│                                         │
│  5. Do you need to ADD return types     │
│     to the method signatures?           │
│                                         │
└─────────────────────────────────────────┘
```



The method BODIES and RETURN TYPES

both need your attention!

(Now go to the Editor and try it!)
