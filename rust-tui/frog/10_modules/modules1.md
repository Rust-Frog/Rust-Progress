# Organizing Code With Modules



## As Programs Grow...

Small programs can live in one file.

But real projects have thousands of lines!



```
┌─────────────────────────────────────────┐
│                                         │
│  WITHOUT organization:                  │
│                                         │
│  • Functions everywhere                 │
│  • Name collisions ("which `process`?") │
│  • Hard to find anything                │
│  • Can't hide implementation details    │
│  • Everything is exposed                │
│                                         │
│  WITH modules:                          │
│                                         │
│  • Logical groupings                    │
│  • Namespaced names                     │
│  • Easy to navigate                     │
│  • Control what's public/private        │
│  • Clean APIs                           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# What Is a Module?



## A Container for Code

A module is a namespace that contains:

- Functions
- Structs
- Enums
- Constants
- Other modules (nesting!)



```rust
mod my_module {
    fn some_function() { }
    struct SomeStruct { }
    const SOME_CONST: i32 = 42;
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  mod module_name {                      │
│      // stuff lives here                │
│  }                                      │
│                                         │
│  Everything inside is GROUPED together  │
│  under the module's name.               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Accessing Module Contents



## The :: Path Syntax

To access something inside a module, use `::`:

```rust
mod math {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    math::add(2, 3);  // module::function
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  module_name::item_name                 │
│             ↑↑                          │
│             Path separator              │
│                                         │
│  You've seen this before!               │
│    String::from()                       │
│    Vec::new()                           │
│    Option::Some                         │
│                                         │
│  These are all module paths!            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Privacy Problem



## Try Running This...

```rust
mod math {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    math::add(2, 3);  // ERROR!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  COMPILER ERROR:                        │
│                                         │
│  function `add` is private              │
│                                         │
│  Wait, what? We defined it!             │
│  Why can't we use it?                   │
│                                         │
└─────────────────────────────────────────┘
```



This is Rust's **privacy system** at work.



--- slide ---

# Private by Default



## Rust's Philosophy

In Rust, everything inside a module is

**PRIVATE by default**.



```
┌─────────────────────────────────────────┐
│                                         │
│  PRIVATE means:                         │
│                                         │
│  • Can be used INSIDE the module        │
│  • CANNOT be used from OUTSIDE          │
│                                         │
│  mod secret {                           │
│      fn hidden() { }  // private!       │
│  }                                      │
│                                         │
│  // Outside the module:                 │
│  secret::hidden();  // ERROR! Private!  │
│                                         │
│  // Inside the module:                  │
│  // hidden() works fine                 │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why Private by Default?



## Encapsulation

This is a FEATURE, not a bug!



```
┌─────────────────────────────────────────┐
│                                         │
│  Benefits of privacy:                   │
│                                         │
│  1. HIDE implementation details         │
│     Users don't need to know HOW        │
│     something works internally.         │
│                                         │
│  2. PROTECT invariants                  │
│     Internal state can't be corrupted   │
│     by outside code.                    │
│                                         │
│  3. FREEDOM to change                   │
│     You can refactor internals without  │
│     breaking code that uses the module. │
│                                         │
│  4. CLEANER APIs                        │
│     Only expose what users need.        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The pub Keyword



## Making Things Public

To allow access from outside, use `pub`:

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    math::add(2, 3);  // Works now!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  pub fn add(...)                        │
│  ↑↑↑                                    │
│  Makes this function PUBLIC             │
│                                         │
│  "pub" = "public"                       │
│  Anyone can access this!                │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# pub Works on Many Things



## Not Just Functions

You can make many items public:

```rust
mod my_module {
    pub fn public_function() { }
    pub struct PublicStruct { }
    pub enum PublicEnum { A, B }
    pub const PUBLIC_CONST: i32 = 42;

    fn private_function() { }      // no pub = private
    struct PrivateStruct { }       // no pub = private
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  pub fn ...       public function       │
│  pub struct ...   public struct         │
│  pub enum ...     public enum           │
│  pub const ...    public constant       │
│  pub mod ...      public nested module  │
│                                         │
│  No pub?          Private!              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Mixing Public and Private



## The Common Pattern

Modules often have BOTH:

```rust
mod sausage_factory {
    // Private - internal secret!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // Public - the API
    pub fn make_sausage() {
        get_secret_recipe();  // Can use private internally!
        println!("sausage!");
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  From OUTSIDE:                          │
│    sausage_factory::make_sausage() ✓    │
│    sausage_factory::get_secret_recipe() ✗│
│                                         │
│  From INSIDE:                           │
│    make_sausage() ✓                     │
│    get_secret_recipe() ✓                │
│                                         │
│  Internal code can use everything.      │
│  External code only sees pub items.     │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Real World Analogy



## The Restaurant Kitchen

```
┌─────────────────────────────────────────┐
│                                         │
│  🍽️  RESTAURANT (module)                │
│                                         │
│  PUBLIC (customers can access):         │
│    • order_food()                       │
│    • view_menu()                        │
│    • pay_bill()                         │
│                                         │
│  PRIVATE (kitchen secrets):             │
│    • secret_sauce_recipe()              │
│    • supplier_contacts()                │
│    • profit_margins()                   │
│                                         │
│  Customers interact through the         │
│  PUBLIC interface.                      │
│                                         │
│  They don't need (and shouldn't have)   │
│  access to internal operations.         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Visibility Rules Summary



## The Complete Picture

```
┌─────────────────────────────────────────┐
│                                         │
│  INSIDE a module:                       │
│    • Can access ALL items (pub or not)  │
│    • Everything is visible internally   │
│                                         │
│  OUTSIDE a module:                      │
│    • Can ONLY access pub items          │
│    • Private items are invisible        │
│                                         │
│  CHILD modules:                         │
│    • Can see parent's private items     │
│    • Parent can't see child's private   │
│      (unless child makes them pub)      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Making a Function Accessible

Look at the code:

```rust
mod sausage_factory {
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();  // ERROR!
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  THE PROBLEM:                           │
│                                         │
│  main() tries to call make_sausage()    │
│  but it's PRIVATE (no pub keyword).     │
│                                         │
│  main() is OUTSIDE the module.          │
│  It can only access PUBLIC items.       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# What Should Be Public?



## Think About the Design

```
┌─────────────────────────────────────────┐
│                                         │
│  get_secret_recipe()                    │
│    Should this be public?               │
│    The comment says "Don't let anybody  │
│    outside see this!"                   │
│    → Keep it PRIVATE                    │
│                                         │
│  make_sausage()                         │
│    This is what main() wants to call.   │
│    This is the module's API.            │
│    → Make it PUBLIC                     │
│                                         │
│  The recipe stays hidden.               │
│  Only the sausage-making is exposed.    │
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
│  1. What error does the compiler give?  │
│     (Something about "private")         │
│                                         │
│  2. Which function does main() need     │
│     to access?                          │
│     (Look at the function call!)        │
│                                         │
│  3. What keyword makes things public?   │
│     (Three letters!)                    │
│                                         │
│  4. Should get_secret_recipe() be       │
│     public too?                         │
│     (Read the comment in the code!)     │
│                                         │
│  5. Where exactly do you add the        │
│     keyword?                            │
│     (Before the fn keyword!)            │
│                                         │
└─────────────────────────────────────────┘
```



One small addition fixes the problem!

(Now go to the Editor and try it!)
