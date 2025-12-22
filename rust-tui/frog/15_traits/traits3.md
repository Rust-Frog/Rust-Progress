# Default Trait Implementations



## The Repetition Problem

When multiple types share the same behavior:

```rust
trait Licensed {
    fn licensing_info(&self) -> String;
}

impl Licensed for SomeSoftware {
    fn licensing_info(&self) -> String {
        String::from("Default license")
    }
}

impl Licensed for OtherSoftware {
    fn licensing_info(&self) -> String {
        String::from("Default license")  // Same code!
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  IDENTICAL implementations!             │
│                                         │
│  Copy-pasting the same code is:         │
│  • Error-prone                          │
│  • Hard to maintain                     │
│  • Violates DRY (Don't Repeat Yourself) │
│                                         │
│  What if we want the SAME default       │
│  behavior for most types?               │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Default Implementations



## Providing a Body in the Trait

Traits can provide default method bodies:

```rust
trait Licensed {
    fn licensing_info(&self) -> String {
        String::from("Default license")
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  BEFORE (no default):                   │
│  fn licensing_info(&self) -> String;    │
│                                    ↑    │
│                                    Semicolon│
│                                         │
│  AFTER (with default):                  │
│  fn licensing_info(&self) -> String {   │
│      String::from("Default license")    │
│  }                                      │
│  ↑                                      │
│  Actual body with braces!               │
│                                         │
│  Now implementors get this for FREE!    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Using Default Implementations



## Empty impl Blocks

```rust
trait Licensed {
    fn licensing_info(&self) -> String {
        String::from("Default license")
    }
}

impl Licensed for SomeSoftware {}   // Uses default!
impl Licensed for OtherSoftware {}  // Uses default!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  impl Licensed for SomeSoftware {}      │
│                                   ↑↑    │
│                                   Empty!│
│                                         │
│  An EMPTY impl block means:             │
│  "Use all the default implementations"  │
│                                         │
│  Both types now have licensing_info()   │
│  that returns "Default license"         │
│  WITHOUT writing any code!              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Overriding Defaults



## Customizing When Needed

```rust
trait Licensed {
    fn licensing_info(&self) -> String {
        String::from("Default license")
    }
}

impl Licensed for SomeSoftware {}  // Uses default

impl Licensed for SpecialSoftware {
    fn licensing_info(&self) -> String {
        String::from("Special proprietary license")
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  You can OVERRIDE the default:          │
│                                         │
│  • SomeSoftware → "Default license"     │
│  • SpecialSoftware → Custom message     │
│                                         │
│  Defaults are optional, not mandatory!  │
│                                         │
│  Override when you need different       │
│  behavior for specific types.           │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Mixed Traits



## Some Methods Default, Some Required

```rust
trait Describable {
    // Required - no default (must implement)
    fn name(&self) -> String;

    // Default - has body (optional to implement)
    fn description(&self) -> String {
        format!("This is a {}", self.name())
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Trait methods can be:                  │
│                                         │
│  REQUIRED (no body):                    │
│  fn name(&self) -> String;              │
│  → MUST be implemented                  │
│                                         │
│  DEFAULT (has body):                    │
│  fn description(&self) -> String { ... }│
│  → CAN be implemented (optional)        │
│                                         │
│  Default can use required methods!      │
│  (They're guaranteed to exist)          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# String from &str



## Creating the Return Value

```rust
// String literal is &str
"Default license"  // type: &str

// Convert to String
String::from("Default license")  // type: String
"Default license".to_string()    // type: String
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Return type is String, not &str!       │
│                                         │
│  fn licensing_info(&self) -> String     │
│                              ↑          │
│                              Must return│
│                              owned String│
│                                         │
│  "text" is &str (borrowed)              │
│  String::from("text") is String (owned) │
│                                         │
│  The function must create and return    │
│  an owned String.                       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Adding a Default Implementation

You have:

```rust
trait Licensed {
    // TODO: Add a default implementation for `licensing_info`
    fn licensing_info(&self) -> String;
}

impl Licensed for SomeSoftware {}   // Don't edit
impl Licensed for OtherSoftware {}  // Don't edit
```



```
┌─────────────────────────────────────────┐
│                                         │
│  CURRENT STATE:                         │
│                                         │
│  The trait method has NO body           │
│  (ends with semicolon)                  │
│                                         │
│  The impl blocks are EMPTY              │
│  (they can't compile without default!)  │
│                                         │
│  TASK:                                  │
│                                         │
│  Add a default body to the trait method │
│  that returns "Default license"         │
│                                         │
│  Then empty impls will work!            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Transformation



## What to Change

```
┌─────────────────────────────────────────┐
│                                         │
│  BEFORE:                                │
│                                         │
│  trait Licensed {                       │
│      fn licensing_info(&self) -> String;│
│  }                                 ↑    │
│                               Semicolon │
│                                         │
│  AFTER:                                 │
│                                         │
│  trait Licensed {                       │
│      fn licensing_info(&self) -> String {│
│          String::from("Default license")│
│      }                                  │
│  }                                      │
│     ↑                                   │
│     Body with braces!                   │
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
│  1. What makes a trait method "required"│
│     vs "default"?                       │
│     (Semicolon vs body with braces)     │
│                                         │
│  2. What return type does the method    │
│     have?                               │
│     (String)                            │
│                                         │
│  3. What string should be returned?     │
│     (Check the test for exact text!)    │
│                                         │
│  4. How do you create a String from     │
│     a string literal?                   │
│     (String::from(...) or .to_string()) │
│                                         │
│  5. Where do you add the default body?  │
│     (In the trait definition itself!)   │
│                                         │
│  6. Do you change the impl blocks?      │
│     (No - the comment says don't edit!) │
│                                         │
└─────────────────────────────────────────┘
```



Replace the semicolon with a body!

(Now go to the Editor and try it!)
