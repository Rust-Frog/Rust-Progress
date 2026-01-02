# AsRef and AsMut: Cheap Conversions



## Reference Conversions

Sometimes you want a function that accepts

EITHER `String` OR `&str`. Same for mutations.

`AsRef` and `AsMut` make this easy!

```rust
fn print_len<T: AsRef<str>>(s: T) {
    println!("{}", s.as_ref().len());
}

print_len("hello");           // &str
print_len(String::from("hi")); // String
// Both work!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  AsRef<T>: "I can give you a &T"        │
│  AsMut<T>: "I can give you a &mut T"    │
│                                         │
│  No copying - just borrows!             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The AsRef Trait



## Getting a Reference

```rust
trait AsRef<T> {
    fn as_ref(&self) -> &T;
}
```

Types that implement `AsRef<str>`:
- `String` → gives `&str`
- `&str` → gives `&str`
- `Box<str>` → gives `&str`



```
┌─────────────────────────────────────────┐
│                                         │
│  String implements AsRef<str>:          │
│                                         │
│  let s = String::from("hello");         │
│  let r: &str = s.as_ref();              │
│                                         │
│  &str implements AsRef<str>:            │
│                                         │
│  let s = "hello";                       │
│  let r: &str = s.as_ref();              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Using AsRef in Generics



## Accept Multiple Types

```rust
// Without AsRef: only accepts &str
fn count_bytes(s: &str) -> usize {
    s.len()
}

// With AsRef: accepts String, &str, etc.
fn count_bytes<T: AsRef<str>>(s: T) -> usize {
    s.as_ref().len()
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  T: AsRef<str> means:                   │
│  "T can produce a &str via .as_ref()"   │
│                                         │
│  Inside the function:                   │
│  s.as_ref() gives you the &str          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise Part 1



## byte_counter and char_counter

```rust
// TODO: Add the AsRef trait bound
fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().len()
}

fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().count()
}
```

What trait bound lets us call `.as_ref()`?



```
┌─────────────────────────────────────────┐
│                                         │
│  We need: T: AsRef<str>                 │
│                                         │
│  fn byte_counter<T: AsRef<str>>(...)    │
│  fn char_counter<T: AsRef<str>>(...)    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The AsMut Trait



## Getting a Mutable Reference

```rust
trait AsMut<T> {
    fn as_mut(&mut self) -> &mut T;
}
```

For types that can give mutable access.



```
┌─────────────────────────────────────────┐
│                                         │
│  Box<u32> implements AsMut<u32>:        │
│                                         │
│  let mut b = Box::new(5);               │
│  *b.as_mut() = 10;                      │
│                                         │
│  The Box now contains 10!               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise Part 2



## num_sq: Square a Number

```rust
// TODO: Add trait bound and implement
fn num_sq<T>(arg: &mut T) {
    // Square the number inside
}
```

We need to:
1. Get mutable access to the inner u32
2. Square it: `*x = *x * *x` or `*x *= *x`



```
┌─────────────────────────────────────────┐
│                                         │
│  Trait bound: T: AsMut<u32>             │
│                                         │
│  Implementation:                        │
│  let num = arg.as_mut();                │
│  *num = *num * *num;                    │
│                                         │
│  Or shorter: *num *= *num;              │
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
│  1. What trait gives &str from T?       │
│     (AsRef<str>)                        │
│                                         │
│  2. What trait gives &mut u32 from T?   │
│     (AsMut<u32>)                        │
│                                         │
│  3. How do you add a trait bound?       │
│     (fn name<T: Trait>(arg: T))         │
│                                         │
│  4. How do you square *num?             │
│     (*num = *num * *num)                │
│                                         │
└─────────────────────────────────────────┘
```



AsRef/AsMut: flexible borrowing!

(Go try it in the Editor!)
