# Characters



## What is a Char?

A `char` represents a single character.



In Rust, `char` uses SINGLE quotes:

```rust
let c = 'A';      // char
let s = "A";      // &str (different!)
```



Single quotes = char

Double quotes = string



--- slide ---

# Char is Unicode



Rust chars are 4 bytes and can hold ANY Unicode character!



```rust
let letter = 'a';
let digit = '7';
let emoji = '😀';
let chinese = '中';
let symbol = '★';
```



All of these are valid chars!



## Not Just ASCII

Unlike some languages, Rust's `char` isn't limited 
to English letters and symbols.



--- slide ---

# Char Methods



Characters have useful methods:



```
┌────────────────────────────────────────────────┐
│                                                │
│  c.is_alphabetic()  Is it a letter? (a-z, A-Z) │
│  c.is_numeric()     Is it a digit? (0-9)       │
│  c.is_alphanumeric() Letter or digit?          │
│  c.is_uppercase()   Is it uppercase?           │
│  c.is_lowercase()   Is it lowercase?           │
│  c.is_whitespace()  Space, tab, newline?       │
│                                                │
└────────────────────────────────────────────────┘
```



These return a `bool` - useful with `if`!



--- slide ---

# Think About Your Exercise



The code has an example with `my_first_initial`.

You need to create YOUR character.



```
┌────────────────────────────────────────────────┐
│                                                │
│  QUESTIONS:                                    │
│                                                │
│  1. What quote marks do you use for a char?    │
│                                                │
│  2. Can you try different characters?          │
│     - A letter                                 │
│     - A digit (still in single quotes!)        │
│     - An emoji                                 │
│     - A symbol                                 │
│                                                │
│  3. What do you think the methods will         │
│     return for each type?                      │
│                                                │
└────────────────────────────────────────────────┘
```



Experiment! Try different characters and see 
what the program prints.



(Go try it in the Editor!)
