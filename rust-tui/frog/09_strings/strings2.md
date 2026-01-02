# The Other Direction



## String to &str

Last exercise: you converted `&str` → `String`

Now let's go the OTHER way: `String` → `&str`



```
┌─────────────────────────────────────────┐
│                                         │
│  Why would you need this?               │
│                                         │
│  Many functions take &str as input      │
│  because they only need to READ.        │
│                                         │
│  If you have a String, you need to      │
│  give them a &str view into it.         │
│                                         │
└─────────────────────────────────────────┘
```



This conversion is CHEAP (unlike the other way).



--- slide ---

# The Scenario



## Function Expects &str, You Have String

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let my_name = String::from("Alice");
    greet(my_name);  // ERROR! Type mismatch
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  greet() wants:  &str (borrowed)        │
│  You have:       String (owned)         │
│                                         │
│  These are different types!             │
│                                         │
│  You need to BORROW from your String    │
│  to get a &str.                         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Take &str?



## A Design Pattern

Functions often take `&str` instead of `String`:

```rust
// This function can accept BOTH:
fn process(text: &str) { ... }

process("literal");              // &str ✓
process(&my_string);             // String → &str ✓
```



```
┌─────────────────────────────────────────┐
│                                         │
│  WHY TAKE &str?                         │
│                                         │
│  • More flexible - accepts both types   │
│  • Doesn't take ownership of your data  │
│  • Caller keeps their String            │
│  • Just needs to READ, not own          │
│                                         │
│  This is a common Rust idiom:           │
│  "Accept references when you only read" │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Converting String to &str



## Just Add &

The simplest way: use the reference operator `&`

```rust
let my_string = String::from("hello");
let my_slice: &str = &my_string;
```



```
┌─────────────────────────────────────────┐
│                                         │
│  &my_string                             │
│  ↑                                      │
│  Borrow the String as &str              │
│                                         │
│  When you borrow a String, Rust         │
│  automatically gives you a &str!        │
│                                         │
│  This is called "Deref coercion" -      │
│  Rust knows String can act as &str.     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Deref Coercion



## Rust's Helpful Magic

Rust has a feature called **deref coercion**:

When you borrow a `String` with `&`, Rust

automatically converts it to `&str`.

```rust
fn wants_str(s: &str) { ... }

let owned = String::from("hello");
wants_str(&owned);  // &String → &str automatically!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  You write:    &owned  (type: &String)  │
│  Rust sees:    &str                     │
│                                         │
│  Rust applies deref coercion:           │
│  &String → &str                         │
│                                         │
│  This happens automatically when needed!│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Is This Cheap?



## No Copying!

Converting `&str` → `String` was EXPENSIVE:

- Allocate heap memory
- Copy all the bytes



Converting `String` → `&str` is CHEAP:

- Just create a pointer to existing data
- No allocation, no copying!



```
┌─────────────────────────────────────────┐
│                                         │
│  String (on heap):                      │
│  ┌───┬───┬───┬───┬───┐                  │
│  │ h │ e │ l │ l │ o │                  │
│  └───┴───┴───┴───┴───┘                  │
│    ↑                                    │
│    │                                    │
│  &str just points here!                 │
│  Same data, no copy.                    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Alternative: .as_str()



## More Explicit Method

You can also use the `.as_str()` method:

```rust
let my_string = String::from("hello");
let my_slice = my_string.as_str();
```



```
┌──────────────────────────────────────────┐
│                                          │
│  &my_string     vs     my_string.as_str()│
│                                          │
│  Both give you &str!                     │
│                                          │
│  .as_str() is more explicit:             │
│  "Give me this AS a &str"                │
│                                          │
│  & relies on deref coercion:             │ 
│  "Borrow this (Rust figures out &str)"   │
│                                          │
│  Both are valid. Use what reads best.    │
│                                          │
└──────────────────────────────────────────┘
```



--- slide ---

# The Borrowing Rules Apply



## You're Creating a Borrow

When you convert `String` to `&str`, you're

creating a BORROW of the String's data.

```rust
let owned = String::from("hello");
let borrowed = &owned;  // borrowed is &str

// Borrowing rules apply:
// - owned cannot be mutated while borrowed exists
// - borrowed cannot outlive owned
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Remember from ownership:               │
│                                         │
│  • Can't modify while borrowed          │
│  • Borrow must not outlive owner        │
│  • Can have multiple immutable borrows  │
│                                         │
│  The &str is a borrow of the String.    │
│  All borrowing rules apply!             │
│                                         │
└─────────────────────────────────────────┘

-
```



--- slide ---

# Passing to Functions



## The Common Pattern

```rust
fn is_greeting(text: &str) -> bool {
    text == "hello" || text == "hi"
}

fn main() {
    // With a String:
    let s = String::from("hello");
    is_greeting(&s);  // Pass &String, coerced to &str

    // With a literal:
    is_greeting("hello");  // Already &str
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The function takes &str, so:           │
│                                         │
│  • Literals work directly ("hello")     │
│  • Strings need & prefix (&my_string)   │
│                                         │
│  This is why &str is preferred for      │
│  function parameters - maximum flex!    │
│                                         │
└─────────────────────────────────────────┘

-
```



--- slide ---

# Summary: Both Directions



## The Complete Picture

```
┌─────────────────────────────────────────┐
│                                         │
│  &str → String (EXPENSIVE)              │
│                                         │
│    String::from(slice)                  │
│    slice.to_string()                    │
│    slice.to_owned()                     │
│                                         │
│    Creates new heap allocation.         │
│    Copies all bytes.                    │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  String → &str (CHEAP)                  │
│                                         │
│    &my_string                           │
│    my_string.as_str()                   │
│                                         │
│    Just creates a reference.            │
│    No allocation, no copying.           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Passing String Where &str Expected

Look at the code:

```rust
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // Don't change this!
    if is_a_color_word(word) { ... }  // ERROR!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  THE PROBLEM:                           │
│                                         │
│  is_a_color_word wants: &str            │
│  word is:               String          │
│                                         │
│  You need to pass a &str view           │
│  of your String to the function.        │
│                                         │
│  How do you borrow a String as &str?    │
│                                         │
└─────────────────────────────────────────┘

-
```



--- slide ---

# Think It Through



## Before You Type



```
┌─────────────────────────────────────────┐
│  ASK YOURSELF:                          │
│                                         │
│  1. What type does is_a_color_word()    │
│     expect? (Look at the parameter!)    │
│                                         │
│  2. What type is `word`?                │
│     (String::from gives you a String)   │
│                                         │
│  3. How do you convert String → &str?   │
│     • &word                             │
│     • word.as_str()                     │
│                                         │
│  4. Where do you need to make           │
│     the change? (The function call!)    │
│                                         │
│  5. Can you change the function?        │
│     (No! The TODO says not to.)         │
│                                         │
└─────────────────────────────────────────┘
```



The fix is small - just one character or method!

(Now go to the Editor and try it!)
