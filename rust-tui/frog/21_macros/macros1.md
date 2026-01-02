# Macros: Code That Writes Code



## What Are Macros?

Macros are code that GENERATES other code.

They run at COMPILE time, not runtime!



```
┌─────────────────────────────────────────┐
│                                         │
│  Functions:                             │
│  • Called at runtime                    │
│  • Take values as arguments             │
│  • Return values                        │
│                                         │
│  Macros:                                │
│  • Expanded at compile time             │
│  • Take CODE as arguments               │
│  • Generate more code                   │
│                                         │
│  println!("hi") is a macro!             │
│  vec![1, 2, 3] is a macro!              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Calling Macros



## The Exclamation Mark

Macros are called with a `!` after their name:

```rust
// Macro calls (with !)
println!("Hello");
vec![1, 2, 3];
format!("{}", x);

// Function calls (no !)
String::from("hello");
my_function();
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The ! tells Rust:                      │
│  "This is a MACRO, not a function!"    │
│                                         │
│  my_macro!()  ← Macro call              │
│  my_func()    ← Function call           │
│                                         │
│  Without the !, Rust looks for a        │
│  function named my_macro and fails!     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Defining Simple Macros



## macro_rules!

The `macro_rules!` keyword creates a macro:

```rust
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

// Now call it:
my_macro!();  // Prints: Check out my macro!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  macro_rules! name {                    │
│      (pattern) => {                     │
│          // code to generate            │
│      };                                 │
│  }                                      │
│                                         │
│  () means "match empty input"           │
│  The code inside => { } is what gets    │
│  inserted when the macro is called.     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Macro Invocation Syntax



## Three Ways to Call

Macros can be called with different brackets:

```rust
my_macro!();       // Parentheses
my_macro![];       // Square brackets
my_macro!{};       // Curly braces
```

All three work! Convention:
- `()` for function-like macros
- `[]` for array-like (vec![])
- `{}` for block-like



```
┌─────────────────────────────────────────┐
│                                         │
│  These are all equivalent:              │
│                                         │
│  println!("hi");                        │
│  println!["hi"];                        │
│  println!{"hi"};                        │
│                                         │
│  But () is conventional for println!    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Missing the Bang!

Look at this code:

```rust
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro();  // ← Something's wrong!
}
```

Rust thinks `my_macro` is a FUNCTION!



--- slide ---

# The Fix



## Add the !

```
┌─────────────────────────────────────────┐
│                                         │
│  Current:  my_macro()                   │
│                     ↑                   │
│            Rust looks for a FUNCTION    │
│            named my_macro - not found!  │
│                                         │
│  Fix:      my_macro!()                  │
│                    ↑                    │
│            The ! tells Rust it's a      │
│            MACRO, not a function!       │
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
│  1. What character distinguishes a      │
│     macro call from a function call?    │
│     (Hint: !)                           │
│                                         │
│  2. Where does this character go?       │
│     (After the macro name, before the   │
│     parentheses)                        │
│                                         │
│  3. Why does Rust require this?         │
│     (Macros and functions are processed │
│     differently by the compiler)        │
│                                         │
└─────────────────────────────────────────┘
```



Macros need their bang!

(Go try it in the Editor!)
