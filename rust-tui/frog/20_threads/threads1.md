# Threads: Running Code Simultaneously



## What Are Threads?

A thread is an independent path of execution.

Your program can run multiple threads AT THE SAME TIME!



```
┌─────────────────────────────────────────┐
│                                         │
│  Without threads:                       │
│                                         │
│  Task A ───────→ Task B ───────→ Done   │
│  (sequential, one after another)        │
│                                         │
│  With threads:                          │
│                                         │
│  Task A ───────→                        │
│  Task B ───────→  All done together!    │
│  Task C ───────→                        │
│  (parallel, running simultaneously)     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Spawning a Thread



## thread::spawn()

To create a new thread, use `thread::spawn`:

```rust
use std::thread;

thread::spawn(|| {
    println!("Hello from new thread!");
});

println!("Hello from main thread!");
```



```
┌─────────────────────────────────────────┐
│                                         │
│  thread::spawn takes a CLOSURE:         │
│                                         │
│  thread::spawn(|| {                     │
│      // This code runs in new thread    │
│  });                                    │
│                                         │
│  The closure is the thread's work.      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Problem: Main Exits Too Soon



## Threads Need Time

What happens when main() ends?

```rust
thread::spawn(|| {
    thread::sleep(Duration::from_secs(1));
    println!("Thread done!");  // Never prints!
});
// main() exits immediately, kills all threads
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Main Thread:  Start ─→ End             │
│                          ↓              │
│                    Program exits!       │
│                                         │
│  Spawned:      Start ──────→ X          │
│                          ↑              │
│                    Killed before done!  │
│                                         │
│  The main thread doesn't WAIT.          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# JoinHandle: The Solution



## Waiting for Threads

`thread::spawn` returns a `JoinHandle`.

Call `.join()` on it to WAIT for the thread!

```rust
let handle = thread::spawn(|| {
    thread::sleep(Duration::from_secs(1));
    println!("Thread done!");
});

handle.join().unwrap();  // Wait here!
println!("Main continues after thread");
```



```
┌─────────────────────────────────────────┐
│                                         │
│  JoinHandle is like a "ticket" for      │
│  your thread.                           │
│                                         │
│  .join() means: "Wait until this        │
│  thread finishes before continuing"     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Getting Return Values



## Threads Can Return Data

The value returned by the closure comes back through `.join()`:

```rust
let handle = thread::spawn(|| {
    42  // Return value
});

let result = handle.join().unwrap();
println!("Thread returned: {}", result);  // 42
```



```
┌─────────────────────────────────────────┐
│                                         │
│  handle.join() returns:                 │
│                                         │
│  Result<T, Error>                       │
│    │                                    │
│    └── T is whatever the closure returns│
│                                         │
│  .unwrap() gets the T out               │
│  (panics if thread panicked)            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Collect Results from 10 Threads

You spawn 10 threads, each returns how long it took:

```rust
for i in 0..10 {
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(250));
        start.elapsed().as_millis()  // Returns time
    });
    handles.push(handle);
}
```

Now you need to collect all results!



--- slide ---

# What You Need to Do



## Join Each Thread

```rust
for handle in handles {
    // TODO: Collect the results of all threads
    // into the `results` vector.
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  For each handle:                       │
│                                         │
│  1. Call .join() to wait for thread     │
│  2. Call .unwrap() to get the value     │
│  3. Push the value into results         │
│                                         │
│  One line does all three:               │
│  results.push(handle.???().???());      │
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
│  1. What method waits for a thread to   │
│     finish?                             │
│     (Hint: rhymes with "coin")          │
│                                         │
│  2. What does .join() return?           │
│     (A Result that needs unwrapping)    │
│                                         │
│  3. How do you add to a vector?         │
│     (Hint: results.push(...))           │
│                                         │
│  4. Can you chain these together?       │
│     handle.join().unwrap()              │
│                                         │
└─────────────────────────────────────────┘
```



Join waits, unwrap gets the value!

(Go try it in the Editor!)
