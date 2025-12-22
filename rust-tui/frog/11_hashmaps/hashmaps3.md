# Updating HashMap Values



## Beyond Simple Insert

Sometimes you need to UPDATE existing values,

not just insert new ones.



```
┌─────────────────────────────────────────┐
│                                         │
│  COMMON SCENARIOS:                      │
│                                         │
│  • Counting occurrences                 │
│    "word" appears → increment count     │
│                                         │
│  • Accumulating totals                  │
│    Player scores again → add to total   │
│                                         │
│  • Tracking statistics                  │
│    Team scores → update scored/conceded │
│                                         │
│  You need to:                           │
│  1. Get the existing value (or default) │
│  2. Modify it                           │
│  3. Store the updated value             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Challenge



## Get, Modify, Store?

The naive approach is awkward:

```rust
// Get current value (or 0 if missing)
let current = *map.get(&key).unwrap_or(&0);

// Calculate new value
let new_value = current + amount;

// Store it back
map.insert(key, new_value);
```



```
┌─────────────────────────────────────────┐
│                                         │
│  PROBLEMS:                              │
│                                         │
│  • Multiple operations                  │
│  • Multiple lookups (get + insert)      │
│  • Verbose and error-prone              │
│  • Ownership can get complicated        │
│                                         │
│  The Entry API solves this elegantly!   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# or_insert Returns &mut



## The Key Insight

Remember: `or_insert()` returns `&mut V`:

```rust
let count = map.entry(key).or_insert(0);
// count is &mut V - a mutable reference!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  .or_insert(default) returns &mut V     │
│                                         │
│  This reference points to:              │
│  • The existing value (if key existed)  │
│  • The newly inserted default (if not)  │
│                                         │
│  Either way, you get a mutable          │
│  reference to the value in the map!     │
│                                         │
│  You can MODIFY through this reference! │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Modifying Through Reference



## The *dereference Pattern

```rust
let count = map.entry("word").or_insert(0);
*count += 1;  // Increment the value!
```



```
┌─────────────────────────────────────────┐
│                                         │
│  *count += 1;                           │
│  ↑                                      │
│  Dereference operator                   │
│                                         │
│  `count` is &mut i32 (reference)        │
│  `*count` is the actual i32 value       │
│                                         │
│  To modify through a reference:         │
│  *reference = new_value;                │
│  *reference += amount;                  │
│  *reference -= amount;                  │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Complete Pattern



## Get-Or-Insert-Then-Modify

```rust
// One line does it all:
*map.entry(key).or_insert(default) += amount;
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Breaking it down:                      │
│                                         │
│  map.entry(key)                         │
│    → Get the Entry for this key         │
│                                         │
│  .or_insert(default)                    │
│    → If missing, insert default         │
│    → Returns &mut to the value          │
│                                         │
│  *...                                   │
│    → Dereference to get actual value    │
│                                         │
│  += amount                              │
│    → Modify the value in place          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Word Counting Example



## A Classic Use Case

```rust
let text = "hello world hello rust hello";
let mut counts = HashMap::new();

for word in text.split_whitespace() {
    let count = counts.entry(word).or_insert(0);
    *count += 1;
}

// counts: {"hello": 3, "world": 1, "rust": 1}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  First "hello":                         │
│    entry("hello") → Vacant              │
│    or_insert(0) → inserts 0, returns &mut│
│    *count += 1 → value becomes 1        │
│                                         │
│  "world":                               │
│    entry("world") → Vacant              │
│    or_insert(0) → inserts 0             │
│    *count += 1 → value becomes 1        │
│                                         │
│  Second "hello":                        │
│    entry("hello") → Occupied (value: 1) │
│    or_insert(0) → returns &mut to 1     │
│    *count += 1 → value becomes 2        │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Struct Values



## HashMaps Can Store Anything

Values can be complex types like structs:

```rust
struct Stats {
    wins: u32,
    losses: u32,
}

let mut teams: HashMap<String, Stats> = HashMap::new();
```



```
┌─────────────────────────────────────────┐
│                                         │
│  HashMap<String, Stats>                 │
│                                         │
│  Key: team name (String)                │
│  Value: Stats struct                    │
│                                         │
│  ┌────────────┬─────────────────────┐   │
│  │   "Red"    │ Stats{wins:5,losses:2}│ │
│  │   "Blue"   │ Stats{wins:3,losses:4}│ │
│  └────────────┴─────────────────────┘   │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Modifying Struct Fields



## Access Through Reference

When you have `&mut Struct`, you can modify fields:

```rust
let stats = map.entry(team).or_insert(Stats::default());
stats.wins += 1;
stats.losses += 0;
```



```
┌─────────────────────────────────────────┐
│                                         │
│  stats is &mut Stats                    │
│                                         │
│  Access fields with dot notation:       │
│    stats.wins                           │
│    stats.losses                         │
│                                         │
│  Modify fields directly:                │
│    stats.wins += 1;                     │
│    stats.losses = new_value;            │
│                                         │
│  No need to dereference for struct      │
│  field access! Rust handles it.         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Default Trait



## Creating Default Values

Types can implement Default for easy initialization:

```rust
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

// TeamScores::default() creates:
// TeamScores { goals_scored: 0, goals_conceded: 0 }
```



```
┌─────────────────────────────────────────┐
│                                         │
│  #[derive(Default)]                     │
│                                         │
│  Auto-implements Default trait.         │
│  All fields get their default values:   │
│                                         │
│  • Numbers → 0                          │
│  • bool → false                         │
│  • String → ""                          │
│  • Option → None                        │
│                                         │
│  Use with or_default():                 │
│  map.entry(key).or_default()            │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Processing Multiple Records



## Building Up Statistics

Common pattern: process records, accumulate stats:

```rust
for record in records {
    // Get or create entry for this key
    let entry = map.entry(record.key).or_default();

    // Update the entry's fields
    entry.field1 += record.value1;
    entry.field2 += record.value2;
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Each record might:                     │
│                                         │
│  • Reference an EXISTING key            │
│    → Get the existing entry             │
│    → Update its values                  │
│                                         │
│  • Reference a NEW key                  │
│    → Create entry with defaults         │
│    → Update its values                  │
│                                         │
│  Entry API handles both cases!          │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Your Exercise



## Building a Scores Table

You're processing soccer match results:

```
"England,France,4,2"
```

This means: England scored 4, France scored 2.



```
┌─────────────────────────────────────────┐
│                                         │
│  For EACH team in EACH match:           │
│                                         │
│  England in "England,France,4,2":       │
│    • Scored 4 goals                     │
│    • Conceded 2 goals (France's score)  │
│                                         │
│  France in "England,France,4,2":        │
│    • Scored 2 goals                     │
│    • Conceded 4 goals (England's score) │
│                                         │
│  You must ACCUMULATE across all matches!│
│  A team might appear in multiple games. │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Data Structure



## TeamScores Struct

```rust
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}
```



```
┌─────────────────────────────────────────┐
│                                         │
│  goals_scored = goals this team made    │
│  goals_conceded = goals against them    │
│                                         │
│  #[derive(Default)] means:              │
│    TeamScores::default() creates        │
│    TeamScores { goals_scored: 0,        │
│                 goals_conceded: 0 }     │
│                                         │
│  Perfect for initializing new teams!    │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Understanding the Data



## Each Line Has Two Teams

```
"team_1,team_2,team_1_goals,team_2_goals"
```



```
┌─────────────────────────────────────────┐
│                                         │
│  Line: "England,France,4,2"             │
│                                         │
│  team_1_name = "England"                │
│  team_2_name = "France"                 │
│  team_1_score = 4                       │
│  team_2_score = 2                       │
│                                         │
│  TWO teams need updating per line!      │
│                                         │
│  Team 1 (England):                      │
│    scored += team_1_score (4)           │
│    conceded += team_2_score (2)         │
│                                         │
│  Team 2 (France):                       │
│    scored += team_2_score (2)           │
│    conceded += team_1_score (4)         │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Cross Relationship



## One Team's Score = Other's Conceded

```
┌─────────────────────────────────────────┐
│                                         │
│  Critical insight:                      │
│                                         │
│  Team 1's goals_scored                  │
│    = team_1_score                       │
│                                         │
│  Team 1's goals_conceded                │
│    = team_2_score (what opponent scored)│
│                                         │
│  ─────────────────────────────          │
│                                         │
│  Team 2's goals_scored                  │
│    = team_2_score                       │
│                                         │
│  Team 2's goals_conceded                │
│    = team_1_score (what opponent scored)│
│                                         │
│  Goals scored BY team 1                 │
│  = Goals conceded BY team 2             │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# The Algorithm



## For Each Match Line

```
┌─────────────────────────────────────────┐
│                                         │
│  1. Parse the line (already done)       │
│     → team_1_name, team_2_name          │
│     → team_1_score, team_2_score        │
│                                         │
│  2. Update team 1's entry               │
│     → Get or create entry for team_1    │
│     → Add to goals_scored               │
│     → Add to goals_conceded             │
│                                         │
│  3. Update team 2's entry               │
│     → Get or create entry for team_2    │
│     → Add to goals_scored               │
│     → Add to goals_conceded             │
│                                         │
│  The Entry API + struct field access    │
│  makes this clean!                      │
│                                         │
└─────────────────────────────────────────┘
```



--- slide ---

# Entry With Struct Values



## The Pattern You Need

```rust
// Get entry (or create with defaults)
let team = scores.entry(team_name).or_default();

// Modify struct fields through the reference
team.goals_scored += some_value;
team.goals_conceded += other_value;
```



```
┌─────────────────────────────────────────┐
│                                         │
│  scores.entry(team_name)                │
│    → Entry for this team                │
│                                         │
│  .or_default()                          │
│    → If new team: create TeamScores     │
│      with zeros                         │
│    → Returns &mut TeamScores            │
│                                         │
│  team.goals_scored += value             │
│    → Modify through the reference       │
│    → Changes the value in the HashMap   │
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
│  1. How many teams need updating        │
│     for each match line?                │
│                                         │
│  2. For team_1:                         │
│     - What value goes into scored?      │
│     - What value goes into conceded?    │
│                                         │
│  3. For team_2:                         │
│     - What value goes into scored?      │
│     - What value goes into conceded?    │
│                                         │
│  4. How do you get a mutable reference  │
│     to a team's entry (creating if new)?│
│                                         │
│  5. How do you add to a struct field    │
│     through a mutable reference?        │
│                                         │
│  6. += adds to existing value.          │
│     Why is this important for teams     │
│     that play multiple matches?         │
│                                         │
└─────────────────────────────────────────┘
```



Two teams per line, both need updating!

(Now go to the Editor and try it!)
