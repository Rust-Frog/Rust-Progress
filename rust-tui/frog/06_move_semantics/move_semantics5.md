# References and Ownership



## Putting It All Together

This is the final move_semantics exercise!

It combines everything you've learned.



But first, let's understand WHY this matters.

When should you borrow? When should you own?

These aren't arbitrary rules - there are real reasons!



--- slide ---

# Why Does Borrowing vs Owning Matter?



## The Real-World Impact

Imagine you're writing a program that processes files.



```
┌─────────────────────────────────────────┐
│                                         │
│  SCENARIO A: File Checker               │
│                                         │
│  You just want to CHECK if a file is    │
│  valid. You read it, check it, done.    │
│                                         │
│  Should the checker OWN the filename?   │
│  NO! The caller might need it later!    │
│  → Use BORROWING (&)                    │
│                                         │
│                                         │
│  SCENARIO B: File Deletor               │
│                                         │
│  You want to DELETE a file. After       │
│  deletion, the filename is meaningless. │
│                                         │
│  Should the deletor OWN the filename?   │
│  YES! The caller shouldn't use it after │
│  → Take OWNERSHIP (no &)                │
│                                         │
└─────────────────────────────────────────┘
```



**Ownership communicates intent!**



--- slide ---

# The Design Question



## What Does Your Function DO With Data?

Before writing any function, ask yourself:



```
┌─────────────────────────────────────────┐
│                                         │
│  DOES THE CALLER NEED THE DATA AFTER?   │
│                                         │
│                                         │
│  YES, they still need it:               │
│    → BORROW with &                      │
│    → You're just "looking at" their data│
│    → Give it back when done             │
│                                         │
│                                         │
│  NO, they're giving it away:            │
│    → Take OWNERSHIP (no &)              │
│    → The data is now yours              │
│    → Caller can't use it anymore        │
│    → You're responsible for cleanup     │
│                                         │
└─────────────────────────────────────────┘
```



This is a DESIGN decision you make as the programmer.



--- slide ---

# Understanding the & Symbol



## The Difference Between Owning and Borrowing

Let's visualize what happens in memory:



```
┌─────────────────────────────────────────┐
│                                         │
│  WITHOUT & (Takes Ownership):           │
│                                         │
│  fn consume(data: String)               │
│                                         │
│    BEFORE CALL:                         │
│      caller ─────▶ "Hello"              │
│                                         │
│    AFTER CALL:                          │
│      caller ─────▶ (nothing, moved!)    │
│      function ───▶ "Hello"              │
│                                         │
│    Caller LOST their data!              │
│                                         │
└─────────────────────────────────────────┘
```

```
┌─────────────────────────────────────────┐
│                                         │
│  WITH & (Borrows):                      │
│                                         │
│  fn look_at(data: &String)              │
│                                         │
│    DURING CALL:                         │
│      caller ─────▶ "Hello"              │
│                    ↑                    │
│      function ─────┘ (just a pointer!)  │
│                                         │
│    AFTER CALL:                          │
│      caller ─────▶ "Hello" (still owns!)│
│      function ───▶ (nothing, done)      │
│                                         │
│    Caller STILL HAS their data!         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# When to Use Each



## A Decision Framework

```
┌─────────────────────────────────────────┐
│                                         │
│  USE & (BORROW) WHEN:                   │
│                                         │
│  • You only need to READ the data       │
│    (Like reading a book at the library) │
│                                         │
│  • The caller needs the data afterward  │
│    (They want their book back!)         │
│                                         │
│  • Multiple functions will use the      │
│    same data one after another          │
│    (Pass the same book around)          │
│                                         │
└─────────────────────────────────────────┘
```

```
┌─────────────────────────────────────────┐
│                                         │
│  USE NO & (OWN) WHEN:                   │
│                                         │
│  • The function will CONSUME the data   │
│    (Like eating a sandwich - it's gone!)│
│                                         │
│  • The caller doesn't need it anymore   │
│    (They're giving it away)             │
│                                         │
│  • You want to TRANSFORM and return     │
│    something new (take cake ingredients,│
│    return a cake)                       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Reading the Code Comments



## What Each Function Should Do

Now let's look at your exercise:



```rust
// Shouldn't take ownership
fn get_char(data: String) -> char { ... }

// Should take ownership
fn string_uppercase(mut data: &String) { ... }
```

The COMMENTS tell you the intent.

The CODE might not match!



```
┌─────────────────────────────────────────┐
│                                         │
│  get_char says:                         │
│  "Shouldn't take ownership"             │
│                                         │
│  What does GET_char do?                 │
│  Just READS one character from the data.│
│  Doesn't consume it!                    │
│                                         │
│  But the signature is:                  │
│    fn get_char(data: String)            │
│                      ↑↑↑↑↑↑             │
│    NO & means it DOES take ownership!   │
│                                         │
│  That's a mismatch! The code doesn't    │
│  match its intent.                      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Function 1: get_char



## Why Shouldn't It Take Ownership?

```rust
// Shouldn't take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}
```

Think about what this function DOES:

- Takes a string
- Returns the LAST CHARACTER
- That's it!



```
┌─────────────────────────────────────────┐
│                                         │
│  Does it DESTROY the string? NO.        │
│  Does it TRANSFORM the string? NO.      │
│  Does it need exclusive access? NO.     │
│                                         │
│  It just READS one character.           │
│                                         │
│  📚 It's like looking at the last page  │
│     of a library book.                  │
│     You don't need to OWN the book      │
│     to read the last page!              │
│                                         │
│  So it should BORROW, not OWN.          │
│  But currently it takes ownership!      │
│                                         │
│  Add & to borrow instead of owning.     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Function 2: string_uppercase



## Why SHOULD It Take Ownership?

```rust
// Should take ownership
fn string_uppercase(mut data: &String) {
    data = data.to_uppercase();
    println!("{data}");
}
```

Think about what this function DOES:

- Takes a string
- Transforms it to UPPERCASE
- Creates a NEW string
- Prints it



```
┌─────────────────────────────────────────┐
│                                         │
│  Look at this line:                     │
│    data = data.to_uppercase();          │
│                                         │
│  .to_uppercase() creates a NEW String!  │
│  The old string is replaced.            │
│                                         │
│  🍳 It's like taking eggs and making    │
│     an omelet. The eggs are CONSUMED.   │
│     You can't give the eggs back!       │
│                                         │
│  The caller doesn't need the original   │
│  after - it's being transformed.        │
│                                         │
│  So it SHOULD own the data.             │
│  But currently it borrows (&String)!    │
│                                         │
│  Remove the & to take ownership.        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The main() Function



## Callers Must Match

When you change a function signature,

you also need to change how it's CALLED!



```rust
fn main() {
    let data = "Rust is great!".to_string();

    get_char(data);         // How is it called?

    string_uppercase(&data); // How is it called?
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  THE MATCHING RULE:                     │
│                                         │
│  If function expects: │ Call with:      │
│  ─────────────────────┼─────────────────│
│  fn f(x: String)      │ f(value)        │
│  fn f(x: &String)     │ f(&value)       │
│  fn f(x: &mut String) │ f(&mut value)   │
│                                         │
│  The call must match the signature!     │
│                                         │
│  If get_char now borrows → add &        │
│  If string_uppercase owns → remove &    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Order Also Matters!



## Can data Be Used After get_char?

Think about which function runs FIRST:



```
┌─────────────────────────────────────────┐
│                                         │
│  get_char runs FIRST:                   │
│    If it borrows (&), data still exists │
│    string_uppercase can use it after!   │
│                                         │
│  string_uppercase runs SECOND:          │
│    If it takes ownership, that's fine   │
│    No one needs data after printing!    │
│                                         │
│                                         │
│    get_char(&data)   ← borrows, gives   │
│         ↓               back            │
│    string_uppercase(data) ← takes,      │
│                            consumes     │
│                                         │
│  This flow makes sense!                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Think It Through



## The Exercise Asks for Minimal Changes

You can ONLY add or remove the `&` character.

No other changes allowed!



```
┌─────────────────────────────────────────┐
│  QUESTIONS:                             │
│                                         │
│  1. get_char just READS data.           │
│     Should it borrow (&) or own?        │
│     What change to the signature?       │
│                                         │
│  2. string_uppercase TRANSFORMS data.   │
│     Should it borrow (&) or own?        │
│     What change to the signature?       │
│                                         │
│  3. If get_char now borrows, does       │
│     the CALL need to change?            │
│                                         │
│  4. If string_uppercase now owns, does  │
│     the CALL need to change?            │
│                                         │
└─────────────────────────────────────────┘
```

Remember: READ = borrow, CONSUME = own!



(Go try adding/removing & in the Editor!)
