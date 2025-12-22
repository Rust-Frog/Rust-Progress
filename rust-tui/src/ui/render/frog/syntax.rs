//! Rust syntax highlighting for code blocks

use crate::ui::theme;
use ratatui::prelude::*;

/// Basic Rust syntax highlighting for a code line
pub fn highlight_rust_line(line: &str) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    spans.push(Span::raw("  ".to_string())); // Indent

    // Check for comment first
    let trimmed = line.trim_start();
    if trimmed.starts_with("//") {
        spans.push(Span::styled(
            line.to_string(),
            Style::default().fg(theme::colors::TEXT_DIM),
        ));
        return spans;
    }

    // Simple tokenizer
    let mut chars = line.chars().peekable();
    let mut current_token = String::new();

    while let Some(c) = chars.next() {
        match c {
            // String literals
            '"' => {
                if !current_token.is_empty() {
                    spans.push(style_token(&current_token));
                    current_token.clear();
                }
                let mut string = String::from('"');
                loop {
                    match chars.next() {
                        Some('\\') => {
                            string.push('\\');
                            if let Some(escaped) = chars.next() {
                                string.push(escaped);
                            }
                        }
                        Some('"') => {
                            string.push('"');
                            break;
                        }
                        Some(ch) => string.push(ch),
                        None => break,
                    }
                }
                spans.push(Span::styled(
                    string,
                    Style::default().fg(theme::colors::STRING),
                ));
            }
            // Punctuation and operators
            '(' | ')' | '{' | '}' | '[' | ']' | ';' | ':' | ',' | '.' | '+' | '-' | '*' | '/'
            | '=' | '<' | '>' | '!' | '&' | '|' | '^' | '?' => {
                if !current_token.is_empty() {
                    spans.push(style_token(&current_token));
                    current_token.clear();
                }
                if c == '-' && chars.peek() == Some(&'>') {
                    chars.next();
                    spans.push(Span::styled(
                        "->".to_string(),
                        Style::default().fg(theme::colors::PRIMARY),
                    ));
                } else {
                    spans.push(Span::styled(
                        c.to_string(),
                        Style::default().fg(theme::colors::TEXT),
                    ));
                }
            }
            // Whitespace
            ' ' | '\t' => {
                if !current_token.is_empty() {
                    spans.push(style_token(&current_token));
                    current_token.clear();
                }
                spans.push(Span::raw(c.to_string()));
            }
            // Part of identifier/number
            _ => {
                current_token.push(c);
            }
        }
    }

    if !current_token.is_empty() {
        spans.push(style_token(&current_token));
    }

    spans
}

/// Style a token based on whether it's a keyword, type, or identifier
fn style_token(token: &str) -> Span<'static> {
    const KEYWORDS: &[&str] = &[
        "fn", "let", "mut", "const", "if", "else", "match", "loop", "while", "for", "in", "return",
        "break", "continue", "struct", "enum", "impl", "trait", "pub", "mod", "use", "self",
        "Self", "super", "crate", "where", "async", "await", "move", "ref", "static", "type",
        "unsafe", "extern", "dyn", "as",
    ];

    const TYPES: &[&str] = &[
        "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
        "f32", "f64", "bool", "char", "str", "String", "Vec", "Option", "Result", "Box", "Rc",
        "Arc", "Ok", "Err", "Some", "None", "true", "false",
    ];

    if KEYWORDS.contains(&token) {
        Span::styled(
            token.to_string(),
            Style::default()
                .fg(theme::colors::PRIMARY)
                .add_modifier(Modifier::BOLD),
        )
    } else if TYPES.contains(&token) {
        Span::styled(
            token.to_string(),
            Style::default().fg(theme::colors::ACCENT),
        )
    } else if token
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        Span::styled(
            token.to_string(),
            Style::default().fg(theme::colors::WARNING),
        )
    } else if token.starts_with(char::is_uppercase) {
        Span::styled(
            token.to_string(),
            Style::default().fg(theme::colors::ACCENT),
        )
    } else {
        Span::styled(token.to_string(), Style::default().fg(theme::colors::TEXT))
    }
}
