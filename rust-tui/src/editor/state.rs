//! Editor state management
use anyhow::{Context, Result};
use edtui::{EditorState as EdtuiState, Lines};
use std::fs;
use std::process::Command;

use crate::app_state::AppState;

/// Commands that can be executed from the editor command mode
#[derive(Debug, Clone)]
pub enum EditorCommand {
    Save,
    Quit,
    SaveAndQuit,
    SaveAndCompile,
    ForceQuit,
    ToggleHint,
    ConfirmQuit,
}

/// Editor mode
#[derive(Debug, Clone, PartialEq)]
pub enum EditorMode {
    Normal,
    Insert,
    Command,
}

/// State for the integrated editor
pub struct EditorState {
    /// The underlying edtui editor state
    pub editor: EdtuiState,
    /// Path to the current file
    pub file_path: String,
    /// Current editor mode
    pub mode: EditorMode,
    /// Command buffer for : commands
    pub command_buffer: String,
    /// Result of compile
    pub compile_result: Option<String>,
    /// Whether the file has been modified
    pub modified: bool,
    /// Hint text for current exercise
    pub hint: String,
    /// Whether to show hint
    pub show_hint: bool,
}

impl EditorState {
    /// Create a new editor state for the given exercise
    pub fn new(app_state: &AppState) -> Result<Self> {
        let exercise = app_state.current_exercise();
        let file_path = exercise.path;
        let hint = exercise.hint;

        // Read file content
        let content = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read exercise file: {}", file_path))?;

        // Create edtui state with content using Lines::from
        let editor = EdtuiState::new(Lines::from(content.as_str()));

        Ok(Self {
            editor,
            file_path: file_path.to_string(),
            mode: EditorMode::Normal,
            command_buffer: String::new(),
            compile_result: None,
            modified: false,
            hint: hint.to_string(),
            show_hint: false,
        })
    }

    /// Save the current file
    pub fn save(&mut self) -> Result<()> {
        let content = self.get_content();
        fs::write(&self.file_path, &content)
            .with_context(|| format!("Failed to save file: {}", self.file_path))?;
        self.modified = false;
        Ok(())
    }

    /// Get the current content as a string
    pub fn get_content(&self) -> String {
        // Use flatten with newline separator to get Vec<char>, then collect to String
        self.editor.lines.flatten(&Some('\n')).into_iter().collect()
    }

    /// Save and compile the exercise
    pub fn save_and_compile(&mut self) -> Result<()> {
        self.save()?;

        // Run cargo check on the file
        let output = Command::new("cargo")
            .args(["check", "--color=never"])
            .current_dir(
                std::path::Path::new(&self.file_path)
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap(),
            )
            .output()?;

        let result = if output.status.success() {
            "✓ Compilation successful!".to_string()
        } else {
            String::from_utf8_lossy(&output.stderr).to_string()
        };

        self.compile_result = Some(result);
        Ok(())
    }

    /// Toggle hint display
    pub fn toggle_hint(&mut self) {
        self.show_hint = !self.show_hint;
    }

    /// Enter insert mode
    pub fn enter_insert_mode(&mut self) {
        self.mode = EditorMode::Insert;
    }

    /// Enter normal mode
    pub fn enter_normal_mode(&mut self) {
        self.mode = EditorMode::Normal;
    }

    /// Enter command mode
    pub fn enter_command_mode(&mut self) {
        self.mode = EditorMode::Command;
        self.command_buffer.clear();
    }

    /// Parse and execute a command, returns Some(true) if should exit
    pub fn execute_command(&mut self, cmd: &str) -> Result<Option<bool>> {
        match cmd.trim() {
            "w" => {
                self.save()?;
                self.compile_result = Some("File saved.".to_string());
                Ok(Some(false))
            }
            "q" => {
                if self.modified {
                    self.compile_result = Some(
                        "Unsaved changes! Use :q! to force quit or :wq to save and quit."
                            .to_string(),
                    );
                    Ok(Some(false))
                } else {
                    Ok(Some(true))
                }
            }
            "q!" => Ok(Some(true)),
            "wq" | "x" => {
                self.save()?;
                Ok(Some(true))
            }
            "c" => {
                self.save_and_compile()?;
                Ok(Some(false))
            }
            _ => {
                self.compile_result = Some(format!("Unknown command: {}", cmd));
                Ok(Some(false))
            }
        }
    }
}
