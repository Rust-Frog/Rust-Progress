# Shared Mutable State with Mutex



## The Problem

Multiple threads want to modify the SAME data.

But Rust's rules say: only ONE mutable reference!



```
┌─────────────────────────────────────────┐
│                                         │
│  Thread 1: status.jobs_done += 1        │
│                ↘                        │
│                  → CONFLICT! ←          │
│                ↗                        │
│  Thread 2: status.jobs_done += 1        │
│                                         │
│  Both trying to mutate at the same time │
│  This is a DATA RACE - undefined!       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Arc Is Not Enough



## Sharing vs Mutating

Remember `Arc<T>` from smart pointers?

It lets multiple threads SHARE data.

But it doesn't let them MUTATE it safely!

```rust
let status = Arc::new(JobStatus { jobs_done: 0 });

// In thread:
status.jobs_done += 1;  // ERROR!
// Cannot mutate through Arc!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Arc<T> = shared ownership              │
│           (multiple threads can READ)   │
│                                         │
│  But for mutation, we need MORE.        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Enter Mutex



## Mutual Exclusion

`Mutex` = "Mutual Exclusion"

Only ONE thread can access the data at a time!

```rust
use std::sync::Mutex;

let data = Mutex::new(5);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Mutex is like a room with a LOCK:      │
│                                         │
│  ┌──────────────────┐                   │
│  │  🔒 Mutex        │                   │
│  │  ┌────────────┐  │                   │
│  │  │ data: 5    │  │                   │
│  │  └────────────┘  │                   │
│  └──────────────────┘                   │
│                                         │
│  Only one thread can hold the lock!     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Using a Mutex



## Lock, Use, Unlock

To access data in a Mutex, you must LOCK it:

```rust
let data = Mutex::new(5);

let mut guard = data.lock().unwrap();
*guard += 1;  // Now we can mutate!
// guard dropped here, lock released
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .lock() returns a MutexGuard           │
│                                         │
│  Thread 1: lock() ──→ 🔐 has lock       │
│  Thread 2: lock() ──→ ⏳ waiting...     │
│                                         │
│  When guard is dropped:                 │
│  Thread 1: drops guard ──→ 🔓 released  │
│  Thread 2: gets lock ──→ 🔐 has lock    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Arc + Mutex = Thread-Safe Mutation



## The Complete Pattern

```rust
use std::sync::{Arc, Mutex};

let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));
//           ↑         ↑
//           │         └── Mutex: safe mutation
//           └── Arc: shared across threads
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Arc<Mutex<T>>                          │
│                                         │
│  Arc  = Multiple threads can have it    │
│  Mutex = Only one can mutate at a time  │
│                                         │
│  Together = Safe shared mutable state!  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# In Each Thread



## Lock Before Mutating

```rust
let status_shared = Arc::clone(&status);

thread::spawn(move || {
    // Lock the mutex to get access
    let mut guard = status_shared.lock().unwrap();

    // Now mutate through the guard
    guard.jobs_done += 1;

    // guard dropped, lock released
});
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The MutexGuard (from .lock()):         │
│                                         │
│  • Gives mutable access to inner data   │
│  • Automatically unlocks when dropped   │
│  • Uses Deref so you can use it like &T │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Fix the Shared Counter

Current code (broken):

```rust
let status = Arc::new(JobStatus { jobs_done: 0 });

// In thread:
status_shared.jobs_done += 1;  // Can't mutate!
```

You need to:
1. Wrap JobStatus in a Mutex
2. Lock before mutating
3. Access jobs_done at the end



--- slide ---

# What You Need to Do



## Three Changes

```
┌─────────────────────────────────────────┐
│                                         │
│  1. ADD MUTEX TO THE TYPE               │
│                                         │
│  Change: Arc::new(JobStatus { ... })    │
│  To:     Arc::new(Mutex::new(...))      │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  2. LOCK BEFORE MUTATING                │
│                                         │
│  In thread, before += 1:                │
│  status_shared.lock().unwrap()          │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  3. PRINT THE FINAL VALUE               │
│                                         │
│  status.lock().unwrap().jobs_done       │
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
│  1. What type allows safe mutation      │
│     across threads?                     │
│     (Hint: Mutual Exclusion)            │
│                                         │
│  2. How do you wrap data in a Mutex?    │
│     (Hint: Mutex::new(...))             │
│                                         │
│  3. What method gets access to the      │
│     data inside a Mutex?                │
│     (Hint: .lock().unwrap())            │
│                                         │
│  4. Don't forget to import Mutex!       │
│     use std::sync::{Arc, Mutex};        │
│                                         │
└─────────────────────────────────────────┘
```



Arc for sharing, Mutex for mutating!

(Go try it in the Editor!)
