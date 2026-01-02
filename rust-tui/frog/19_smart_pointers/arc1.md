# Arc: Thread-Safe Reference Counting



## The Problem with Rc and Threads

`Rc<T>` is great for sharing data...

but it's NOT thread-safe!



```
┌─────────────────────────────────────────┐
│                                         │
│  Thread 1: Rc::clone(&data)  count: 2   │
│                      ↓                  │
│  Thread 2: Rc::clone(&data)  count: 2   │
│                                         │
│  RACE CONDITION!                        │
│  Both threads read count as 1,          │
│  both write 2... we lost a count!       │
│                                         │
│  Rc uses simple increment/decrement.    │
│  With multiple threads, this breaks.    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Enter Arc<T>



## Atomic Reference Counting

`Arc` stands for "Atomically Reference Counted".

It's just like Rc, but THREAD-SAFE!

```rust
use std::sync::Arc;

let data = Arc::new(vec![1, 2, 3]);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Arc uses ATOMIC operations:            │
│                                         │
│  • Atomic = indivisible                 │
│  • Either fully happens or doesn't      │
│  • Can't be interrupted mid-operation   │
│                                         │
│  Thread 1: Arc::clone ──→ count: 2      │
│  Thread 2: Arc::clone ──→ count: 3      │
│                                         │
│  No race conditions!                    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Arc vs Rc



## Same API, Different Guarantees

```rust
// Rc - single thread only
use std::rc::Rc;
let data = Rc::new(5);
let clone = Rc::clone(&data);

// Arc - safe across threads
use std::sync::Arc;
let data = Arc::new(5);
let clone = Arc::clone(&data);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Rc<T>                 Arc<T>           │
│  ─────                 ──────           │
│  Fast                  Slightly slower  │
│  Single thread         Multi-thread     │
│  Not Send              Is Send + Sync   │
│                                         │
│  Use Rc when you can, Arc when needed!  │
│                                         │
│  The performance cost of Arc is small,  │
│  but Rust wants you to be explicit.     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Sharing Data Across Threads



## The Pattern

When spawning threads that need shared data:

1. Wrap data in `Arc`
2. Clone the Arc for each thread
3. Move the clone into the thread



```
┌─────────────────────────────────────────┐
│                                         │
│  let data = Arc::new(vec![...]);        │
│                                         │
│  for i in 0..3 {                        │
│      let data_clone = Arc::clone(&data);│
│      thread::spawn(move || {            │
│          // use data_clone here         │
│      });                                │
│  }                                      │
│                                         │
│  Each thread gets its OWN Arc clone,    │
│  but they all point to the SAME data!   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Clone Before the Thread?



## Move Semantics Matter

The `move` keyword moves variables INTO the closure.

If you don't clone first, you move the ONLY Arc!

```rust
// WRONG - sun moves into first iteration
for i in 0..3 {
    thread::spawn(move || {
        println!("{}", data);  // data moved!
    });
    // Next iteration: data is gone!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The fix:                               │
│                                         │
│  for i in 0..3 {                        │
│      let clone = Arc::clone(&data);     │
│      //  ↑ Create clone BEFORE spawn    │
│      thread::spawn(move || {            │
│          // clone moves in, data stays  │
│      });                                │
│  }                                      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Sharing Numbers Across 8 Threads

You have a vector of numbers 0-99.

8 threads will each sum different numbers.



```
┌─────────────────────────────────────────┐
│                                         │
│  Thread 0: sum 0, 8, 16, 24, ...        │
│  Thread 1: sum 1, 9, 17, 25, ...        │
│  Thread 2: sum 2, 10, 18, 26, ...       │
│  ...                                    │
│  Thread 7: sum 7, 15, 23, 31, ...       │
│                                         │
│  All threads need to READ the same      │
│  vector of numbers!                     │
│                                         │
│  Solution: Arc<Vec<u32>>                │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# What You Need to Do



## Two TODOs

```rust
let numbers: Vec<_> = (0..100u32).collect();

// TODO 1: Wrap numbers in Arc
// let shared_numbers = ???;

for offset in 0..8 {
    // TODO 2: Clone the Arc for this thread
    // let child_numbers = ???;

    thread::spawn(move || {
        // child_numbers is used here
    });
}
```



--- slide ---

# Task Breakdown



## Step by Step

```
┌─────────────────────────────────────────┐
│                                         │
│  1. CREATE THE ARC                      │
│                                         │
│  let shared_numbers = Arc::new(numbers);│
│                                         │
│  This wraps the vector in Arc,          │
│  making it shareable across threads.    │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  2. CLONE FOR EACH THREAD               │
│                                         │
│  Inside the loop, BEFORE spawn:         │
│  let child_numbers = Arc::clone(&???);  │
│                                         │
│  What variable holds the Arc?           │
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
│  1. What type makes data shareable      │
│     across threads?                     │
│     (Hint: Atomic Reference Counted)    │
│                                         │
│  2. How do you wrap a value in Arc?     │
│     (Hint: Arc::new(...))               │
│                                         │
│  3. Why must we clone BEFORE spawn,     │
│     not inside it?                      │
│     (Hint: move semantics)              │
│                                         │
│  4. What's the function to clone an Arc?│
│     (Hint: Arc::clone(&...))            │
│                                         │
└─────────────────────────────────────────┘
```



Arc = Rc that works across threads!

(Go try it in the Editor!)
