# Calling Functions with Arguments

## The Problem

```
fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me();  // <-- Something's missing!
}
```

The function expects an argument, but gets nothing!


--- slide ---

# Parameters vs Arguments

## Definitions

```
┌────────────────────────────────────────────────┐
│  PARAMETER:                                    │
│  Variable in the function DEFINITION           │
│                                                │
│  fn greet(name: &str)  // 'name' is parameter  │
│           ^^^^                                 │
│                                                │
│  ARGUMENT:                                     │
│  Value passed when CALLING the function        │
│                                                │
│  greet("Alice");       // "Alice" is argument  │
│        ^^^^^^^                                 │
└────────────────────────────────────────────────┘
```

## The Connection

When you call a function, arguments fill in 
the parameters:

```
fn say(message: &str)  // Parameter: message
       ^^^^^^^
say("Hello");          // Argument: "Hello"
    ^^^^^^^
```

--- slide ---

# What Happens When You Call

## The Process

```
┌────────────────────────────────────────────────┐
│  1. You write: call_me(3)                      │
│                                                │
│  2. Rust sees: fn call_me(num: u8)             │
│                                                │
│  3. Rust assigns: num = 3                      │
│                                                │
│  4. Function runs with num = 3                 │
└────────────────────────────────────────────────┘
```

## If You Forget the Argument

```
call_me();  // ERROR!

error: this function takes 1 argument 
       but 0 arguments were supplied
```

--- slide ---

# Understanding the Problem

The function signature says:

```
fn call_me(num: u8)
           ^^^^^^^
           │
           └── Expects one u8 value
```

But the call says:

```
call_me();
        ^^
        │
        └── No argument provided!
```

## Think About It

```
┌────────────────────────────────────────────────┐
│  QUESTIONS TO CONSIDER:                        │
│                                                │
│  1. How many arguments does the function need? │
│                                                │
│  2. What type should that argument be?         │
│     (Look at the parameter: num: u8)           │
│                                                │
│  3. What value would make sense for "number   │
│     of times to ring"?                         │
└────────────────────────────────────────────────┘
```

--- slide ---

# Function Call Syntax

## Basic Pattern

```
function_name(argument);
              ^^^^^^^^
              │
              └── Value that fills the parameter
```

## With Multiple Arguments

```
full_name("Alice", "Smith");
          ^^^^^^^  ^^^^^^^
          │        │
          First    Second
```

## Type Matching

The argument type must match the parameter type:

```
fn double(x: i32) { ... }

double(5);      // OK: 5 is an integer
double("five"); // ERROR: expected i32, got &str
```

--- slide ---

# Key Takeaways

## Remember

```
┌────────────────────────────────────────────────┐
│  • Parameters define what a function expects   │
│  • Arguments provide actual values            │
│  • Count and types must match!                │
└────────────────────────────────────────────────┘
```

## The Error Tells You

```
this function takes 1 argument but 0 were supplied
                    ^              ^
                    │              │
                    Expected       Got
```

Read error messages carefully — they tell you 
exactly what's wrong!
