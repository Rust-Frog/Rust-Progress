# Rustlings TUI - Future Enhancements

A modern, enhanced TUI for Rustlings with better visual feedback and user experience.

## Completed Features

- [x] **Proper Exercise Runner** - Uses rustlings' `RunnableExercise` (cargo build, test, clippy)
- [x] **Solution Viewer** - Press `s` to view solutions side-by-side with exercises
- [x] **File Watcher** - Auto-reloads and compiles when files change externally
- [x] **Pulsing Progress Ball** - Smooth sinusoidal animation with orange theme
- [x] **Scrollable Output** - J/K or PageUp/PageDown to scroll compiler output
- [x] **Vim-style Editor** - Normal, Insert, and Command modes
- [x] **Syntax Highlighting** - Rust keywords, strings, comments, numbers
- [x] **Progress Tracking** - Persistent state, shows completed/pending exercises
- [x] **Strikethrough Completed** - Exercise names crossed out when done

## Planned Enhancements

### High Priority

- [ ] **Exercise List View** - Browse all exercises with filter (done/pending)
- [ ] **Better Error Display** - Parse and highlight specific error lines in code
- [ ] **Jump to Error** - Click or press key to jump to error line in editor
- [ ] **Undo/Redo** - Basic undo/redo for editor changes
- [ ] **Search in File** - `/` to search within current file

### Medium Priority

- [ ] **Split Pane Resize** - Drag or keys to resize editor/solution panes
- [ ] **Theme Selector** - Switch between color themes (Dracula, Nord, Solarized)
- [ ] **Exercise Categories** - Show exercise category/chapter in header
- [ ] **Time Tracking** - Track time spent on each exercise
- [ ] **Diff View** - Show diff between your solution and official solution

### Low Priority / Nice to Have

- [ ] **Mini-map** - Code mini-map on the side like VS Code
- [ ] **Auto-complete** - Basic keyword/snippet completion
- [ ] **Bracket Matching** - Highlight matching brackets
- [ ] **Line Wrapping Toggle** - Option for soft line wrapping
- [ ] **Custom Keybindings** - Config file for key remapping
- [ ] **Session Statistics** - Show stats at end (exercises done, time, etc.)

### Visual Enhancements

- [ ] **Animated Success** - Celebration animation when completing exercises
- [ ] **Progress Milestones** - Special effects at 25%, 50%, 75%, 100%
- [ ] **Typing Effects** - Subtle cursor animations
- [ ] **Gradient Progress Bar** - Gradient from orange to green as you progress

## Commands Reference

### Normal Mode
| Key | Action |
|-----|--------|
| `i` | Enter insert mode |
| `:` | Enter command mode |
| `s` | Toggle solution view |
| `n` | Next exercise |
| `p` | Previous exercise |
| `h/j/k/l` | Move cursor (vim-style) |
| `J/K` | Scroll output up/down |
| `PageUp/PageDown` | Scroll output (3 lines) |
| `Home/End` | Scroll to top/bottom |
| `q` | Quit |

### Command Mode
| Command | Action |
|---------|--------|
| `:w` | Save file |
| `:c` | Compile/check exercise |
| `:h` or `:hint` | Show hint |
| `:s` or `:solution` | Toggle solution |
| `:n` or `:next` | Next exercise |
| `:p` or `:prev` | Previous exercise |
| `:auto` | Toggle auto-advance |
| `:watch` | Toggle auto-compile on file change |
| `:r` or `:reload` | Reload exercise from disk |
| `:reset` | Reset exercise to original |
| `:help` | Show command help |
| `:q` | Quit (warns if unsaved) |
| `:q!` | Force quit |
| `:wq` or `:x` | Save and quit |

## Contributing

Ideas and PRs welcome! Focus areas:
1. Stability and correctness
2. User experience improvements
3. Visual polish
4. Performance optimizations

## Architecture Notes

- `src/ui/tui.rs` - Main TUI logic, rendering, key handling
- `src/ui/theme.rs` - Colors and styling
- `src/ui/layout.rs` - Layout calculations
- `src/app_state.rs` - Exercise state, progress tracking
- `src/exercise.rs` - Exercise running (build, test, clippy)
- `src/embedded.rs` - Embedded files (solutions, exercises)
