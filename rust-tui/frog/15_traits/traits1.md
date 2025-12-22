# Traits: Shared Behavior



## What Are Traits?

Traits define shared behavior across types:

```rust
trait Greet {
    fn say_hello(&self);
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  A trait is like a CONTRACT:            │
│                                         │
│  "Any type implementing this trait      │
│   MUST provide these methods."          │
│                                         │
│  Similar to:                            │
│  • Interfaces in Java/C#               │
│  • Protocols in Swift                   │
│  • Abstract base classes in Python      │
│                                         │
│  But Rust traits are more powerful!     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Defining a Trait



## The Syntax

```rust
trait TraitName {
    fn method_one(&self);
    fn method_two(&self) -> String;
    fn method_three(&mut self, arg: i32);
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  trait TraitName { ... }                │
│        ↑                                │
│        Name of the trait                │
│                                         │
│  Inside: method SIGNATURES              │
│  (No body - just the shape!)            │
│                                         │
│  fn name(&self) -> ReturnType;          │
│                               ↑         │
│                               Semicolon!│
│                                         │
│  The semicolon means "no body here"     │
│  Implementors provide the body.         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Implementing a Trait



## For a Specific Type

```rust
struct Dog {
    name: String,
}

impl Greet for Dog {
    fn say_hello(&self) {
        println!("Woof! I'm {}", self.name);
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  impl TraitName for Type { ... }        │
│       ↑             ↑                   │
│       │             └─ The type         │
│       └─ The trait                      │
│                                         │
│  "Implement this trait FOR this type"   │
│                                         │
│  Inside: provide the actual code        │
│  for each trait method.                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Using Trait Methods



## After Implementation

```rust
let dog = Dog { name: String::from("Rex") };
dog.say_hello();  // "Woof! I'm Rex"
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Once a trait is implemented:           │
│                                         │
│  • You can call trait methods on        │
│    instances of that type               │
│                                         │
│  • The type "has" that behavior         │
│                                         │
│  • Multiple types can implement the     │
│    same trait differently               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Multiple Implementations



## Same Trait, Different Types

```rust
trait Greet {
    fn say_hello(&self);
}

impl Greet for Dog {
    fn say_hello(&self) {
        println!("Woof!");
    }
}

impl Greet for Cat {
    fn say_hello(&self) {
        println!("Meow!");
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  ONE trait, MANY implementations!       │
│                                         │
│  Each type provides its OWN version     │
│  of the behavior.                       │
│                                         │
│  Dog::say_hello() → "Woof!"             │
│  Cat::say_hello() → "Meow!"             │
│                                         │
│  Same interface, different behavior.    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Implementing for Built-in Types



## You Can Extend Anything!

```rust
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(self) -> Self {
        // Add "Bar" to the string
        // Return the modified string
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  You can implement YOUR traits          │
│  for types you didn't create!           │
│                                         │
│  String is from the standard library.   │
│  But you can add behavior to it!        │
│                                         │
│  This is called "extension traits"      │
│                                         │
│  Now String has .append_bar() method!   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Self in Traits



## The Implementing Type

```rust
trait AppendBar {
    fn append_bar(self) -> Self;
    //           ↑         ↑
    //           │         └─ Returns same type
    //           └─ Takes ownership
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Self = the type implementing the trait │
│                                         │
│  In impl AppendBar for String:          │
│  • self is String (takes ownership)     │
│  • Self is String (return type)         │
│                                         │
│  The method consumes the value and      │
│  returns a new/modified value.          │
│                                         │
│  This allows CHAINING:                  │
│  s.append_bar().append_bar()            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# String Concatenation



## Ways to Append

```rust
// Method 1: Using +
let s = s + "Bar";

// Method 2: Using format!
let s = format!("{}Bar", s);

// Method 3: Using push_str (needs mut)
let mut s = s;
s.push_str("Bar");
s
```



```
┌─────────────────────────────────────────┐
│                                         │
│  For appending to String:               │
│                                         │
│  • + takes ownership of left side       │
│  • format! creates a new String         │
│  • push_str modifies in place           │
│                                         │
│  Since append_bar takes self (owned),   │
│  any of these approaches work!          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Implementing AppendBar for String

You have:

```rust
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for the type `String`.
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  TASK:                                  │
│                                         │
│  Fill in the implementation!            │
│                                         │
│  The method should:                     │
│  1. Take self (ownership of String)     │
│  2. Append "Bar" to it                  │
│  3. Return the result                   │
│                                         │
│  The signature is already given:        │
│  fn append_bar(self) -> Self            │
│                                         │
│  You provide the body!                  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Test Expectations



## What Should Happen

```rust
// "Foo" + "Bar" = "FooBar"
String::from("Foo").append_bar()  // → "FooBar"

// "" + "Bar" + "Bar" = "BarBar"
String::from("").append_bar().append_bar()  // → "BarBar"
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The method must:                       │
│                                         │
│  • Append exactly "Bar" (capital B)     │
│  • Return Self (so chaining works)      │
│  • Work on any String                   │
│                                         │
│  Chaining shows the value flows:        │
│  "" → "Bar" → "BarBar"                  │
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
│  1. What is the method signature?       │
│     fn append_bar(self) -> Self         │
│                                         │
│  2. What type is self in this impl?     │
│     (It's String - owned!)              │
│                                         │
│  3. How do you add "Bar" to a String?   │
│     (Think: + operator, format!, etc.)  │
│                                         │
│  4. What should you return?             │
│     (The String with "Bar" appended)    │
│                                         │
│  5. Do you need mut? Why or why not?    │
│     (Depends on your approach!)         │
│                                         │
│  6. Can you use self + "Bar" directly?  │
│     (Yes! String + &str works)          │
│                                         │
└─────────────────────────────────────────┘
```



Write the body of append_bar!

(Now go to the Editor and try it!)
