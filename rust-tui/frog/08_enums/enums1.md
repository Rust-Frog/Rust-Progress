# The Problem With "One Of Many"



## When Structs Aren't Enough

You learned structs: they bundle related data together.

But what if something can be ONE of several options?



```
┌─────────────────────────────────────────┐
│                                         │
│  Think about a TRAFFIC LIGHT:           │
│                                         │
│  It can be:                             │
│    • Red                                │
│    • Yellow                             │
│    • Green                              │
│                                         │
│  But NEVER two at once!                 │
│  It's always exactly ONE of these.      │
│                                         │
└─────────────────────────────────────────┘
```



How would you represent this with a struct?



--- slide ---

# The Struct Approach (Awkward)



## Let's Try It

```rust
struct TrafficLight {
    is_red: bool,
    is_yellow: bool,
    is_green: bool,
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  PROBLEMS WITH THIS:                    │
│                                         │
│  1. What if is_red AND is_green are     │
│     both true? Invalid state!           │
│                                         │
│  2. What if ALL are false?              │
│     Also invalid!                       │
│                                         │
│  3. You have to check three fields      │
│     just to know what color it is.      │
│                                         │
│  4. Nothing prevents impossible         │
│     combinations.                       │
│                                         │
└─────────────────────────────────────────┘
```



Structs say "I have ALL of these things."

We need "I am ONE of these things."



--- slide ---

# Enter: Enums



## The Right Tool

An **enum** (enumeration) represents a value that

can be ONE of several possible **variants**.

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  This says:                             │
│                                         │
│  "A TrafficLight is EITHER:             │
│     Red, OR Yellow, OR Green.           │
│   Never multiple. Never none.           │
│   Exactly one."                         │
│                                         │
│  Invalid states are IMPOSSIBLE!         │
│                                         │
└─────────────────────────────────────────┘
```



The compiler GUARANTEES valid states.



--- slide ---

# Struct vs Enum



## A Mental Model

```
┌─────────────────────────────────────────┐
│                                         │
│  STRUCT = "AND"                         │
│                                         │
│    A Person has:                        │
│      name AND age AND email             │
│                                         │
│    All fields exist together.           │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  ENUM = "OR"                            │
│                                         │
│    A TrafficLight is:                   │
│      Red OR Yellow OR Green             │
│                                         │
│    Only ONE variant at a time.          │
│                                         │
└─────────────────────────────────────────┘
```



This distinction is FUNDAMENTAL.

Structs bundle. Enums choose.



--- slide ---

# The Syntax



## Defining an Enum

```rust
enum Direction {
    North,
    South,
    East,
    West,
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  enum       ← keyword                   │
│  Direction  ← type name (PascalCase)    │
│  { }        ← curly braces              │
│                                         │
│  Inside:                                │
│    VariantName,                         │
│    VariantName,                         │
│    VariantName,                         │
│                                         │
│  Each variant is also PascalCase.       │
│  Separated by commas.                   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Creating Enum Values



## Using the :: Syntax

To create a value of an enum type, use `::`:

```rust
let light = TrafficLight::Red;
let heading = Direction::North;
```



```
┌─────────────────────────────────────────┐
│                                         │
│  TypeName::VariantName                  │
│          ↑↑                             │
│          Double colon!                  │
│                                         │
│  TrafficLight::Red                      │
│       ↑            ↑                    │
│       The type     The variant          │
│                                         │
│  This creates a TrafficLight value      │
│  that is specifically the Red variant.  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Why :: Instead of .?



## Namespacing

The `::` means "inside this namespace."

Variants live INSIDE their enum type.



```
┌─────────────────────────────────────────┐
│                                         │
│  Without namespacing:                   │
│                                         │
│    Red          ← Red what?             │
│    North        ← North what?           │
│                                         │
│  With namespacing:                      │
│                                         │
│    TrafficLight::Red    ← Ah, a light!  │
│    Direction::North     ← Ah, a heading!│
│                                         │
│  Two enums could both have a variant    │
│  called "None" - namespacing keeps      │
│  them separate.                         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Real World Examples



## Enums Are Everywhere

```
┌─────────────────────────────────────────┐
│                                         │
│  🎮 Game Actions                        │
│     enum Action { Move, Attack, Wait }  │
│                                         │
│  📱 App Screen                          │
│     enum Screen { Home, Settings, Help }│
│                                         │
│  📦 Order Status                        │
│     enum Status { Pending, Shipped,     │
│                   Delivered, Cancelled }│
│                                         │
│  🎵 Playback State                      │
│     enum Playback { Playing, Paused,    │
│                     Stopped }           │
│                                         │
│  ⌨️ Key Press                           │
│     enum Key { Up, Down, Left, Right,   │
│                Enter, Escape }          │
│                                         │
└─────────────────────────────────────────┘
```



Anytime you think "it's one of these options"

→ that's an enum!



--- slide ---

# Enums vs Magic Numbers



## Why Not Just Use Integers?

You COULD use numbers:

```rust
// 1 = Red, 2 = Yellow, 3 = Green
let light: i32 = 1;
```

But this is TERRIBLE:

```
┌─────────────────────────────────────────┐
│                                         │
│  PROBLEMS WITH NUMBERS:                 │
│                                         │
│  • What does 1 mean? (No clue!)         │
│  • What if someone uses 5? (Invalid!)   │
│  • Easy to confuse with other integers  │
│  • No compiler help                     │
│                                         │
│  WITH ENUMS:                            │
│                                         │
│  • TrafficLight::Red is self-explaining │
│  • Only valid variants exist            │
│  • Type system prevents mistakes        │
│  • Compiler catches errors              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Enums vs Strings



## Why Not Just Use Strings?

You COULD use strings:

```rust
let light = "red";
```

Also terrible:

```
┌─────────────────────────────────────────┐
│                                         │
│  PROBLEMS WITH STRINGS:                 │
│                                         │
│  • Typos: "red" vs "Red" vs "RED"       │
│  • No exhaustiveness checking           │
│  • Runtime errors instead of compile    │
│  • What if someone passes "purple"?     │
│                                         │
│  WITH ENUMS:                            │
│                                         │
│  • TrafficLight::red won't compile!     │
│  • Compiler knows all valid options     │
│  • Exhaustive matching (coming soon)    │
│  • Zero runtime overhead                │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Multiple Variants



## As Many As You Need

Enums can have any number of variants:

```rust
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

enum CardSuit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
```



No limit! Define what makes sense for your domain.



--- slide ---

# Deriving Debug



## Printing Enum Values

To print enum values with `{:?}`, you need `#[derive(Debug)]`:

```rust
#[derive(Debug)]
enum Direction {
    North,
    South,
}

fn main() {
    let dir = Direction::North;
    println!("{:?}", dir);  // prints: North
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  #[derive(Debug)]                       │
│                                         │
│  This is an ATTRIBUTE that tells Rust   │
│  to automatically generate the Debug    │
│  trait for your enum.                   │
│                                         │
│  Without it, println!("{:?}", ...) won't│
│  work on your enum values.              │
│                                         │
│  Put it RIGHT ABOVE the enum definition.│
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Defining Enum Variants

You have a `Message` enum that needs variants.

Look at how the enum is USED in main():

```rust
println!("{:?}", Message::Resize);
println!("{:?}", Message::Move);
println!("{:?}", Message::Echo);
println!("{:?}", Message::ChangeColor);
println!("{:?}", Message::Quit);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  WHAT THIS TELLS YOU:                   │
│                                         │
│  The Message enum needs these variants: │
│    • Resize                             │
│    • Move                               │
│    • Echo                               │
│    • ChangeColor                        │
│    • Quit                               │
│                                         │
│  Each one is used with Message::Name    │
│  so they must exist inside the enum!    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Unit Variants



## The Simplest Kind

In this exercise, you're creating **unit variants**.

These are variants with NO data attached:

```rust
enum Message {
    SomeVariant,    // ← unit variant
    AnotherOne,     // ← unit variant
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Unit variants are like labels.         │
│  They just represent "which one."       │
│                                         │
│  No parentheses: SomeVariant            │
│  No curly braces: SomeVariant           │
│  Just the name: SomeVariant             │
│                                         │
│  (Later you'll see variants WITH data,  │
│   but for now, keep it simple!)         │
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
│  1. How many variants does Message need?│
│     (Count the println! lines)          │
│                                         │
│  2. What are their exact names?         │
│     (Look at what follows Message::)    │
│                                         │
│  3. What syntax defines a unit variant? │
│     (Just the name, comma separated)    │
│                                         │
│  4. Is #[derive(Debug)] already there?  │
│     (Check the code - it should be!)    │
│                                         │
└─────────────────────────────────────────┘
```



The main() function is your specification.

Just define the variants it expects!

(Now go to the Editor and try it!)
