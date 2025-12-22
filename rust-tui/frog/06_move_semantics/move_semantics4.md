# Mutable Borrowing Rules



## Now We Get to the Deep Stuff

This exercise introduces one of Rust's most

important rules about borrowing.



```
┌─────────────────────────────────────────┐
│                                         │
│  RUST'S BORROWING RULE:                 │
│                                         │
│  You can have EITHER:                   │
│    • ONE mutable reference              │
│    OR                                   │
│    • ANY number of immutable references │
│                                         │
│  But NOT both at the same time!         │
│                                         │
└─────────────────────────────────────────┘
```



This might seem strict. Let's understand why.



--- slide ---

# What's a Reference?



## Borrowing vs Owning

Instead of GIVING ownership (moving),

you can LEND access (borrowing).



```
┌─────────────────────────────────────────┐
│                                         │
│  MOVING (Like before):                  │
│    "Here, take my car. It's yours now." │
│                                         │
│  BORROWING:                             │
│    "You can use my car for a bit.       │
│     But it's still mine."               │
│                                         │
└─────────────────────────────────────────┘
```



In code, borrowing uses the `&` symbol:

```rust
let x = 5;
let y = &x;   // y BORROWS x (doesn't own it)
```



--- slide ---

# Two Types of Borrows



## Immutable and Mutable References

```
┌─────────────────────────────────────────┐
│                                         │
│  IMMUTABLE REFERENCE: &x                │
│    "You can LOOK at my car."            │
│    Read-only access. Can't change it.   │
│                                         │
│                                         │
│  MUTABLE REFERENCE: &mut x              │
│    "You can DRIVE my car."              │
│    Read AND write access.               │
│                                         │
└─────────────────────────────────────────┘
```



```rust
let x = 5;
let y = &x;      // Can read x through y
let z = &mut x;  // Can read AND modify x through z
```



--- slide ---

# The Golden Rule



## Why Only One Mutable Reference?

Imagine if two things could modify the same data:



```
┌─────────────────────────────────────────┐
│                                         │
│  Person A has &mut to a list.           │
│  Person B has &mut to the SAME list.    │
│                                         │
│  Person A deletes item 3.               │
│  Person B tries to read item 3.         │
│                                         │
│  CRASH! Data race! Undefined behavior!  │
│                                         │
│                                         │
│  Rust PREVENTS this at compile time.    │
│  Only ONE &mut allowed at a time.       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# When IS It "At The Same Time"?



## Understanding Reference Lifetimes

This is the KEY to understanding the rule.

"At the same time" doesn't mean wall-clock time.

It means: **while the reference is still being used.**



```
┌─────────────────────────────────────────┐
│                                         │
│  📚 Think of borrowing a library book.  │
│                                         │
│  You CHECK OUT a book (create &mut).    │
│  You READ the book (use the reference). │
│  You RETURN the book (last use).        │
│                                         │
│  The book is "borrowed" from checkout   │
│  until you return it.                   │
│                                         │
│  Someone else can borrow it AFTER       │
│  you return it - not before!            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# What Does "Still Being Used" Mean?



## A Reference "Lives" Until Its Last Use

Rust is smart. It tracks when you STOP using a reference.



```
┌─────────────────────────────────────────┐
│                                         │
│  LINE 1:  let y = &mut x;               │
│           ↑ y is CREATED (borrowed)     │
│                                         │
│  LINE 2:  y.push(1);                    │
│           ↑ y is USED                   │
│                                         │
│  LINE 3:  y.push(2);                    │
│           ↑ y is USED AGAIN             │
│                                         │
│  LINE 4:  (y is never used again)       │
│           ↑ y is DEAD - book returned!  │
│                                         │
│  LINE 5:  let z = &mut x;               │
│           ↑ NOW z can borrow! y is done │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Timeline: When Are References Active?



## Visualizing the "Alive" Periods

```
┌─────────────────────────────────────────┐
│                                         │
│  Code:       Timeline:                  │
│                                         │
│  let y = &mut x;  y: ████               │
│  y.push(1);       y: ████▓▓▓▓           │
│  y.push(2);       y: ████████▓▓▓▓       │
│  // (y not used)  y:         ░░░░ DEAD  │
│  let z = &mut x;  z:             ████   │
│  z.push(3);       z:             ████▓▓ │
│                                         │
│  █ = created    ▓ = still used          │
│                 ░ = dead (returned)     │
│                                         │
│  y and z DON'T OVERLAP - that's OK!     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# When They DO Overlap (Error!)



## The Problem Case

```
┌─────────────────────────────────────────┐
│                                         │
│  Code:        Timeline:                 │
│                                         │
│  let y = &mut x;  y: ████               │
│  let z = &mut x;  y: ████████  z: ████  │
│                   ↑ OVERLAP!            │
│  y.push(42);      y: ████████████       │
│  z.push(13);                   z: ████▓▓│
│                                         │
│  y is STILL ACTIVE when z is created!   │
│  y isn't "returned" until after line 4. │
│                                         │
│  Two people trying to borrow the same   │
│  book at the same time = NOT ALLOWED!   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Simple Rule



## Finish Using One Before Starting Another

```
┌─────────────────────────────────────────┐
│                                         │
│  BROKEN:                                │
│    Borrow y → Borrow z → Use y → Use z  │
│    ↑ y and z both active at same time!  │
│                                         │
│                                         │
│  WORKS:                                 │
│    Borrow y → Use y → Borrow z → Use z  │
│    ↑ y finishes before z starts!        │
│                                         │
│                                         │
│  📚 Return the first book before        │
│     checking out the second one!        │
│                                         │
└─────────────────────────────────────────┘
```



Now you understand HOW lifetimes work.

Let's apply this to your exercise!



--- slide ---

# The Problem in Your Exercise



## Look at the Code

```rust
let mut x = Vec::new();
let y = &mut x;    // First mutable borrow
let z = &mut x;    // Second mutable borrow!
y.push(42);        // Uses y
z.push(13);        // Uses z
```



```
┌─────────────────────────────────────────┐
│                                         │
│  THE PROBLEM:                           │
│                                         │
│  Both y and z are:                      │
│    - Mutable references (&mut)          │
│    - To the SAME vector (x)             │
│    - Active at the SAME time!           │
│                                         │
│  y is created on line 2.                │
│  z is created on line 3.                │
│  y is still active! (used on line 4)    │
│                                         │
│  That's TWO &mut at once = ERROR!       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Constraint



## Only Reorder, Don't Add or Remove

The TODO says:

"Fix by REORDERING the lines."

"Don't add, change or remove any line."



```
┌─────────────────────────────────────────┐
│                                         │
│  You have these lines:                  │
│                                         │
│    let mut x = Vec::new();              │
│    let y = &mut x;                      │
│    let z = &mut x;                      │
│    y.push(42);                          │
│    z.push(13);                          │
│    assert_eq!(x, [42, 13]);             │
│                                         │
│  Same lines, different order.           │
│  That's the only thing allowed.         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Key Insight



## References End When Last Used

If you USE a reference earlier,

it becomes "inactive" earlier.

Then you can create a new mutable reference!



```
┌─────────────────────────────────────────┐
│                                         │
│  BROKEN ORDER:                          │
│                                         │
│    let y = &mut x;   // y starts        │
│    let z = &mut x;   // ERROR! y active │
│    y.push(42);                          │
│    z.push(13);                          │
│                                         │
│                                         │
│  WORKING ORDER (hint: think about it):  │
│                                         │
│    Create y, USE y, then y is done.     │
│    THEN create z, use z.                │
│    One at a time!                       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Think It Through



## What Order Makes Sense?



```
┌─────────────────────────────────────────┐
│  QUESTIONS:                             │
│                                         │
│  1. What line CREATES y?                │
│     What line USES y?                   │
│                                         │
│  2. If you put the "use y" line         │
│     RIGHT AFTER creating y...           │
│     Then y ends there!                  │
│                                         │
│  3. Can you do the same for z?          │
│     Create z, immediately use it.       │
│                                         │
│  4. What order of lines would make      │
│     y finish before z starts?           │
│                                         │
│  5. The assert needs to check the       │
│     final result - where should it go?  │
│                                         │
└─────────────────────────────────────────┘
```



(Go try reordering in the Editor!)
