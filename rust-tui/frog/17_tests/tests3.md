# Testing Panics



## Functions That Should Fail

Some functions are designed to panic on invalid input:

```rust
fn new(width: i32, height: i32) -> Rectangle {
    if width <= 0 || height <= 0 {
        panic!("Width and height must be positive");
    }
    Rectangle { width, height }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  How do you TEST that a panic happens?  │
│                                         │
│  If the function panics, the test       │
│  crashes... which looks like failure!   │
│                                         │
│  But sometimes panic IS the correct     │
│  behavior.                              │
│                                         │
│  We need to tell Rust: "This test       │
│  SHOULD panic - that means it passed!"  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The #[should_panic] Attribute



## Expecting a Panic

```rust
#[test]
#[should_panic]
fn test_negative_width() {
    Rectangle::new(-10, 10);  // Should panic!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  #[should_panic]                        │
│                                         │
│  "This test SHOULD panic"               │
│                                         │
│  If the function panics:                │
│    → Test PASSES ✓                      │
│                                         │
│  If the function doesn't panic:         │
│    → Test FAILS ✗                       │
│                                         │
│  It's the OPPOSITE of normal behavior!  │
│                                         │
│  Use for testing error conditions.      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Attribute Stacking



## Multiple Attributes

```rust
#[test]           // First: mark as test
#[should_panic]   // Second: expect panic
fn test_panic() {
    // ...
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Attributes stack on top of each other. │
│                                         │
│  #[test]                                │
│    → Run this as a test                 │
│                                         │
│  #[should_panic]                        │
│    → Expect it to panic                 │
│                                         │
│  Order doesn't matter, but convention   │
│  is #[test] first, then modifiers.      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Accessing Struct Fields



## In Assertions

```rust
struct Rectangle {
    width: i32,
    height: i32,
}

let rect = Rectangle::new(10, 20);

// Access fields directly
assert_eq!(rect.width, 10);
assert_eq!(rect.height, 20);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  To test struct construction:           │
│                                         │
│  1. Create the instance                 │
│  2. Check each field                    │
│                                         │
│  rect.width  → the width field          │
│  rect.height → the height field         │
│                                         │
│  Compare with expected values!          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The todo! Macro



## Placeholder for Code

```rust
assert_eq!(todo!(), 10);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  todo!() is a placeholder that panics   │
│  if executed.                           │
│                                         │
│  It means: "Code needed here!"          │
│                                         │
│  Replace todo!() with actual code:      │
│                                         │
│  BEFORE:                                │
│  assert_eq!(todo!(), 10);               │
│                                         │
│  AFTER:                                 │
│  assert_eq!(rect.width, 10);            │
│                                         │
│  The todo!() tells you what to do!      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Three Tests to Fix

```rust
// Test 1: Check width and height
#[test]
fn correct_width_and_height() {
    let rect = Rectangle::new(10, 20);
    assert_eq!(todo!(), 10); // Check width
    assert_eq!(todo!(), 20); // Check height
}

// Test 2: Should panic on negative width
#[test]
fn negative_width() {
    let _rect = Rectangle::new(-10, 10);
}

// Test 3: Should panic on negative height
#[test]
fn negative_height() {
    let _rect = Rectangle::new(10, -10);
}
```



--- slide ---

# Task 1: Field Access



## Replace todo!()

```
┌─────────────────────────────────────────┐
│                                         │
│  In correct_width_and_height:           │
│                                         │
│  let rect = Rectangle::new(10, 20);     │
│  assert_eq!(todo!(), 10);  // Width     │
│  assert_eq!(todo!(), 20);  // Height    │
│                                         │
│  Replace todo!() with field access:     │
│                                         │
│  • First todo!() → check width          │
│    How do you access rect's width?      │
│                                         │
│  • Second todo!() → check height        │
│    How do you access rect's height?     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Task 2 & 3: Should Panic



## Add the Attribute

```
┌─────────────────────────────────────────┐
│                                         │
│  Tests negative_width and negative_height│
│  are SUPPOSED to panic!                 │
│                                         │
│  Rectangle::new(-10, 10) panics         │
│  Rectangle::new(10, -10) panics         │
│                                         │
│  But without #[should_panic], the       │
│  panic looks like test failure!         │
│                                         │
│  ADD #[should_panic] to both tests:     │
│                                         │
│  #[test]                                │
│  #[should_panic]  ← Add this!           │
│  fn negative_width() { ... }            │
│                                         │
│  #[test]                                │
│  #[should_panic]  ← Add this!           │
│  fn negative_height() { ... }           │
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
│  FOR correct_width_and_height:          │
│                                         │
│  1. How do you access a struct field?   │
│     (instance.field_name)               │
│                                         │
│  2. What replaces the first todo!()?    │
│     (rect.??? for width)                │
│                                         │
│  3. What replaces the second todo!()?   │
│     (rect.??? for height)               │
│                                         │
│  FOR negative_width and negative_height:│
│                                         │
│  4. What attribute marks expected panic?│
│     (#[should_panic])                   │
│                                         │
│  5. Where does the attribute go?        │
│     (Above the fn, after #[test])       │
│                                         │
│  6. Do you need to change the fn body?  │
│     (No - just add the attribute!)      │
│                                         │
└─────────────────────────────────────────┘
```



Replace todo!() and add #[should_panic]!

(Now go to the Editor and try it!)
