# Message Passing with Channels



## Another Way to Share Data

Instead of sharing memory, SEND messages!

Channels let threads communicate by passing values.



```
┌─────────────────────────────────────────┐
│                                         │
│  Shared State (Mutex):                  │
│  Threads access the SAME data           │
│                                         │
│  Message Passing (Channels):            │
│  Threads SEND data to each other        │
│                                         │
│  Thread 1 ───[value]───→ Thread 2       │
│             channel                     │
│                                         │
│  "Do not communicate by sharing memory; │
│   share memory by communicating."       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# mpsc: Multi-Producer, Single-Consumer



## The Channel Type

Rust's standard channel is `mpsc`:

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();
//   ↑   ↑
//   │   └── Receiver (rx = "receive")
//   └── Sender (tx = "transmit")
```



```
┌─────────────────────────────────────────┐
│                                         │
│  mpsc = Multiple Producer, Single Consumer│
│                                         │
│  Producer 1 ──┐                         │
│               ├──→ [channel] ──→ Consumer│
│  Producer 2 ──┘                         │
│                                         │
│  Many senders, ONE receiver!            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Sending and Receiving



## Basic Usage

```rust
let (tx, rx) = mpsc::channel();

// Send a value
tx.send(42).unwrap();

// Receive the value
let received = rx.recv().unwrap();
println!("Got: {}", received);  // 42
```



```
┌─────────────────────────────────────────┐
│                                         │
│  tx.send(value) ─────→ [channel buffer] │
│                              │          │
│                              ▼          │
│  rx.recv() ←──────── gets the value     │
│                                         │
│  .recv() BLOCKS until a value arrives   │
│  (or all senders are dropped)           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Multiple Senders



## Clone the Transmitter

To have multiple senders, CLONE the tx:

```rust
let (tx, rx) = mpsc::channel();

let tx2 = tx.clone();  // Now we have two senders!

// Thread 1 uses tx
// Thread 2 uses tx2
// Both send to the same rx
```



```
┌─────────────────────────────────────────┐
│                                         │
│  tx ────────┐                           │
│             ├──→ [channel] ──→ rx       │
│  tx2 ───────┘                           │
│  (clone)                                │
│                                         │
│  Clone creates another sender to the    │
│  SAME channel. Not a new channel!       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Sender Gets Moved



## The Ownership Problem

When you move tx into a thread, it's GONE:

```rust
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send(1).unwrap();  // tx moved here
});

thread::spawn(move || {
    tx.send(2).unwrap();  // ERROR! tx is gone!
});
```



```
┌─────────────────────────────────────────┐
│                                         │
│  First spawn: move tx ──→ Thread 1      │
│                                         │
│  Second spawn: tx is already moved!     │
│                No tx left to move!      │
│                                         │
│  Solution: Clone BEFORE moving!         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Solution



## Clone Before Spawning

```rust
let (tx, rx) = mpsc::channel();

let tx1 = tx.clone();  // Clone for thread 1
thread::spawn(move || {
    tx1.send(1).unwrap();
});

let tx2 = tx.clone();  // Clone for thread 2
thread::spawn(move || {
    tx2.send(2).unwrap();
});

// Or just use the original tx somewhere
drop(tx);  // Drop original if not needed
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Clone BEFORE spawn!                    │
│                                         │
│  let tx_clone = tx.clone();             │
│  thread::spawn(move || {                │
│      tx_clone.send(...);                │
│  });                                    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Two Threads, One Channel

You have a function that spawns two threads.

Both should send values through the same channel.

```rust
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    thread::spawn(move || {
        for val in q.first_half {
            tx.send(val).unwrap();  // tx moves!
        }
    });

    thread::spawn(move || {
        for val in q.second_half {
            tx.send(val).unwrap();  // tx is gone!
        }
    });
}
```



--- slide ---

# What's Wrong



## tx Gets Moved

The first `thread::spawn(move || ...)` moves tx.

The second thread can't use it anymore!



```
┌─────────────────────────────────────────┐
│                                         │
│  Current code:                          │
│                                         │
│  spawn(move || { tx.send... })          │
│        ↑                                │
│        tx moves into first thread       │
│                                         │
│  spawn(move || { tx.send... })          │
│        ↑                                │
│        ERROR: tx already moved!         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Fix



## Clone for Each Thread

```
┌─────────────────────────────────────────┐
│                                         │
│  BEFORE spawning each thread:           │
│                                         │
│  let tx1 = tx.clone();                  │
│  thread::spawn(move || {                │
│      // use tx1 here                    │
│  });                                    │
│                                         │
│  let tx2 = tx.clone();  // or just tx   │
│  thread::spawn(move || {                │
│      // use tx2 here                    │
│  });                                    │
│                                         │
│  Each thread gets its OWN clone!        │
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
│  1. Why can't both threads use the same │
│     tx directly?                        │
│     (It gets moved into the first one)  │
│                                         │
│  2. How do you create another sender    │
│     for the same channel?               │
│     (Hint: .clone())                    │
│                                         │
│  3. Where should you clone tx?          │
│     (Before each spawn, not inside)     │
│                                         │
│  4. For the second thread, you can      │
│     clone OR just use the original tx   │
│     (since nothing else needs it)       │
│                                         │
└─────────────────────────────────────────┘
```



Clone the sender to share the channel!

(Go try it in the Editor!)
