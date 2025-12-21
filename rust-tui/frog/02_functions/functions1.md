# Defining Functions

## The Problem

```
fn main() {
    call_me();  // <-- Where is call_me defined?
}
```

The function `call_me` doesn't exist yet!
 
exist! That's what you need to create.

--- slide ---

# What is a Function?

A function is a reusable block of code with a name.

## Why Functions?

```
┌────────────────────────────────────────────────┐
│  BENEFITS OF FUNCTIONS                         │
├────────────────────────────────────────────────┤
│  • Organize code into logical pieces           │
│  • Avoid repeating the same code               │
│  • Make code easier to read and maintain       │
│  • Give meaningful names to operations         │
└────────────────────────────────────────────────┘
```

## The `main` Function

Every Rust program starts with `main`:

```
fn main() {
    // Program starts here
}
```

`main` is just a function with a special name!

--- slide ---

# Function Syntax

## The Parts of a Function

```
fn function_name() {
^^  ^^^^^^^^^^^^
│   │
│   └── Name (use snake_case)
└── The `fn` keyword (required)

    // Body goes here

}   // Closing brace
```

## Naming Rules

```
fn call_me()        // ✓ snake_case (correct)
fn callMe()         // ✗ camelCase (wrong)
fn CallMe()         // ✗ PascalCase (wrong)
fn call-me()        // ✗ kebab-case (wrong)
```

## Empty Functions Are Valid

```
fn do_nothing() {
    // This is fine!
}
```

--- slide ---

# Understanding the Problem

Look at the error when you compile:

```
error[E0425]: cannot find function `call_me` 
              in this scope
```

## What This Means

```
┌────────────────────────────────────────────────┐
│  The code says: call_me()                      │
│                                                │
│  Rust says: "I don't know what 'call_me' is!" │
│                                                │
│  You need to tell Rust what 'call_me' means   │
│  by defining the function.                     │
└────────────────────────────────────────────────┘
```

## Think About It

```
┌────────────────────────────────────────────────┐
│  QUESTIONS TO CONSIDER:                        │
│                                                │
│  1. What keyword starts a function definition? │
│  2. What name should your function have?       │
│  3. Where can you define the function?         │
│     (Hint: before OR after main)               │
└────────────────────────────────────────────────┘
```

--- slide ---

# Key Takeaways

## Function Definition Template

```
fn name() {
    // body
}
```

## Location Doesn't Matter

In Rust, you can define functions before or after 
they're called (unlike some languages):

```
call_me();  // Called first

fn call_me() { }  // Defined after - OK!
```

## Remember

- Use the `fn` keyword
- Name must match exactly
- Use snake_case naming
- Even an empty body `{}` is valid
