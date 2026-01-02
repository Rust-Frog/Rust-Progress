# String or &str?



## The Ultimate Test

You've learned about `String` and `&str`.

Now: can you IDENTIFY which type an expression produces?



```
┌─────────────────────────────────────────┐
│                                         │
│  This exercise tests your understanding │
│  of the whole String/&str system.       │
│                                         │
│  Given an expression, you must know:    │
│    → Is it String? (owned)              │
│    → Is it &str? (borrowed)             │
│                                         │
│  This is essential Rust knowledge!      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Core Rule



## Where Does the Data Live?

```
┌─────────────────────────────────────────┐
│                                         │
│  &str = Data exists SOMEWHERE ELSE      │
│                                         │
│    • String literal in binary           │
│    • Borrowed from a String             │
│    • Slice of existing data             │
│                                         │
│  String = Data is NEWLY CREATED         │
│                                         │
│    • Fresh heap allocation              │
│    • Owns its own copy                  │
│    • Created by constructors/methods    │
│                                         │
└─────────────────────────────────────────┘
```



Ask yourself: "Was new memory allocated?"



--- slide ---

# String Literals



## Always &str

```rust
"hello"
"any text in quotes"
"blue"
```



```
┌─────────────────────────────────────────┐
│                                         │
│  String literals = &str                 │
│                                         │
│  The text is embedded in your program.  │
│  You're just borrowing a view of it.    │
│                                         │
│  No allocation happens.                 │
│  The data was always there.             │
│                                         │
│  Type: &'static str                     │
│  (lives for entire program)             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# String::from()



## Creates String

```rust
String::from("hello")
String::from("any text")
```



```
┌─────────────────────────────────────────┐
│                                         │
│  String::from() = String                │
│                                         │
│  You're explicitly asking:              │
│  "Create a String from this &str"       │
│                                         │
│  Allocates heap memory.                 │
│  Copies the bytes.                      │
│  You OWN the result.                    │
│                                         │
│  The name says it: String::from         │
│  Returns: String                        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# .to_string()



## Creates String

```rust
"hello".to_string()
some_var.to_string()
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .to_string() = String                  │
│                                         │
│  The name tells you:                    │
│  "Convert this TO a String"             │
│                                         │
│  Works on any type that implements      │
│  the Display trait.                     │
│                                         │
│  Always allocates.                      │
│  Always returns owned String.           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# .to_owned()



## Creates String (from &str)

```rust
"hello".to_owned()
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .to_owned() on &str = String           │
│                                         │
│  The name tells you:                    │
│  "Give me an OWNED version"             │
│                                         │
│  You have borrowed data (&str)          │
│  You want owned data (String)           │
│                                         │
│  Allocates and copies.                  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# .into()



## Context-Dependent Conversion

```rust
let s: String = "hello".into();
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .into() = depends on context!          │
│                                         │
│  .into() converts to whatever type      │
│  is EXPECTED by the context.            │
│                                         │
│  "hello".into() where String expected   │
│  → Creates a String                     │
│                                         │
│  In strings4, when passed to string():  │
│    string(arg: String)                  │
│  Rust knows it needs String.            │
│  "hello".into() becomes String.         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# format!()



## Always Creates String

```rust
format!("Hello {}", name)
format!("Value: {}", 42)
format!("Interpolation {}", "Station")
```



```
┌─────────────────────────────────────────┐
│                                         │
│  format!() = String                     │
│                                         │
│  It's like println! but returns         │
│  the formatted text as a String.        │
│                                         │
│  Always allocates new memory.           │
│  Always returns owned String.           │
│                                         │
│  Even format!("hello") with no          │
│  interpolation returns String!          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# String Slicing



## Returns &str

```rust
let s = String::from("hello");
let slice = &s[0..2];  // "he"
```



```
┌─────────────────────────────────────────┐
│                                         │
│  &String[start..end] = &str             │
│                                         │
│  You're taking a SLICE (view) of        │
│  the String's data.                     │
│                                         │
│  String: "h e l l o"                    │
│           0 1 2 3 4                     │
│                                         │
│  &s[0..2]: "h e"                        │
│  Just points to part of original!       │
│                                         │
│  No allocation. Returns &str.           │
│                                         │
│  NOTE: This is BYTE indexing!           │
│  Be careful with non-ASCII!             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# .trim()



## Returns &str

```rust
"  hello  ".trim()
my_string.trim()
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .trim() = &str                         │
│                                         │
│  Just adjusts the view window.          │
│  Points to subset of original data.     │
│                                         │
│  Original: "  hello  "                  │
│  Trimmed:    "hello"                    │
│              └─────┘ same bytes!        │
│                                         │
│  No allocation needed.                  │
│  Returns borrowed &str.                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# .replace()



## Returns String

```rust
"hello".replace("l", "L")
text.replace("old", "new")
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .replace() = String                    │
│                                         │
│  The result might be different size!    │
│                                         │
│  "cat".replace("cat", "elephant")       │
│  → "elephant"                           │
│                                         │
│  Can't just point to original data.     │
│  Must create new String with result.    │
│                                         │
│  Always allocates.                      │
│  Returns owned String.                  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# .to_lowercase() / .to_uppercase()



## Returns String

```rust
"HELLO".to_lowercase()   // "hello"
"hello".to_uppercase()   // "HELLO"
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Case conversion = String               │
│                                         │
│  Why can't this return &str?            │
│                                         │
│  Some characters change SIZE when       │
│  changing case (in Unicode)!            │
│                                         │
│  German ß → SS (1 char → 2 chars)       │
│                                         │
│  Result might be different length.      │
│  Must allocate new String.              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Quick Reference



## The Complete Picture

```
┌─────────────────────────────────────────┐
│                                         │
│  PRODUCES &str:                         │
│                                         │
│    "literal"          (string literal)  │
│    &string[0..n]      (slicing)         │
│    string.trim()      (trimming)        │
│    string.as_str()    (explicit view)   │
│    &my_string         (borrowing)       │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  PRODUCES String:                       │
│                                         │
│    String::from(...)  (constructor)     │
│    "...".to_string()  (conversion)      │
│    "...".to_owned()   (ownership)       │
│    "...".into()       (conversion)      │
│    format!(...)       (formatting)      │
│    .replace(...)      (replacement)     │
│    .to_lowercase()    (case change)     │
│    .to_uppercase()    (case change)     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Decision Process



## How to Figure It Out

```
┌─────────────────────────────────────────┐
│                                         │
│  ASK THESE QUESTIONS:                   │
│                                         │
│  1. Is it a literal in quotes?          │
│     → &str                              │
│                                         │
│  2. Does the method/function name       │
│     suggest creating something new?     │
│     (to_string, from, format, replace)  │
│     → Probably String                   │
│                                         │
│  3. Is it just a VIEW of existing data? │
│     (trim, slice, borrow)               │
│     → Probably &str                     │
│                                         │
│  4. Could the result be different size  │
│     than input?                         │
│     → Must be String (can't be &str)    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Identify Each Type

You have two functions:

```rust
fn string_slice(arg: &str)   // Takes &str
fn string(arg: String)       // Takes String
```

And many expressions to classify:

```rust
placeholder("blue");
placeholder("red".to_string());
placeholder(String::from("hi"));
// ... and more
```



```
┌──────────────────────────────────────────┐
│                                          │
│  YOUR TASK:                              │
│                                          │
│  Replace each placeholder(...) with      │
│  either string_slice(...) or string(...) │
│                                          │
│  If the expression is &str:              │
│    → use string_slice()                  │
│                                          │
│  If the expression is String:            │
│    → use string()                        │
│                                          │
└──────────────────────────────────────────┘
```



--- slide ---

# Let's Analyze Each One



## Work Through the List

```
┌─────────────────────────────────────────┐
│                                         │
│  "blue"                                 │
│    → Literal in quotes = &str           │
│                                         │
│  "red".to_string()                      │
│    → .to_string() creates String        │
│                                         │
│  String::from("hi")                     │
│    → Constructor creates String         │
│                                         │
│  "rust is fun!".to_owned()              │
│    → .to_owned() creates String         │
│                                         │
│  "nice weather".into()                  │
│    → Context expects String = String    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# More Analysis



## The Remaining Expressions

```
┌─────────────────────────────────────────┐
│                                         │
│  format!("Interpolation {}", "Station") │
│    → format! always returns String      │
│                                         │
│  &String::from("abc")[0..1]             │
│    → Slicing a String gives &str        │
│                                         │
│  "  hello there ".trim()                │
│    → trim() returns &str                │
│                                         │
│  "Happy Monday!".replace("Mon", "Tues") │
│    → replace() returns String           │
│                                         │
│  "mY sHiFt KeY".to_lowercase()          │
│    → case conversion returns String     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Think It Through



## Before You Type



```
┌─────────────────────────────────────────┐
│  FOR EACH EXPRESSION, ASK:              │
│                                         │
│  1. Is it a plain string literal?       │
│     "text" without any method calls     │
│     → &str → string_slice()             │
│                                         │
│  2. Does it call a String-creating      │
│     method?                             │
│     .to_string(), .to_owned(), .into()  │
│     String::from(), format!()           │
│     .replace(), .to_lowercase()         │
│     → String → string()                 │
│                                         │
│  3. Does it call a view-creating        │
│     operation?                          │
│     .trim(), &s[..], .as_str()          │
│     → &str → string_slice()             │
│                                         │
└─────────────────────────────────────────┘
```



Go through each line systematically!

(Now go to the Editor and try it!)
