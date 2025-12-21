# 🦀 Rustlings TUI

> **The premium, interactive way to learn Rust.**

![Main Interface](assets/main.png)

A modern, immersive Terminal User Interface (TUI) for [Rustlings](https://github.com/rust-lang/rustlings), designed to make learning Rust a beautiful and seamless experience.

## ✨ Features

### 🖥️ Immersive Editor
Forgot about switching windows. Write code directly in the terminal with a **custom-built, Vim-inspired editor** featuring syntax highlighting, line numbers, and smooth scrolling.

![Error Handling](assets/error.png)

### 🚀 Real-time Feedback
Instant gratification (or correction) with an integrated output panel.
- **Compilation Errors**: See standard Rustc output right below your code.
- **Automatic Scrolling**: Smart output scrolling ensures you never miss the actual error message.
- **Hints**: Stuck? Toggle hints with a single keystroke.

### 📊 Beautiful Progress Tracking
Keep your eyes on the prize with a redesigned progress bar.
- **Pulse Animation**: A living, breathing indicator of your session.
- **Precise Metrics**: Track your completion percentage and completed exercises at a glance.

### 🆚 Intelligent Solution View
Compare your work against the official solution side-by-side without leaving your flow.

![Solution View](assets/solution.png)

## 🎮 Controls

The TUI allows for both keyboard-centric workflows and mouse interaction.

| Key | Action |
| :--- | :--- |
| **Navigation** | |
| `n` | Go to **Next** exercise |
| `p` | Go to **Previous** exercise |
| `Shift+J` / `PgDn` | Scroll output **Down** |
| `Shift+K` / `PgUp` | Scroll output **Up** |
| **Editing** | |
| `i` | Enter **Insert** mode |
| `Esc` | Exit Insert mode / Cancel |
| `:w` | **Save** changes |
| **Tools** | |
| `:c` | **Check/Compile** code manually |
| `:auto` | Toggle **Auto-Advance** on success |
| `:watch` | Toggle **Auto-Compile** on file change |
| `s` / `:sol` | Toggle **Solution** view |
| `h` / `:hint` | Show **Hint** |
| `r` / `:reload` | **Reload** file from disk |
| `:reset` | **Reset** exercise to original state |
| `q` / `:q` | **Quit** |

![Command Mode](assets/commands.png)

## 🛠️ Installation

Already have Rustlings installed? Just run the updated binary!

```bash
cargo install --path . --force
```

Then start the TUI:

```bash
rustlings
```

---
*Built with ❤️ for the Rust community.*
