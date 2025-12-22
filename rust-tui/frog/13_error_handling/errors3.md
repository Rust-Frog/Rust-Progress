# The ? Problem in main()



## Where Does the Error Go?

You learned: ? propagates errors to the caller.

But what about main()? There's no caller!

```rust
fn main() {
    let value = might_fail()?;  // ERROR!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  COMPILER ERROR:                        │
│                                         │
│  "the `?` operator can only be used     │
│   in a function that returns `Result`"  │
│                                         │
│  main() returns () by default.          │
│  ? needs somewhere to propagate to!     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why main() Is Special



## The Entry Point

```rust
fn main() {
    // This is where your program starts
    // Default return type: () (unit, nothing)
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Normal functions:                      │
│    → ? propagates error to caller       │
│    → Caller decides what to do          │
│                                         │
│  main():                                │
│    → No caller exists                   │
│    → It's the TOP of the call stack     │
│    → Error has nowhere to go!           │
│                                         │
│  The operating system calls main()      │
│  but doesn't expect a Result.           │
│                                         │
│  OR DOES IT?                            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# main() Can Return Result!



## The Secret

Rust allows main() to return Result:

```rust
fn main() -> Result<(), SomeError> {
    let value = might_fail()?;  // Now this works!
    println!("Got: {}", value);
    Ok(())
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  fn main() -> Result<(), ErrorType>     │
│              ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑     │
│              Add a return type!         │
│                                         │
│  Now main() returns Result:             │
│  • Ok(()) = program succeeded           │
│  • Err(e) = program failed              │
│                                         │
│  ? can now propagate errors!            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Unit Type ()



## What's Ok(())?

```rust
fn main() -> Result<(), Error> {
    // ...
    Ok(())  // Return success with no value
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  () is the "unit type"                  │
│                                         │
│  It means "no meaningful value".        │
│  Functions that don't return anything   │
│  actually return ().                    │
│                                         │
│  Ok(()) means:                          │
│  "Success! But no value to give you."   │
│                                         │
│  It's like returning void in other      │
│  languages, but wrapped in Result.      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# What Happens on Error?



## OS Behavior

When main() returns Err:

```
┌─────────────────────────────────────────┐
│                                         │
│  main() returns Ok(())                  │
│    → Program exits with code 0          │
│    → Operating system sees "success"    │
│                                         │
│  main() returns Err(e)                  │
│    → Error is printed (Debug format)    │
│    → Program exits with code 1          │
│    → Operating system sees "failure"    │
│                                         │
│  This is useful for command-line tools! │
│  Exit codes communicate success/failure │
│  to shells and scripts.                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Choosing the Error Type



## What Goes in Result<(), E>?

The error type must match what ? produces:

```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let n: i32 = "42".parse()?;  // Returns ParseIntError on fail
    Ok(())
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  The error type E in main()'s return    │
│  must be compatible with:               │
│                                         │
│  • Errors from ? operations in main     │
│  • Errors from functions you call       │
│                                         │
│  If you only use parse(), use ParseIntError│
│  If you use multiple error types, you   │
│  need a more general type (later!)      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Full Pattern



## Before and After

```rust
// BEFORE: Can't use ? in main
fn main() {
    let cost = total_cost("10")?;  // ERROR!
    println!("{}", cost);
}

// AFTER: main returns Result
fn main() -> Result<(), ParseIntError> {
    let cost = total_cost("10")?;  // Works!
    println!("{}", cost);
    Ok(())  // Must return Ok at the end
}
```



--- slide ---

# Don't Forget Ok(())



## The Success Return

When main() returns Result, you must

explicitly return Ok(()) for success:

```rust
fn main() -> Result<(), Error> {
    do_stuff()?;
    more_stuff()?;
    // ...

    Ok(())  // <-- Don't forget this!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Without Ok(()), you'd get:             │
│                                         │
│  "expected Result, found ()"            │
│                                         │
│  The function body implicitly returns ()│
│  but the signature says Result!         │
│                                         │
│  Ok(()) wraps () in a Result.           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Alternative: Handle in main



## Not Using ?

You can also handle errors explicitly:

```rust
fn main() {
    match total_cost("10") {
        Ok(cost) => println!("Cost: {}", cost),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Without ? in main, you can:            │
│                                         │
│  • match on the Result                  │
│  • Use if let                           │
│  • Use unwrap_or_else                   │
│                                         │
│  But if you want ?, main must return    │
│  Result. That's the tradeoff.           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Making main() Use ?

The code uses ? in main():

```rust
fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;
    // ...
}
```

But main() doesn't return Result!



```
┌─────────────────────────────────────────┐
│                                         │
│  COMPILER ERROR:                        │
│                                         │
│  You can't use ? in a function that     │
│  returns (). main() needs to return     │
│  Result to use ?.                       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# What to Change



## Two Modifications Needed

```
┌─────────────────────────────────────────┐
│                                         │
│  1. ADD RETURN TYPE TO main()           │
│                                         │
│     fn main() { }                       │
│           ↓                             │
│     fn main() -> Result<(), ErrorType>  │
│                                         │
│     What error type? Check what         │
│     total_cost returns!                 │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  2. ADD Ok(()) AT THE END               │
│                                         │
│     The function must return Result.    │
│     For success, return Ok(()).         │
│                                         │
│     Don't forget the import if needed!  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Finding the Error Type



## Look at total_cost

```rust
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError>
```



```
┌─────────────────────────────────────────┐
│                                         │
│  total_cost returns:                    │
│  Result<i32, ParseIntError>             │
│                   ↑                     │
│                   This is the error type│
│                                         │
│  When you use ? on total_cost():        │
│  • On success: get i32                  │
│  • On error: ParseIntError propagates   │
│                                         │
│  main() must be able to return          │
│  ParseIntError for ? to work!           │
│                                         │
│  main() -> Result<(), ParseIntError>    │
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
│  1. What error does the current code    │
│     produce? (Can't use ? because...)   │
│                                         │
│  2. What must main() return to use ?    │
│                                         │
│  3. What error type does total_cost     │
│     return? (Look at its signature!)    │
│                                         │
│  4. Is ParseIntError already imported?  │
│     (Check the use statement at top!)   │
│                                         │
│  5. After changing main's signature,    │
│     what must you return at the end?    │
│                                         │
│  6. How do you write "success with      │
│     no value" as a Result?              │
│                                         │
└─────────────────────────────────────────┘
```



Change signature, add Ok(()) at end!

(Now go to the Editor and try it!)
