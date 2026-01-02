# The From Trait: Value Conversions



## From and Into

`From` defines how to create a type from another type:

```rust
let s: String = String::from("hello");
let num: i64 = i64::from(5i32);
```

When you implement `From`, you get `Into` for free!

```rust
let s: String = "hello".into();
```



```
┌─────────────────────────────────────────┐
│                                         │
│  From<T> for U means:                   │
│  "I can create a U from a T"            │
│                                         │
│  U::from(t)  ← explicit                 │
│  t.into()    ← inferred from context    │
│                                         │
│  They're two sides of the same coin!    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Implementing From



## The Trait Signature

```rust
impl From<SourceType> for TargetType {
    fn from(source: SourceType) -> Self {
        // Convert source into Self (TargetType)
    }
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Example:                               │
│                                         │
│  impl From<&str> for Person {           │
│      fn from(s: &str) -> Self {         │
│          // Parse s into a Person       │
│      }                                  │
│  }                                      │
│                                         │
│  Then you can do:                       │
│  Person::from("Mark,20")                │
│  "Mark,20".into()                       │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Parse "name,age" into Person

```rust
struct Person {
    name: String,
    age: u8,
}

impl From<&str> for Person {
    fn from(s: &str) -> Self {
        // TODO: Parse "Mark,20" into Person
    }
}
```

If parsing fails, return `Person::default()`.



--- slide ---

# Parsing Strategy



## Step by Step

```
┌─────────────────────────────────────────┐
│                                         │
│  Input: "Mark,20"                       │
│                                         │
│  1. Split on comma:                     │
│     s.split(',') → ["Mark", "20"]       │
│                                         │
│  2. Collect into Vec:                   │
│     .collect::<Vec<&str>>()             │
│                                         │
│  3. Check length == 2:                  │
│     if parts.len() != 2 { return default }│
│                                         │
│  4. Get name (first part):              │
│     parts[0]                            │
│                                         │
│  5. Parse age (second part):            │
│     parts[1].parse::<u8>()              │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Handling Edge Cases



## Return Default on Failure

```rust
// Wrong number of parts
if parts.len() != 2 {
    return Person::default();
}

// Empty name
if name.is_empty() {
    return Person::default();
}

// Can't parse age
let age = match parts[1].parse::<u8>() {
    Ok(a) => a,
    Err(_) => return Person::default(),
};
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Person::default() returns:             │
│  Person { name: "John", age: 30 }       │
│                                         │
│  Use it as fallback for ANY error!      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Useful String Methods



## Parsing Tools

```rust
// Split string by delimiter
"a,b,c".split(',')  // Iterator: "a", "b", "c"

// Collect into Vec
.collect::<Vec<&str>>()

// Parse string to number
"42".parse::<u8>()  // Ok(42)
"xx".parse::<u8>()  // Err(...)

// Check if string is empty
"".is_empty()  // true
```



--- slide ---

# Think It Through



## Before You Type



```
┌─────────────────────────────────────────┐
│  ASK YOURSELF:                          │
│                                         │
│  1. How do you split a string on ','?   │
│     (s.split(','))                      │
│                                         │
│  2. How do you convert an iterator to   │
│     a Vec?                              │
│     (.collect::<Vec<&str>>())           │
│                                         │
│  3. How do you parse a string to u8?    │
│     (.parse::<u8>())                    │
│                                         │
│  4. What should you return on error?    │
│     (Person::default())                 │
│                                         │
│  5. How do you create the final Person? │
│     (Person { name: ..., age: ... })    │
│                                         │
└─────────────────────────────────────────┘
```



From: flexible type conversion!

(Go try it in the Editor!)
