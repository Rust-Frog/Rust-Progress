# Getting Started with Rustlings TUI

Welcome to your interactive Rust learning environment.

## Navigating This Tutorial

```
┌──────────────┬─────────────────────────────┐
│ Shift+Right  │ Next slide                  │
│ Shift+Left   │ Previous slide              │
│ Shift+Down   │ Scroll down                 │
│ Shift+Up     │ Scroll up                   │
│ Shift+F      │ Hide/show this panel        │
└──────────────┴─────────────────────────────┘
```

## Overview

Rustlings is a curated collection of small exercises 
designed to teach Rust through hands-on practice. 
You learn by fixing broken code.

This TUI provides an integrated environment where you 
can read, edit, and compile Rust code in your terminal.

--- slide ---

# Understanding the Interface

## Panel Layout

```
┌─────────────────────────────────────────────────┐
│  EDITOR PANEL          │  FROG PANEL 🐸         │
│                        │                        │
│  Read and edit the     │  This tutorial panel.  │
│  exercise code here.   │  Toggle with Shift+F   │
│                        │                        │
├────────────────────────┴────────────────────────┤
│  OUTPUT PANEL                                   │
│  Compiler output appears here after each save.  │
└─────────────────────────────────────────────────┘
```

## What Each Panel Does

```
┌─────────────┬───────────────────────────────────┐
│ Panel       │ Purpose                           │
├─────────────┼───────────────────────────────────┤
│ Editor      │ Main workspace for code editing   │
│ Output      │ Compiler errors and test results  │
│ Frog        │ Tutorial and help content         │
└─────────────┴───────────────────────────────────┘
```

--- slide ---

# How Exercises Work

## The Learning Model

Rustlings teaches through compiler-driven development.
Each exercise starts with broken code. Your job:

1. Read the error messages
2. Understand what's wrong
3. Fix it

The Rust compiler is famously helpful — its error messages 
often tell you exactly what's wrong and how to fix it.

## Exercise Structure

```
┌─────────────────────────────────────────────────┐
│  1. COMMENTS    What you need to do             │
│  2. CODE        The Rust code to fix            │
│  3. TESTS       Verify your solution works      │
└─────────────────────────────────────────────────┘
```

## Types of Exercises

```
┌──────────────────────┬──────────────────────────┐
│ Type                 │ What to Do               │
├──────────────────────┼──────────────────────────┤
│ Compiler Errors      │ Fix syntax/type issues   │
│ Failing Tests        │ Make tests pass          │
│ Incomplete Code      │ Replace todo!() macros   │
└──────────────────────┴──────────────────────────┘
```

--- slide ---

# The Development Cycle

## Step-by-Step Workflow

```
┌─────────────────────────────────────────────────┐
│                                                 │
│  1. READ THE EXERCISE                           │
│     Read the comments at the top first.         │
│                                                 │
│  2. EXAMINE THE CODE                            │
│     Find what's broken or missing.              │
│                                                 │
│  3. CHECK THE OUTPUT PANEL                      │
│     Read compiler errors carefully.             │
│                                                 │
│  4. MAKE YOUR CHANGES                           │
│     Press i to edit, Esc when done, :w to save  │
│                                                 │
│  5. ITERATE                                     │
│     Repeat until you see green output.          │
│                                                 │
│  6. MOVE ON                                     │
│     Press ] to go to the next exercise.         │
│                                                 │
└─────────────────────────────────────────────────┘
```

--- slide ---

# Reading Compiler Errors

## Anatomy of a Rust Error

```
error[E0425]: cannot find value `x` in this scope
 --> src/main.rs:5:20
  |
5 |     println!("{}", x);
  |                    ^ not found in this scope
```

## Understanding the Parts

```
┌────────────────────────┬────────────────────────┐
│ Part                   │ Meaning                │
├────────────────────────┼────────────────────────┤
│ error[E0425]           │ Searchable error code  │
│ cannot find value      │ What went wrong        │
│ --> src/main.rs:5:20   │ File, line, column     │
│ ^ not found            │ Exact location + hint  │
└────────────────────────┴────────────────────────┘
```

## Pro Tips

- Start with the FIRST error (later ones cascade)
- Read the full message — look for "help:" lines
- Check the line ABOVE for missing semicolons

--- slide ---

# The Editor

The TUI uses a Vim-style modal editor.

## Modal Editing

Unlike normal editors, Vim has modes:

```
┌─────────────┬───────────────────────────────────┐
│ Mode        │ Purpose                           │
├─────────────┼───────────────────────────────────┤
│ Normal      │ Navigate, delete, copy (default)  │
│ Insert      │ Type text (press i to enter)      │
│ Command     │ Save, quit (press : to enter)     │
└─────────────┴───────────────────────────────────┘
```

## Essential Commands

```
┌─────────────┬─────────────────────────────┐
│ Key         │ Action                      │
├─────────────┼─────────────────────────────┤
│ i           │ Enter Insert mode           │
│ Esc         │ Return to Normal mode       │
│ :w          │ Save file                   │
│ :q          │ Quit TUI                    │
│ u           │ Undo                        │
└─────────────┴─────────────────────────────┘
```

--- slide ---

# Navigation Reference

## Moving the Cursor (Normal Mode)

```
┌─────────────┬─────────────────────────────┐
│ Key         │ Action                      │
├─────────────┼─────────────────────────────┤
│ h / l       │ Left / Right                │
│ j / k       │ Down / Up                   │
│ w / b       │ Next word / Previous word   │
│ 0 / $       │ Line start / Line end       │
│ gg / G      │ File start / File end       │
└─────────────┴─────────────────────────────┘
```

## Exercise Navigation

```
┌─────────────┬─────────────────────────────┐
│ Key         │ Action                      │
├─────────────┼─────────────────────────────┤
│ [           │ Previous exercise           │
│ ]           │ Next exercise               │
│ Shift+L     │ Open exercise list          │
└─────────────┴─────────────────────────────┘
```

--- slide ---

# Tips & Tricks

## Get More Screen Space

- Press `Shift+F` to hide this Frog panel
- More room for code editing!

## Viewing Solutions

- Press `s` to toggle the solution view
- Try solving yourself first — struggle = learning

## When You're Stuck

- Read the comments in the exercise carefully
- Check compiler error messages for hints
- Many exercises have hint text in the Output panel

## Moving Through Exercises

- Press `]` to go to the next exercise
- Press `[` to go to the previous exercise
- Press `Shift+L` to open the exercise list

--- slide ---

# Quick Reference Card

## File Operations

```
┌─────────────┬─────────────────────────────┐
│ :w          │ Save (triggers compile)     │
│ :q          │ Quit                        │
│ :wq         │ Save and quit               │
└─────────────┴─────────────────────────────┘
```

## Editing Commands

```
┌─────────────┬─────────────────────────────┐
│ x           │ Delete character            │
│ dd          │ Delete line                 │
│ yy          │ Copy line                   │
│ p           │ Paste                       │
│ u           │ Undo                        │
│ Ctrl+r      │ Redo                        │
└─────────────┴─────────────────────────────┘
```

## Getting Help

- Read the TODO comments in each exercise
- Check hints before solutions
- Search Rust error codes online (e.g., "Rust E0308")

---

You're ready! Press ] to go to the first exercise.
