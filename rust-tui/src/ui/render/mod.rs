use crate::ui::theme;
use ratatui::prelude::*;

pub mod editor;
pub mod footer;
pub mod frog;
pub mod header;
pub mod help;
pub mod panels;

pub use editor::render_editor;
pub use footer::render_footer;
pub use frog::render_frog_panel;
pub use header::render_header;
pub use help::render_help_modal;
pub use panels::{render_expanded_output, render_solution};

/// Utility to strip ANSI codes for clean TUI display
pub fn strip_ansi_codes(s: &str) -> String {
    let mut clean = String::with_capacity(s.len());
    let mut in_escape = false;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            in_escape = true;
            if let Some('[') = chars.peek() {
                chars.next();
            }
            continue;
        }
        if in_escape {
            if c.is_ascii_alphabetic() || c == '@' {
                in_escape = false;
            }
            continue;
        }
        clean.push(c);
    }
    clean
}

pub fn highlight_rust_line(line: &str, _is_current: bool) -> Line<'static> {
    // Simple syntax highlighting
    let mut spans = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Comments
        if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '/' {
            let comment: String = chars[i..].iter().collect();
            spans.push(Span::styled(
                comment,
                Style::default().fg(theme::colors::COMMENT),
            ));
            break;
        }

        // Strings
        if chars[i] == '"' {
            let start = i;
            i += 1;
            while i < chars.len() && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    i += 1;
                }
                i += 1;
            }
            if i < chars.len() {
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            spans.push(Span::styled(s, Style::default().fg(theme::colors::STRING)));
            continue;
        }

        // Keywords
        let keywords = [
            "fn", "let", "mut", "if", "else", "match", "for", "while", "loop", "return", "use",
            "mod", "pub", "struct", "enum", "impl", "trait", "where", "const", "static", "self",
            "Self", "true", "false", "None", "Some", "Ok", "Err",
        ];
        let mut found_keyword = false;
        for kw in keywords {
            if i + kw.len() <= chars.len() {
                let word: String = chars[i..i + kw.len()].iter().collect();
                if word == kw {
                    let next_char = chars.get(i + kw.len());
                    let prev_char = if i > 0 { chars.get(i - 1) } else { None };
                    if (next_char.is_none() || !next_char.unwrap().is_alphanumeric())
                        && (prev_char.is_none() || !prev_char.unwrap().is_alphanumeric())
                    {
                        spans.push(Span::styled(
                            word,
                            Style::default().fg(theme::colors::KEYWORD),
                        ));
                        i += kw.len();
                        found_keyword = true;
                        break;
                    }
                }
            }
        }
        if found_keyword {
            continue;
        }

        // Numbers
        if chars[i].is_ascii_digit() {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }
            let num: String = chars[start..i].iter().collect();
            spans.push(Span::styled(
                num,
                Style::default().fg(theme::colors::NUMBER),
            ));
            continue;
        }

        // Default text
        spans.push(Span::styled(
            chars[i].to_string(),
            Style::default().fg(theme::colors::TEXT),
        ));
        i += 1;
    }

    if spans.is_empty() {
        spans.push(Span::styled(" ", Style::default()));
    }

    Line::from(spans)
}
