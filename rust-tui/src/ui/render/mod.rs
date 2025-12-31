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

// strip ansi escape codes
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

// rust keywords for syntax highlighting
const KEYWORDS: &[&str] = &[
    "fn", "let", "mut", "if", "else", "match", "for", "while", "loop", "return", "use", "mod",
    "pub", "struct", "enum", "impl", "trait", "where", "const", "static", "self", "Self", "true",
    "false", "None", "Some", "Ok", "Err",
];

pub fn highlight_rust_line(line: &str, _is_current: bool) -> Line<'static> {
    let chars: Vec<char> = line.chars().collect();
    let mut spans = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        if let Some((span, _len)) = try_parse_comment(&chars, i) {
            spans.push(span);
            break; // comment goes to end of line
        }
        if let Some((span, len)) = try_parse_string(&chars, i) {
            spans.push(span);
            i += len;
            continue;
        }
        if let Some((span, len)) = try_parse_keyword(&chars, i) {
            spans.push(span);
            i += len;
            continue;
        }
        if let Some((span, len)) = try_parse_number(&chars, i) {
            spans.push(span);
            i += len;
            continue;
        }

        // default: single char
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

// try to parse // comment
fn try_parse_comment(chars: &[char], i: usize) -> Option<(Span<'static>, usize)> {
    if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '/' {
        let comment: String = chars[i..].iter().collect();
        let len = comment.len();
        Some((
            Span::styled(comment, Style::default().fg(theme::colors::COMMENT)),
            len,
        ))
    } else {
        None
    }
}

// try to parse "string"
fn try_parse_string(chars: &[char], start: usize) -> Option<(Span<'static>, usize)> {
    if chars[start] != '"' {
        return None;
    }

    let mut i = start + 1;
    while i < chars.len() && chars[i] != '"' {
        if chars[i] == '\\' && i + 1 < chars.len() {
            i += 1; // skip escaped char
        }
        i += 1;
    }
    if i < chars.len() {
        i += 1; // include closing quote
    }

    let s: String = chars[start..i].iter().collect();
    Some((
        Span::styled(s, Style::default().fg(theme::colors::STRING)),
        i - start,
    ))
}

// try to parse keyword
fn try_parse_keyword(chars: &[char], i: usize) -> Option<(Span<'static>, usize)> {
    for kw in KEYWORDS {
        if i + kw.len() > chars.len() {
            continue;
        }

        let word: String = chars[i..i + kw.len()].iter().collect();
        if word != *kw {
            continue;
        }

        // check word boundaries
        let next = chars.get(i + kw.len());
        let prev = if i > 0 { chars.get(i - 1) } else { None };

        let next_ok = next.map(|c| !c.is_alphanumeric()).unwrap_or(true);
        let prev_ok = prev.map(|c| !c.is_alphanumeric()).unwrap_or(true);

        if next_ok && prev_ok {
            return Some((
                Span::styled(word, Style::default().fg(theme::colors::KEYWORD)),
                kw.len(),
            ));
        }
    }
    None
}

// try to parse number
fn try_parse_number(chars: &[char], start: usize) -> Option<(Span<'static>, usize)> {
    if !chars[start].is_ascii_digit() {
        return None;
    }

    let mut i = start;
    while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
        i += 1;
    }

    let num: String = chars[start..i].iter().collect();
    Some((
        Span::styled(num, Style::default().fg(theme::colors::NUMBER)),
        i - start,
    ))
}
