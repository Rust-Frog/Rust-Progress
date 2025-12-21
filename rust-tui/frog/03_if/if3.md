# Mixed Types Problem



## The Problem

Look at your code in the Editor.

There's an `if` expression assigning to `identifier`.



The compiler is complaining about mismatched types.

Different branches are returning different types!



This is a deeper version of the lesson from if2:

**All branches must return the SAME type.**



--- slide ---

# Why Types Must Match



When you write:

```rust
let x = if condition {
    value_a
} else {
    value_b
};
```



Rust needs to know: "What type is `x`?"



If `value_a` is an integer and `value_b` is a string,
Rust can't decide. It refuses to compile.



```
┌────────────────────────────────────────────────┐
│                                                │
│  ERROR: `if` and `else` have incompatible      │
│         types                                  │
│                                                │
│  One branch: integer                           │
│  Another branch: float                         │
│  Another branch: string                        │
│                                                │
│  Rust: "Pick ONE type for ALL branches!"       │
│                                                │
└────────────────────────────────────────────────┘
```



--- slide ---

# Reading the Code



This exercise has TWO `if` expressions.



The FIRST `if` creates `identifier`.

The SECOND `if` uses `identifier`.



```
┌────────────────────────────────────────────────┐
│                                                │
│  FIRST IF:                                     │
│  let identifier = if animal == "..." { ??? }   │
│                                                │
│  SECOND IF:                                    │
│  if identifier == 1 { ... }                    │
│  else if identifier == 2 { ... }               │
│  else if identifier == 3 { ... }               │
│                                                │
└────────────────────────────────────────────────┘
```



Look at how `identifier` is USED in the second `if`.

What is it compared to? Numbers or strings?



--- slide ---

# Detective Work



```
┌────────────────────────────────────────────────┐
│                                                │
│  CLUES:                                        │
│                                                │
│  1. The second `if` compares identifier to     │
│     numbers like 1, 2, 3...                    │
│                                                │
│  2. So identifier must be a NUMBER type        │
│                                                │
│  3. But in the first `if`, some branches       │
│     return things that AREN'T integers...      │
│                                                │
│  4. Find those branches. What are they         │
│     returning? What SHOULD they return?        │
│                                                │
└────────────────────────────────────────────────┘
```



Look carefully at each branch of the first `if`.

Which ones don't return integers?



--- slide ---

# Think About It



```
┌────────────────────────────────────────────────┐
│                                                │
│  QUESTIONS:                                    │
│                                                │
│  1. In the first if, how many branches are     │
│     there? List out what each returns.         │
│                                                │
│  2. What type does identifier need to be?      │
│     (Look at how the second if uses it)        │
│                                                │
│  3. Which branches return the WRONG type?      │
│                                                │
│  4. What integer values should those branches  │
│     return instead? (The second if tells you!) │
│                                                │
└────────────────────────────────────────────────┘
```



Don't change the second `if` - it's correct.

Fix the first `if` so all branches match.



(Go try it in the Editor!)
