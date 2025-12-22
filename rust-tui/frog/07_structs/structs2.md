# The Copy Problem



## Structs Can Get Tedious

Imagine you have a big struct with many fields:

```rust
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}
```



That's 7 fields!



--- slide ---

# Creating Similar Instances



## What If Most Values Are The Same?

Let's say you have a template order:

```rust
let template = Order {
    name: String::from("Bob"),
    year: 2019,
    made_by_phone: false,
    made_by_mobile: false,
    made_by_email: true,
    item_number: 123,
    count: 0,
};
```



Now you want YOUR order.

Only 2 things differ: the `name` and `count`.



--- slide ---

# The Painful Way



## Copying Everything Manually

You COULD write it all out again:

```rust
let my_order = Order {
    name: String::from("Alice"),  // different
    year: 2019,                   // same
    made_by_phone: false,         // same
    made_by_mobile: false,        // same
    made_by_email: true,          // same
    item_number: 123,             // same
    count: 5,                     // different
};
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Ugh. 7 lines of code.                  │
│  Only 2 lines are actually different.   │
│                                         │
│  This is:                               │
│    • Repetitive                         │
│    • Error-prone                        │
│    • Hard to maintain                   │
│                                         │
│  There MUST be a better way...          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Struct Update Syntax



## The Better Way

Rust has a shortcut: the `..` operator

```rust
let my_order = Order {
    name: String::from("Alice"),
    count: 5,
    ..template
};
```



```
┌─────────────────────────────────────────┐
│                                         │
│  What does ..template mean?             │
│                                         │
│  "For any fields I didn't specify,      │
│   copy them from 'template'"            │
│                                         │
│  It's like saying:                      │
│  "Same as template, EXCEPT for these"   │
│                                         │
└─────────────────────────────────────────┘
```



3 lines instead of 7!



--- slide ---

# The .. Operator



## Understanding the Syntax

Let's break it down:



```
┌─────────────────────────────────────────┐
│                                         │
│  Order {                                │
│      name: String::from("Alice"),       │
│      count: 5,                          │
│      ..template                         │
│  }   ↑↑                                 │
│      Two dots!                          │
│                                         │
│  The .. MUST come at the END            │
│  (after all explicitly set fields)      │
│                                         │
└─────────────────────────────────────────┘
```



The `..other_struct` fills in the blanks.



--- slide ---

# What Gets Copied?



## Only Missing Fields

The update syntax only copies fields

you DIDN'T explicitly set:

```rust
let my_order = Order {
    name: String::from("Alice"),  // I set this
    count: 5,                     // I set this
    ..template  // copies: year, made_by_phone,
                // made_by_mobile, made_by_email,
                // item_number
};
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Fields I set:   name, count            │
│  Fields copied:  everything else!       │
│                                         │
│  Your explicit values WIN.              │
│  The template fills in the rest.        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Visual Example



## Before and After

```
┌─────────────────────────────────────────┐
│                                         │
│  TEMPLATE ORDER:                        │
│  ┌───────────────────────────────────┐  │
│  │ name: "Bob"                       │  │
│  │ year: 2019                        │  │
│  │ made_by_phone: false              │  │
│  │ made_by_mobile: false             │  │
│  │ made_by_email: true               │  │
│  │ item_number: 123                  │  │
│  │ count: 0                          │  │
│  └───────────────────────────────────┘  │
│                                         │
│  YOUR ORDER (after update syntax):      │
│  ┌───────────────────────────────────┐  │
│  │ name: "Alice"         ← YOU SET   │  │
│  │ year: 2019            ← copied    │  │
│  │ made_by_phone: false  ← copied    │  │
│  │ made_by_mobile: false ← copied    │  │
│  │ made_by_email: true   ← copied    │  │
│  │ item_number: 123      ← copied    │  │
│  │ count: 5              ← YOU SET   │  │
│  └───────────────────────────────────┘  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Common Use Cases



## When Update Syntax Shines

```
┌─────────────────────────────────────────┐
│                                         │
│  📋 Default configurations              │
│     Start with defaults, override few   │
│                                         │
│  🧪 Testing with fixtures               │
│     Base test data + small changes      │
│                                         │
│  📝 Form data with prefilled values     │
│     Keep most fields, change what user  │
│     actually modified                   │
│                                         │
│  🎮 Game entities from templates        │
│     "Like this enemy, but stronger"     │
│                                         │
└─────────────────────────────────────────┘
```



Anytime you think "mostly the same, but..."

that's when `..` shines.



--- slide ---

# Important Rule



## Order Matters!

The `..source` MUST come last:

```rust
// CORRECT:
let order = Order {
    name: String::from("Alice"),
    count: 5,
    ..template  // at the end!
};

// WRONG:
let order = Order {
    ..template,  // can't be first!
    name: String::from("Alice"),
};
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Think of it as:                        │
│                                         │
│  "Here are my overrides...              │
│   and for the rest, use that one."      │
│                                         │
│  Overrides first, source last.          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Using Update Syntax

You're given a function that creates a template order.

Your task: create YOUR order by:

1. Changing just the fields that need to be different
2. Using `..` to copy everything else



```
┌─────────────────────────────────────────┐
│                                         │
│  HINTS:                                 │
│                                         │
│  • Look at the test assertions          │
│    What values should YOUR order have?  │
│                                         │
│  • Compare those to the template        │
│    What's DIFFERENT? What's the SAME?   │
│                                         │
│  • Only set the different fields        │
│    Let ..template handle the rest       │
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
│  1. What does the template contain?     │
│     (Look at create_order_template)     │
│                                         │
│  2. What does the test expect for       │
│     your_order.name? Is it the same     │
│     as the template's name?             │
│                                         │
│  3. What about your_order.count?        │
│     Same or different from template?    │
│                                         │
│  4. Which fields are "same as template" │
│     and can be copied with ..?          │
│                                         │
└─────────────────────────────────────────┘
```



The test assertions tell you exactly

what values you need!

(Now go to the Editor and try it!)
