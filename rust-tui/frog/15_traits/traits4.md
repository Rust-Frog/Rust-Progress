# Trait Bounds



## Functions That Accept Traits

How do you write a function that accepts ANY type implementing a trait?

```rust
trait Licensed {
    fn licensing_info(&self) -> String;
}

fn print_license(item: ???) {
    println!("{}", item.licensing_info());
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  We want print_license to work with:    │
│                                         │
│  • SomeSoftware (implements Licensed)   │
│  • OtherSoftware (implements Licensed)  │
│  • Any future type with Licensed        │
│                                         │
│  We don't care WHAT type it is,         │
│  only that it HAS licensing_info().     │
│                                         │
│  This is what trait bounds are for!     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The impl Trait Syntax



## Simple Trait Bounds

```rust
fn print_license(item: impl Licensed) {
    println!("{}", item.licensing_info());
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  impl Licensed                          │
│  ↑    ↑                                 │
│  │    └─ The trait it must implement    │
│  └─ "implements"                        │
│                                         │
│  Reads as:                              │
│  "item is some type that implements     │
│   Licensed"                             │
│                                         │
│  The concrete type is decided at        │
│  compile time based on what's passed.   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Using impl Trait



## Calling the Function

```rust
struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware { ... }
impl Licensed for OtherSoftware { ... }

// Both work!
print_license(SomeSoftware);
print_license(OtherSoftware);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The function accepts ANY type that     │
│  implements Licensed.                   │
│                                         │
│  SomeSoftware implements Licensed? ✓    │
│  OtherSoftware implements Licensed? ✓   │
│                                         │
│  Both calls are valid!                  │
│                                         │
│  The compiler checks at compile time    │
│  that the trait is implemented.         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Generic Syntax Alternative



## Two Ways to Write It

```rust
// Syntax 1: impl Trait (simpler)
fn print_license(item: impl Licensed) { ... }

// Syntax 2: Generic with bound (more flexible)
fn print_license<T: Licensed>(item: T) { ... }
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Both are equivalent for simple cases!  │
│                                         │
│  impl Trait:                            │
│  • Shorter, easier to read              │
│  • Good for one or two parameters       │
│                                         │
│  Generic T: Trait:                      │
│  • More explicit                        │
│  • Better for complex bounds            │
│  • Can use T multiple times             │
│                                         │
│  Use whichever fits your situation.     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Multiple Parameters



## Each Can Have Different Types

```rust
fn compare(a: impl Licensed, b: impl Licensed) -> bool {
    a.licensing_info() == b.licensing_info()
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  IMPORTANT:                             │
│                                         │
│  a: impl Licensed                       │
│  b: impl Licensed                       │
│                                         │
│  a and b can be DIFFERENT types!        │
│                                         │
│  compare(SomeSoftware, OtherSoftware)   │
│  is valid!                              │
│                                         │
│  Both just need to implement Licensed.  │
│  They don't have to be the same type.   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why This Matters



## Flexibility in APIs

```rust
fn compare_license_types(
    software1: impl Licensed,
    software2: impl Licensed,
) -> bool {
    software1.licensing_info() == software2.licensing_info()
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  This function can compare:             │
│                                         │
│  • Two SomeSoftware                     │
│  • Two OtherSoftware                    │
│  • SomeSoftware with OtherSoftware      │
│  • Any mix of Licensed types!           │
│                                         │
│  The function doesn't care about the    │
│  concrete types - only the behavior.    │
│                                         │
│  This is the power of trait bounds!     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Fixing the Function Signature

You have:

```rust
trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

// TODO: Fix the compiler error by only changing the signature
fn compare_license_types(software1: ???, software2: ???) -> bool {
    software1.licensing_info() == software2.licensing_info()
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The ??? marks need to be replaced!     │
│                                         │
│  The function body calls:               │
│  • software1.licensing_info()           │
│  • software2.licensing_info()           │
│                                         │
│  So both parameters need a type that    │
│  has .licensing_info() method.          │
│                                         │
│  What trait provides this method?       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Test Cases



## How It's Called

```rust
// Test 1: SomeSoftware and OtherSoftware
compare_license_types(SomeSoftware, OtherSoftware)

// Test 2: OtherSoftware and SomeSoftware (reversed!)
compare_license_types(OtherSoftware, SomeSoftware)
```



```
┌─────────────────────────────────────────┐
│                                         │
│  NOTICE:                                │
│                                         │
│  The parameters are DIFFERENT types!    │
│                                         │
│  Call 1: (SomeSoftware, OtherSoftware)  │
│  Call 2: (OtherSoftware, SomeSoftware)  │
│                                         │
│  You can't use concrete types:          │
│  fn compare(s1: SomeSoftware, s2: SomeSoftware)│
│  would NOT accept OtherSoftware!        │
│                                         │
│  You need trait bounds.                 │
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
│  1. What method is called on the        │
│     parameters?                         │
│     (.licensing_info())                 │
│                                         │
│  2. What trait provides this method?    │
│     (Licensed)                          │
│                                         │
│  3. What syntax says "any type that     │
│     implements Trait"?                  │
│     (impl TraitName)                    │
│                                         │
│  4. What replaces ??? for software1?    │
│     (impl Licensed)                     │
│                                         │
│  5. What replaces ??? for software2?    │
│     (impl Licensed)                     │
│                                         │
│  6. Can they be different concrete types?│
│     (Yes! That's why we use impl Trait) │
│                                         │
└─────────────────────────────────────────┘
```



Replace both ??? with trait bounds!

(Now go to the Editor and try it!)
