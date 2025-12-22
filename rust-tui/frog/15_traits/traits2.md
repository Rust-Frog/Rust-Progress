# Implementing Traits for Different Types



## One Trait, Many Types

The same trait can be implemented for completely different types:

```rust
trait AppendBar {
    fn append_bar(self) -> Self;
}

// Already done: impl for String
// Now: impl for Vec<String>
```



```
┌─────────────────────────────────────────┐
│                                         │
│  String and Vec<String> are DIFFERENT:  │
│                                         │
│  String: a sequence of characters       │
│  Vec<String>: a list of Strings         │
│                                         │
│  But both can "append Bar"!             │
│                                         │
│  For String: append "Bar" to the text   │
│  For Vec: push "Bar" as a new element   │
│                                         │
│  Same concept, different implementations│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Traits for Generic Types



## Implementing for Vec<T>

```rust
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        // Push "Bar" into the vector
        // Return the modified vector
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Vec<String> is a concrete type:        │
│                                         │
│  impl AppendBar for Vec<String>         │
│                     ↑                   │
│                     Specific type!      │
│                                         │
│  This impl ONLY works for Vec<String>.  │
│  Not Vec<i32>, not Vec<&str>.           │
│                                         │
│  Each generic instantiation is its      │
│  own type!                              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Vector Operations



## Adding Elements

```rust
let mut vec = vec![String::from("Foo")];

// Push adds to the end
vec.push(String::from("Bar"));

// Vec is now: ["Foo", "Bar"]
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .push(element) adds to END of vector   │
│                                         │
│  Before: ["Foo"]                        │
│  After:  ["Foo", "Bar"]                 │
│                                         │
│  push() requires:                       │
│  • mut access to the vector             │
│  • Element of the correct type          │
│                                         │
│  For Vec<String>, push takes String     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Ownership in Trait Methods



## self vs &self vs &mut self

```rust
trait Example {
    fn takes_ownership(self);       // Consumes
    fn borrows(&self);              // Reads
    fn borrows_mutably(&mut self);  // Modifies
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  AppendBar uses: fn append_bar(self)    │
│                                ↑        │
│                                Takes    │
│                                ownership│
│                                         │
│  This means:                            │
│  • You OWN the vector                   │
│  • You can modify it                    │
│  • You must return it                   │
│                                         │
│  Pattern:                               │
│  1. Take ownership (self)               │
│  2. Make mut binding                    │
│  3. Modify                              │
│  4. Return                              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The mut Rebinding Pattern



## Modifying Owned Values

```rust
fn append_bar(self) -> Self {
    let mut vec = self;  // Rebind as mutable
    vec.push(...);       // Now we can modify
    vec                  // Return
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  self is immutable by default.          │
│                                         │
│  To modify, rebind as mut:              │
│  let mut vec = self;                    │
│                                         │
│  OR take mut directly:                  │
│  fn append_bar(mut self) -> Self        │
│               ↑                         │
│               Add mut here!             │
│                                         │
│  Both work. Choose what's clearer.      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# What to Push



## Creating the String

```rust
vec.push(String::from("Bar"));
// Or
vec.push("Bar".to_string());
// Or
vec.push("Bar".into());
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Vec<String> needs String elements.     │
│                                         │
│  "Bar" is &str (string literal)         │
│  Must convert to String!                │
│                                         │
│  Ways to convert &str → String:         │
│                                         │
│  • String::from("Bar")                  │
│  • "Bar".to_string()                    │
│  • "Bar".to_owned()                     │
│  • "Bar".into()                         │
│                                         │
│  All work. Use whichever you prefer.    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Implementing AppendBar for Vec<String>

You have the trait defined:

```rust
trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement the trait `AppendBar` for a vector of strings.
// `append_bar` should push the string "Bar" into the vector.
```



```
┌─────────────────────────────────────────┐
│                                         │
│  TASK:                                  │
│                                         │
│  1. Write: impl AppendBar for Vec<String>│
│                                         │
│  2. Implement append_bar:               │
│     • Take ownership of the vector      │
│     • Push "Bar" as a String            │
│     • Return the vector                 │
│                                         │
│  Remember: push() needs mut access!     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Test Expectations



## What Should Happen

```rust
let mut foo = vec![String::from("Foo")].append_bar();
// foo is now: ["Foo", "Bar"]

foo.pop().unwrap()  // → "Bar" (last element)
foo.pop().unwrap()  // → "Foo" (first element)
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .pop() removes from END of vector.     │
│                                         │
│  If vector is ["Foo", "Bar"]:           │
│  • pop() returns Some("Bar")            │
│  • pop() returns Some("Foo")            │
│                                         │
│  Test pops "Bar" first, then "Foo".     │
│  So "Bar" must be pushed AFTER "Foo".   │
│                                         │
│  push() adds to end ✓                   │
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
│  1. What type are you implementing for? │
│     (Vec<String>)                       │
│                                         │
│  2. How do you write the impl header?   │
│     impl AppendBar for ???              │
│                                         │
│  3. What is self in this context?       │
│     (A Vec<String>)                     │
│                                         │
│  4. How do you add to a vector?         │
│     (The push() method)                 │
│                                         │
│  5. Do you need mut? How do you get it? │
│     (Yes - rebind or use mut self)      │
│                                         │
│  6. What type does push() need?         │
│     (String, not &str!)                 │
│                                         │
│  7. What do you return?                 │
│     (The modified vector)               │
│                                         │
└─────────────────────────────────────────┘
```



Write the full impl block!

(Now go to the Editor and try it!)
