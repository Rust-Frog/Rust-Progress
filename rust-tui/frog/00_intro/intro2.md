# Hello, World!

The traditional first program in any programming language.

When learning a new language, it's tradition to write a little 
program that prints "Hello, world!" to the screen. This simple 
program proves that your environment is set up correctly and 
introduces you to the very basics of the language.

## Why Start Here?

The Hello World program is intentionally simple so you can 
focus on the mechanics of writing, compiling, and running 
code rather than solving a complex problem.

## What You'll Learn

In this exercise, you'll understand:

- How to define the `main` function (program entry point)
- How to use the `println!` macro to output text
- The difference between macros and functions in Rust
- Why Rust uses semicolons at the end of statements
- How Rust compiles your code before running it

--- slide ---

# The main Function

## Every Rust Program Starts Here

```
fn main() {
    // Your code goes inside these braces
}
```

The `main` function is special — it is ALWAYS the first code 
that runs in every executable Rust program. Without a `main` 
function, Rust wouldn't know where to begin execution.

## Breaking Down the Syntax

```
┌────────────────────────────────────────────────────┐
│                                                    │
│   fn main() {                                      │
│   ^^                                               │
│   └── "fn" keyword declares a new function         │
│                                                    │
│   fn main() {                                      │
│      ^^^^                                          │
│      └── "main" is the function name (required)    │
│                                                    │
│   fn main() {                                      │
│         ^^                                         │
│         └── parentheses hold parameters (none)     │
│                                                    │
│   fn main() {                                      │
│            ^                                       │
│            └── opening brace starts the body       │
│                                                    │
└────────────────────────────────────────────────────┘
```

## Style Note

Rust requires curly brackets `{}` around ALL function bodies.
The standard style is to place the opening brace on the same 
line as the function declaration, with one space before it.

--- slide ---

# The println! Macro

## The Star of Hello World

```
println!("Hello, world!");
```

This single line does all the work in our program. It prints 
text to the screen (specifically, to "standard output").

## Three Critical Details

Let's examine each part of this line:

```
┌────────────────────────────────────────────────────┐
│                                                    │
│  println!("Hello, world!");                        │
│  ^^^^^^^^                                          │
│  │                                                 │
│  └── DETAIL 1: The exclamation mark (!) tells us   │
│      this is a MACRO, not a regular function.      │
│      Macros can do things functions cannot.        │
│                                                    │
│  println!("Hello, world!");                        │
│           ^^^^^^^^^^^^^^^                          │
│           │                                        │
│      DETAIL 2: The text in double quotes is a      │
│      STRING LITERAL. This is the text that will    │
│      be printed to the screen.                     │
│                                                    │
│  println!("Hello, world!");                        │
│                           ^                        │
│                           │                        │
│      DETAIL 3: The SEMICOLON marks the end of      │
│      this statement. Most lines of Rust code       │
│      end with a semicolon.                         │
│                                                    │
└────────────────────────────────────────────────────┘
```

--- slide ---

# Macros vs Functions

## What Makes Macros Different?

In Rust, `println!` is a macro, not a function. The `!` at 
the end is how you can always tell the difference:

```
┌─────────────────────┬───────────────────────────────┐
│ MACROS (have !)     │ FUNCTIONS (no !)              │
├─────────────────────┼───────────────────────────────┤
│ println!("text")    │ do_something("text")          │
│ vec![1, 2, 3]       │ Vec::new()                    │
│ format!("{}", x)    │ String::from("hello")         │
│ panic!("error!")    │ std::process::exit(1)         │
└─────────────────────┴───────────────────────────────┘
```

## Why Use Macros?

Macros are more powerful than functions because they:

- Can accept a variable number of arguments
- Execute at compile time, not runtime
- Can generate repetitive code automatically
- Don't always follow the same rules as functions

For now, just remember: when you see `!`, you're calling 
a macro. We'll dive deep into macros in Chapter 20.

--- slide ---

# String Literals

## Text in Double Quotes

When you write text inside double quotes, you create what 
Rust calls a "string literal":

```
"Hello, world!"
```

This is a fixed piece of text embedded directly in your 
program. The compiler knows exactly what characters are 
in this string at compile time.

## Escape Sequences

What if you need special characters in your string?

```
┌─────────────┬──────────────────────────────────────┐
│ Escape      │ What It Produces                     │
├─────────────┼──────────────────────────────────────┤
│ \n          │ New line (moves to next line)        │
│ \t          │ Tab character                        │
│ \\          │ A literal backslash                  │
│ \"          │ A literal double quote               │
│ \r          │ Carriage return                      │
└─────────────┴──────────────────────────────────────┘
```

Example:

```
println!("Line 1\nLine 2");
// Output:
// Line 1
// Line 2
```

--- slide ---

# Rust is a Compiled Language

## What Does Compilation Mean?

Before you can run a Rust program, you must COMPILE it.
The Rust compiler (called `rustc`) transforms your human-
readable source code into machine code the computer can 
execute directly.

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│  SOURCE CODE          COMPILER         EXECUTABLE   │
│                                                     │
│  ┌──────────┐        ┌───────┐        ┌──────────┐  │
│  │ main.rs  │  ───>  │ rustc │  ───>  │  main    │  │
│  │ (text)   │        │       │        │ (binary) │  │
│  └──────────┘        └───────┘        └──────────┘  │
│                                                     │
│  You write this      Checks for      You can run    │
│  in your editor      errors and      this file!     │
│                      builds binary                  │
│                                                     │
└─────────────────────────────────────────────────────┘
```

## Why Compilation Matters

Unlike Python or JavaScript (which are interpreted), 
compiled languages like Rust have these advantages:

- FASTER execution — no interpreter overhead
- PORTABLE — give the executable to anyone, no Rust needed
- SAFER — many errors caught at compile time, not runtime

In this TUI, we compile automatically when you save!

--- slide ---

# Semicolons in Rust

## Statements End with Semicolons

Almost every line of Rust code ends with a semicolon (;).
The semicolon tells Rust that this statement is complete 
and the next one can begin.

```
println!("First line");   // semicolon ends statement
println!("Second line");  // semicolon ends statement
let x = 5;                // semicolon ends statement
```

## When You Forget a Semicolon

If you forget a semicolon, the compiler will usually tell 
you with a helpful error message:

```
error: expected `;`
 --> main.rs:2:30
  |
2 |     println!("Hello, world!")
  |                              ^ help: add `;` here
```

There ARE cases where you intentionally omit semicolons 
(like the last expression in a function for return values),
but we'll cover that later. For now: when in doubt, add one!

--- slide ---

# Your Task

## What to Do

Look at the code in the Editor panel on the left.

The TODO comment tells you to make the program print 
"Hello world!" (note: no comma in this version).

## Checklist

Before moving on, verify:

```
┌────────────────────────────────────────────────────┐
│                                                    │
│  [ ] Does the string say exactly "Hello world!"?   │
│  [ ] Is there a semicolon at the end of the line?  │
│  [ ] Does the Output panel show success?           │
│  [ ] Have you removed "// I AM NOT DONE" comment?  │
│                                                    │
└────────────────────────────────────────────────────┘
```

## Troubleshooting

If you see errors in the Output panel:
- Read the error message carefully
- Check line numbers mentioned in the error
- Look for typos in "Hello world!"

## Next Steps

Once the code compiles and runs successfully:
- Press `]` to advance to the next exercise
- Or press `[` to go back to the previous one

Congratulations on your first Rust program!
