# Multiple Trait Bounds



## Requiring More Than One Trait

Sometimes a function needs multiple behaviors:

```rust
fn some_func(item: ???) -> bool {
    item.some_function() && item.other_function()
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  This function calls TWO methods:       │
│                                         │
│  • some_function() from SomeTrait       │
│  • other_function() from OtherTrait     │
│                                         │
│  The item must implement BOTH traits!   │
│                                         │
│  How do we express this requirement?    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The + Syntax



## Combining Trait Bounds

```rust
fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  impl SomeTrait + OtherTrait            │
│                 ↑                       │
│                 Plus sign!              │
│                                         │
│  Reads as:                              │
│  "A type that implements SomeTrait      │
│   AND OtherTrait"                       │
│                                         │
│  The + combines multiple requirements.  │
│                                         │
│  Type must satisfy ALL listed traits.   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Which Types Qualify?



## Both Traits Required

```rust
struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}  // Has both!

struct PartialStruct;
impl SomeTrait for PartialStruct {}
// Missing OtherTrait!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  For impl SomeTrait + OtherTrait:       │
│                                         │
│  SomeStruct ✓                           │
│    Has SomeTrait: ✓                     │
│    Has OtherTrait: ✓                    │
│                                         │
│  PartialStruct ✗                        │
│    Has SomeTrait: ✓                     │
│    Has OtherTrait: ✗ (missing!)         │
│                                         │
│  ALL bounds must be satisfied!          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Generic Syntax Alternative



## With Explicit Type Parameter

```rust
// Using impl Trait
fn some_func(item: impl SomeTrait + OtherTrait) -> bool { ... }

// Using generic
fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool { ... }
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Both mean the same thing!              │
│                                         │
│  T: SomeTrait + OtherTrait              │
│   ↑                                     │
│   "T must implement both traits"        │
│                                         │
│  The + works the same way in both       │
│  syntaxes.                              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Where Clause



## For Complex Bounds

```rust
fn some_func<T>(item: T) -> bool
where
    T: SomeTrait + OtherTrait,
{
    item.some_function() && item.other_function()
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  where T: SomeTrait + OtherTrait        │
│                                         │
│  The where clause:                      │
│  • Comes after parameters, before body  │
│  • Lists all trait bounds               │
│  • Better for many/complex bounds       │
│                                         │
│  All three syntaxes are equivalent:     │
│  • impl Trait1 + Trait2                 │
│  • T: Trait1 + Trait2                   │
│  • where T: Trait1 + Trait2             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Three or More Traits



## Just Keep Adding

```rust
fn complex_func(item: impl A + B + C + D) -> bool {
    // item has methods from all four traits!
}

// Or with where:
fn complex_func<T>(item: T) -> bool
where
    T: A + B + C + D,
{
    // ...
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  + chains as many traits as needed:     │
│                                         │
│  impl A + B + C + D                     │
│                                         │
│  The type must implement ALL of them.   │
│                                         │
│  This is powerful for APIs that need    │
│  rich behavior from their inputs.       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Using Multiple Trait Bounds

You have:

```rust
trait SomeTrait {
    fn some_function(&self) -> bool { true }
}

trait OtherTrait {
    fn other_function(&self) -> bool { true }
}

// TODO: Fix by only changing the signature
fn some_func(item: ???) -> bool {
    item.some_function() && item.other_function()
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The function calls:                    │
│  • some_function() (from SomeTrait)     │
│  • other_function() (from OtherTrait)   │
│                                         │
│  item must implement BOTH traits!       │
│                                         │
│  How do you combine two trait bounds?   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Test Cases



## What Gets Passed

```rust
struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// Both are passed to some_func
some_func(SomeStruct)   // Must work
some_func(OtherStruct)  // Must work
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Both SomeStruct and OtherStruct        │
│  implement BOTH traits.                 │
│                                         │
│  Your trait bound must accept any       │
│  type with both implementations.        │
│                                         │
│  The solution works for both structs    │
│  without naming either specifically.    │
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
│  1. What methods are called on item?    │
│     (some_function AND other_function)  │
│                                         │
│  2. Which trait has some_function()?    │
│     (SomeTrait)                         │
│                                         │
│  3. Which trait has other_function()?   │
│     (OtherTrait)                        │
│                                         │
│  4. How do you require both traits?     │
│     (Use + to combine them!)            │
│                                         │
│  5. What's the syntax for impl with +?  │
│     (impl Trait1 + Trait2)              │
│                                         │
│  6. What replaces ??? in the signature? │
│     (impl SomeTrait + OtherTrait)       │
│                                         │
└─────────────────────────────────────────┘
```



Combine both traits with +!

(Now go to the Editor and try it!)
