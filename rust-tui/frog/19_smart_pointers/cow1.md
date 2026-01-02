# Cow: Clone on Write



## The Idea

Sometimes you have data that MIGHT need to be modified.

If you modify it, you need an owned copy.

If you don't, borrowing is enough!



```
┌─────────────────────────────────────────┐
│                                         │
│  Cow = "Clone On Write"                 │
│                                         │
│  Start with borrowed data (&T)          │
│                    │                    │
│           ┌───────┴───────┐             │
│           ▼               ▼             │
│      No mutation      Mutation needed   │
│           │               │             │
│           ▼               ▼             │
│     Stay borrowed    Clone to owned     │
│      (efficient)     (only if needed)   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Two States



## Borrowed or Owned

`Cow` is an enum with two variants:

```rust
enum Cow<'a, T> {
    Borrowed(&'a T),  // Just a reference
    Owned(T),         // Owns the data
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Cow::Borrowed(&data)                   │
│  • Contains a reference                 │
│  • No copying happened                  │
│  • Cannot be mutated                    │
│                                         │
│  Cow::Owned(data)                       │
│  • Contains owned data                  │
│  • Either started owned, or was cloned  │
│  • Can be mutated                       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Creating a Cow



## From Borrowed or Owned Data

```rust
use std::borrow::Cow;

// From a reference (borrowed)
let vec = vec![1, 2, 3];
let cow: Cow<[i32]> = Cow::from(&vec);
// cow is Cow::Borrowed

// From owned data
let cow: Cow<[i32]> = Cow::from(vec![1, 2, 3]);
// cow is Cow::Owned
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Cow::from(&data) → Cow::Borrowed       │
│  Cow::from(data)  → Cow::Owned          │
│                                         │
│  The type tells you what you have!      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Magic: to_mut()



## Clone Only When Needed

When you call `.to_mut()`:

- If Borrowed: clones the data, becomes Owned
- If already Owned: just returns a mutable reference

```rust
let vec = vec![1, 2, 3];
let mut cow = Cow::from(&vec);  // Borrowed

cow.to_mut()[0] = 99;  // NOW it clones!
// cow is now Cow::Owned
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .to_mut() triggers the "write" in      │
│  "Clone on Write"!                      │
│                                         │
│  Before to_mut():  Cow::Borrowed(&[...])│
│  After to_mut():   Cow::Owned([...])    │
│                                         │
│  The clone happens LAZILY -             │
│  only when mutation is needed!          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# When NO Clone Happens



## Already Owned, or No Mutation

```rust
// Case 1: No mutation needed
let vec = vec![1, 2, 3];  // All positive
let mut cow = Cow::from(&vec);
// If we never call to_mut(), no clone!
// cow stays Borrowed

// Case 2: Already owned
let mut cow = Cow::from(vec![1, 2, 3]);
cow.to_mut()[0] = 99;  // No clone needed!
// Was already Owned, stays Owned
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Clone happens ONLY when:               │
│  • You call to_mut()                    │
│  • AND it was Borrowed                  │
│                                         │
│  No clone if:                           │
│  • You never mutate                     │
│  • OR it was already Owned              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## abs_all Function

The function makes all numbers in a slice absolute:

```rust
fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            input.to_mut()[ind] = -value;
            //    ↑ Clone happens here (if borrowed)
        }
    }
}
```

You need to predict: will it be Borrowed or Owned?



--- slide ---

# The Four Test Cases



## Think Through Each One

```
┌─────────────────────────────────────────┐
│                                         │
│  1. reference_mutation                  │
│     Input: &vec![-1, 0, 1]  (borrowed)  │
│     Has negative → needs mutation       │
│     Result: Cow::Owned (cloned!)        │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  2. reference_no_mutation               │
│     Input: &vec![0, 1, 2]   (borrowed)  │
│     No negatives → no mutation needed   │
│     Result: ???                         │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  3. owned_no_mutation                   │
│     Input: vec![0, 1, 2]    (owned)     │
│     No negatives → no mutation          │
│     Result: ???                         │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  4. owned_mutation                      │
│     Input: vec![-1, 0, 1]   (owned)     │
│     Has negatives → mutation needed     │
│     Result: ???                         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Key Insight



## Borrowed vs Owned

```
┌─────────────────────────────────────────┐
│                                         │
│  Started as BORROWED (&vec):            │
│                                         │
│  • No mutation needed → stays Borrowed  │
│  • Mutation needed → becomes Owned      │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  Started as OWNED (vec):                │
│                                         │
│  • No mutation needed → stays Owned     │
│  • Mutation needed → stays Owned        │
│                                         │
│  Once owned, always owned!              │
│  (You can't go back to borrowed)        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Think It Through



## Before You Type



```
┌─────────────────────────────────────────┐
│  FOR EACH TEST, ASK:                    │
│                                         │
│  1. Did it START as Borrowed or Owned?  │
│     (Look at Cow::from(&vec) vs (vec))  │
│                                         │
│  2. Does it NEED mutation?              │
│     (Are there any negative numbers?)   │
│                                         │
│  3. Based on 1 and 2, what's the final  │
│     state?                              │
│                                         │
│  Use: Cow::Borrowed(_) or Cow::Owned(_) │
│  The underscore matches any inner value │
│                                         │
└─────────────────────────────────────────┘
```



Cow: Pay for cloning only when you need to!

(Go try it in the Editor!)
