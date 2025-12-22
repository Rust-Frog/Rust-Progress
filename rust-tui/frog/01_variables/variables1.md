# � What Are Variables?

                    A variable is a named container that holds a value.

                    Think of it like a labeled box:

```
                              ┌───────────────┐
                              │               │
                              │      42       │  ← The value inside
                              │               │
                              └───────────────┘
                                    age          ← The label (name)
```

                    Variables let you:
                    - Store data to use later
                    - Give meaningful names to values
                    - Change values as your program runs

--- slide ---

# 🏷️ Creating Variables in Rust

                    In Rust, you must DECLARE a variable before using it.

                    This tells Rust: "I want to create a new container 
                    with this name."

## The Declaration Keyword

                    Rust uses a special keyword to create variables.
                    This keyword means "create a new variable":

```
                    ─────────── name = value;
                        ↑
                    What keyword goes here?
```

                    Without this keyword, Rust doesn't know you're 
                    trying to create something new — it thinks you're 
                    referring to something that already exists!

--- slide ---

# � Reading the Error Message

                    Look at the error in the Output panel:

```
                    error[E0425]: cannot find value `x` in this scope
```

                    Rust is telling you:

```
                    "I looked everywhere in this scope (the current
                     code block), but I can't find anything named x!"
```

## Why This Happens

                    When you write just `x = 5`, Rust thinks you mean:
                    "Put 5 into an EXISTING variable called x"

                    But x doesn't exist yet! You never told Rust to 
                    create it.

--- slide ---

# 🧠 The Key Insight

                    There's a difference between:

```
                    CREATING a variable       vs      USING a variable
                    (declaring it)                    (after it exists)


                    ┌─────────────┐                   ┌─────────────┐
                    │   (empty)   │   "Make me!"      │      5      │
                    └─────────────┘                   └─────────────┘
                          x                                 x
                    First time                        Already exists
```

                    The first time you use a name, you need to DECLARE 
                    it. After that, you can refer to it by just its name.

--- slide ---

# 🔍 Comparing Languages

                    Different languages handle this differently:

```
                    Python:       x = 5       (creates automatically)
                    
                    JavaScript:   let x = 5   (explicit creation)
                    
                    Rust:         ??? x = 5   (explicit creation)
```

                    Rust is in the "explicit" camp — you must tell it 
                    when you're creating something new.

## Why Be Explicit?

                    - Catches typos immediately
                    - Makes code intentions clear
                    - Prevents accidental variable creation
                    - Compiler can help you more

--- slide ---

# ✏️ Your Task

                    Look at the code in the Editor panel.

                    The error says Rust "cannot find value x" because 
                    x was never declared.

## Hints

                    1. What keyword creates a new variable in Rust?
                    
                    2. Where should that keyword go in the code?
                    
                    3. Look at the error location — which line needs 
                       to change?

## When You've Fixed It

                    Save with `:w` and look at the Output panel.
                    
                    If you see the message print successfully, 
                    you've got it! Press `]` for the next exercise.
