# Reference Counting with Rc<T>



## The Problem of Multiple Owners

Sometimes, one piece of data needs MULTIPLE owners.

But Rust's ownership rules say: ONE owner only!



```
┌─────────────────────────────────────────┐
│                                         │
│  Think of a Sun with many Planets:      │
│                                         │
│           ☀ Sun                         │
│          /│\                            │
│         / │ \                           │
│        /  │  \                          │
│       🌍  🪐  🔴                         │
│     Earth Jupiter Mars                  │
│                                         │
│  Each planet "has" the sun...           │
│  but there's only ONE sun!              │
│                                         │
│  How can multiple planets OWN the sun?  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Enter Rc<T>



## Reference Counted Pointer

`Rc` stands for "Reference Counted".

It keeps track of HOW MANY owners exist!

```rust
use std::rc::Rc;

let sun = Rc::new(Sun);  // Count: 1
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Rc<T> contains:                        │
│  • The actual data (T)                  │
│  • A counter of references              │
│                                         │
│  ┌──────────────────┐                   │
│  │ Rc<Sun>          │                   │
│  │ ┌──────────────┐ │                   │
│  │ │ count: 1     │ │                   │
│  │ │ data: Sun ☀  │ │                   │
│  │ └──────────────┘ │                   │
│  └──────────────────┘                   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Cloning an Rc



## Creating More Owners

To create another owner, use `Rc::clone()`:

```rust
let sun = Rc::new(Sun);           // Count: 1
let sun2 = Rc::clone(&sun);       // Count: 2
let sun3 = Rc::clone(&sun);       // Count: 3
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Important: Rc::clone does NOT copy     │
│  the data! It just increments the       │
│  counter and returns another pointer.   │
│                                         │
│  sun ────────┐                          │
│              ▼                          │
│  sun2 ───→ ┌──────────────┐             │
│            │ count: 3     │             │
│  sun3 ───→ │ data: Sun ☀  │             │
│            └──────────────┘             │
│                                         │
│  All three point to the SAME data!      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Checking the Count



## Rc::strong_count()

You can see how many owners exist:

```rust
let sun = Rc::new(Sun);
println!("{}", Rc::strong_count(&sun));  // 1

let sun2 = Rc::clone(&sun);
println!("{}", Rc::strong_count(&sun));  // 2
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Rc::strong_count(&rc)                  │
│                                         │
│  Returns: how many Rc pointers exist    │
│  to this data                           │
│                                         │
│  This is useful for:                    │
│  • Debugging                            │
│  • Understanding ownership              │
│  • This exercise!                       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# When Owners Are Dropped



## The Count Goes Down

When an Rc goes out of scope, the count decreases:

```rust
let sun = Rc::new(Sun);    // Count: 1
{
    let sun2 = Rc::clone(&sun);  // Count: 2
}   // sun2 dropped, Count: 1

// When count reaches 0, data is freed
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The Rule:                              │
│                                         │
│  • clone() → count goes UP              │
│  • drop()  → count goes DOWN            │
│  • count = 0 → data is freed            │
│                                         │
│  This is automatic memory management!   │
│  No garbage collector needed.           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## The Solar System

You have a Sun and multiple Planets.

Each Planet holds an `Rc<Sun>`.



```rust
let sun = Rc::new(Sun);

let mercury = Planet::Mercury(Rc::clone(&sun));
let venus = Planet::Venus(Rc::clone(&sun));
// ...
```

The test checks that the reference count

matches the number of planets holding the sun.



--- slide ---

# The Problem in the Code



## Some Planets Create NEW Suns!

Look at the exercise:

```rust
let saturn = Planet::Saturn(Rc::new(Sun));  // WRONG!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Rc::new(Sun)  vs  Rc::clone(&sun)      │
│                                         │
│  Rc::new(Sun):                          │
│  • Creates a BRAND NEW Sun              │
│  • Separate reference count             │
│  • NOT the same sun!                    │
│                                         │
│  Rc::clone(&sun):                       │
│  • Points to the EXISTING sun           │
│  • Increments the count                 │
│  • Same sun, multiple owners!           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# What You Need to Fix



## Two Types of Problems

```
┌─────────────────────────────────────────┐
│                                         │
│  PROBLEM 1: Wrong clone method          │
│                                         │
│  Some planets use Rc::new(Sun)          │
│  instead of Rc::clone(&sun)             │
│                                         │
│  Find these and fix them!               │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  PROBLEM 2: Missing drops               │
│                                         │
│  At the end, planets need to be         │
│  dropped to decrease the count.         │
│                                         │
│  Look for TODO comments!                │
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
│  1. To share the SAME sun between       │
│     planets, should you use:            │
│     Rc::new(Sun) or Rc::clone(&sun)?    │
│                                         │
│  2. If count should be 9 with all       │
│     planets, but some use Rc::new,      │
│     what's wrong?                       │
│                                         │
│  3. To make count go from 5 to 4,       │
│     what do you need to drop?           │
│                                         │
│  4. What function explicitly drops      │
│     a value? (Hint: drop(...))          │
│                                         │
└─────────────────────────────────────────┘
```



All planets should share ONE sun via Rc::clone!

(Go try it in the Editor!)
