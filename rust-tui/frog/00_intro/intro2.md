# 🌍 Hello, World!

The traditional first program in any programming language.

When learning a new language, it's tradition to write a little program 
that prints "Hello, world!" to the screen. This simple program proves 
your environment is set up correctly and introduces you to the basics.

## What You'll Learn

- How to define the `main` function (program entry point)
- How to use the `println!` macro to output text  
- The difference between macros and functions in Rust
- Why Rust uses semicolons at the end of statements
- How Rust compiles your code before running it

--- slide ---

# 📌 The main Function

Every Rust program starts here:

```rust
fn main() {
    // Your code goes inside these braces
}
```

The `main` function is special — it is ALWAYS the first code that runs 
in every executable Rust program.

## Breaking Down the Syntax

```
   fn main() {
   ^^
   └── "fn" keyword declares a new function

   fn main() {
      ^^^^
      └── "main" is the function name (required!)

   fn main() {
         ^^
         └── parentheses hold parameters (none here)

   fn main() {
            ^
            └── opening brace starts the function body
```

## Style Note

Rust requires curly brackets `{}` around ALL function bodies.
The standard style is to place the opening brace on the same 
line as the function declaration, with one space before it.

```
   ✅ CORRECT:     fn main() { ... }
                        ^
                   space before brace

   ❌ WRONG:       fn main()
                   { ... }
```

--- slide ---

# ⭐ The println! Macro

The star of Hello World:

```rust
println!("Hello, world!");
```

This single line does all the work — it prints text to the screen!

## Three Critical Details

```
   println!("Hello, world!");
   ^^^^^^^^
   │
   └── DETAIL 1: The exclamation mark (!)
       This tells us it's a MACRO, not a function.
       Macros can do things functions cannot!


   println!("Hello, world!");
           ^^^^^^^^^^^^^^^
           │
           └── DETAIL 2: String literal
               The text in double quotes is what gets printed.


   println!("Hello, world!");
                           ^
                           │
                           └── DETAIL 3: Semicolon
                               Marks the END of this statement.
```

--- slide ---

# 🔮 Macros vs Functions

In Rust, `println!` is a MACRO, not a function.
The `!` at the end is how you can ALWAYS tell the difference:

```
   🔷 MACROS (have !)          🔶 FUNCTIONS (no !)
   ─────────────────           ───────────────────
   println!("text")            do_something("text")
   vec![1, 2, 3]               Vec::new()
   format!("{}", x)            String::from("hello")
   panic!("error!")            std::process::exit(1)
```

## Why Use Macros?

Macros are more powerful than functions because they:

- ✨ Can accept a VARIABLE number of arguments
- ⚡ Execute at COMPILE TIME, not runtime
- 🔄 Can generate repetitive code automatically
- 🎯 Don't always follow the same rules as functions

💡 For now, just remember: when you see `!`, you're calling a macro.
   We'll dive deep into macros in Chapter 20!

--- slide ---

# 📜 String Literals

When you write text inside double quotes, you create a "string literal":

```rust
"Hello, world!"
```

This is a FIXED piece of text embedded directly in your program.
The compiler knows exactly what characters are in this string 
at compile time.

## Escape Sequences

What if you need special characters in your string?

```
   ESCAPE     WHAT IT PRODUCES
   ──────     ────────────────────────────
     \n       New line (moves to next line)
     \t       Tab character
     \\       A literal backslash
     \"       A literal double quote
     \r       Carriage return
```

## Example

```rust
println!("Line 1\nLine 2");

// Output:
// Line 1
// Line 2
```

--- slide ---

# ⚙️ Rust is a Compiled Language

Before you can run a Rust program, you must COMPILE it.

The Rust compiler (`rustc`) transforms your human-readable source code 
into machine code the computer can execute directly:

```
   SOURCE CODE          COMPILER           EXECUTABLE

   ┌──────────┐        ┌──────────┐        ┌──────────┐
   │ main.rs  │  ───▶  │  rustc   │  ───▶  │   main   │
   │  (text)  │        │          │        │ (binary) │
   └──────────┘        └──────────┘        └──────────┘

   You write           Checks for          You can run
   this in your        errors and          this file!
   editor              builds binary
```

## Why Compilation Matters

Unlike Python or JavaScript (which are interpreted), compiled 
languages like Rust have these advantages:

- 🚀 **FASTER** — No interpreter overhead at runtime
- 📦 **PORTABLE** — Give the executable to anyone, no Rust needed  
- 🛡️ **SAFER** — Many errors caught at compile time, not runtime

💡 In this TUI, we compile AUTOMATICALLY when you save!

--- slide ---

# ❗ Semicolons in Rust

Almost every line of Rust code ends with a semicolon (`;`).

The semicolon tells Rust that this statement is complete 
and the next one can begin.

```rust
println!("First line");    // semicolon ends statement
println!("Second line");   // semicolon ends statement
let x = 5;                 // semicolon ends statement
```

## When You Forget a Semicolon

If you forget one, the compiler will tell you with a helpful error:

```
error: expected `;`
 --> main.rs:2:30
  |
2 |     println!("Hello, world!")
  |                              ^ help: add `;` here
```

## Exception

There ARE cases where you intentionally omit semicolons (like the 
last expression in a function for return values). We'll cover that 
later. For now: **when in doubt, add one!**

--- slide ---

# ✏️ Your Task

Look at the code in the Editor panel on the left.

The TODO comment tells you to make the program print 
`"Hello world!"` (note: no comma in this version).

## Checklist

Before moving on, verify:

- [ ] Does the string say exactly `"Hello world!"`?
- [ ] Is there a semicolon at the end of the line?
- [ ] Does the Output panel show success?
- [ ] Have you removed the `// I AM NOT DONE` comment?

## Troubleshooting

If you see errors in the Output panel:

- 📝 Read the error message carefully
- 🔢 Check line numbers mentioned in the error
- 🔍 Look for typos in `"Hello world!"`
- ❗ Make sure the semicolon is there

## Next Steps

Once the code compiles and runs successfully:

- Press `]` to advance to the next exercise
- Press `[` to go back to the previous one

---

🎉 **Congratulations on your first Rust program!** 🎉
