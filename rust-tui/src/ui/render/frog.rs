use crate::ui::theme;
use crate::ui::tui::TuiState;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render_frog_panel(frame: &mut Frame, area: Rect, state: &mut TuiState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::colors::SUCCESS))
        .title(Span::styled(
            " 🐸 Frog Learning Panel ",
            Style::default()
                .fg(theme::colors::SUCCESS)
                .add_modifier(Modifier::BOLD),
        ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if state.current_frog_steps.is_empty() {
        let text = vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "  No extra content for this exercise.",
                Style::default().fg(theme::colors::TEXT_DIM),
            )]),
        ];
        let content = Paragraph::new(text).style(Style::default().fg(theme::colors::TEXT));
        frame.render_widget(content, inner);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(inner);

    // 1. Render Step Bar
    let total_steps = state.current_frog_steps.len();
    let mut step_spans = Vec::new();

    step_spans.push(Span::raw("  ")); // Padding

    for i in 0..total_steps {
        let is_current = i == state.frog_step;
        let style = if is_current {
            Style::default()
                .fg(theme::colors::BG_DARK)
                .bg(theme::colors::ACCENT)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::colors::TEXT_DIM)
        };

        step_spans.push(Span::styled(format!(" {} ", i + 1), style));
        step_spans.push(Span::raw(" ")); // Spacing
    }

    let step_bar = Paragraph::new(Line::from(step_spans)).block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(theme::colors::BG_LIGHT)),
    );
    frame.render_widget(step_bar, chunks[0]);

    // 2. Render Current Step Content with scroll tracking
    if let Some(content_str) = state.current_frog_steps.get(state.frog_step) {
        let content_lines = render_markdown(content_str);
        let total_lines = content_lines.len();
        let visible_height = chunks[1].height as usize;

        // Calculate max scroll (prevent scrolling past content)
        let max_scroll = if total_lines > visible_height {
            total_lines.saturating_sub(visible_height)
        } else {
            0
        };

        // Clamp frog_scroll to valid range
        if state.frog_scroll > max_scroll {
            state.frog_scroll = max_scroll;
        }

        // Update state with scroll info for next_frog_step logic
        state.frog_content_height = total_lines;
        state.frog_visible_height = visible_height;

        let content = Paragraph::new(content_lines)
            .style(Style::default().fg(theme::colors::TEXT))
            .wrap(Wrap { trim: false })
            .scroll((state.frog_scroll as u16, 0));

        // Reserve space for scroll indicator if needed
        let content_area = if total_lines > visible_height {
            Rect {
                x: chunks[1].x,
                y: chunks[1].y,
                width: chunks[1].width.saturating_sub(2),
                height: chunks[1].height,
            }
        } else {
            chunks[1]
        };

        frame.render_widget(content, content_area);

        // 3. Render scroll indicator if content overflows
        if total_lines > visible_height {
            let can_scroll_up = state.frog_scroll > 0;
            let can_scroll_down = state.frog_scroll < max_scroll;

            // Show scroll position indicator on the right
            let indicator_x = chunks[1].x + chunks[1].width - 1;

            // Top indicator (if can scroll up)
            if can_scroll_up {
                frame.render_widget(
                    Paragraph::new("▲").style(Style::default().fg(theme::colors::ACCENT)),
                    Rect {
                        x: indicator_x,
                        y: chunks[1].y,
                        width: 1,
                        height: 1,
                    },
                );
            }

            // Bottom indicator (if can scroll down)
            if can_scroll_down {
                frame.render_widget(
                    Paragraph::new("▼").style(Style::default().fg(theme::colors::ACCENT)),
                    Rect {
                        x: indicator_x,
                        y: chunks[1].y + chunks[1].height - 1,
                        width: 1,
                        height: 1,
                    },
                );
            }

            // Show scrollbar in the middle
            let scrollbar_height = chunks[1].height.saturating_sub(2);
            if scrollbar_height > 0 {
                let scroll_progress = if max_scroll > 0 {
                    state.frog_scroll as f64 / max_scroll as f64
                } else {
                    0.0
                };
                let thumb_pos = (scroll_progress * (scrollbar_height as f64 - 1.0)) as u16;

                for i in 0..scrollbar_height {
                    let char = if i == thumb_pos { "█" } else { "░" };
                    let color = if i == thumb_pos {
                        theme::colors::ACCENT
                    } else {
                        theme::colors::BG_LIGHT
                    };
                    frame.render_widget(
                        Paragraph::new(char).style(Style::default().fg(color)),
                        Rect {
                            x: indicator_x,
                            y: chunks[1].y + 1 + i,
                            width: 1,
                            height: 1,
                        },
                    );
                }
            }
        }
    }
}

fn render_markdown(text: &str) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let mut in_code_block = false;

    for line in text.lines() {
        let trimmed = line.trim_end();

        // Toggle code block state
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            // Don't render the ``` markers themselves
            continue;
        }

        // Inside code block - render with syntax highlighting
        if in_code_block {
            let highlighted = highlight_rust_line(line);
            lines.push(Line::from(highlighted));
            continue;
        }

        // Horizontal rule
        if trimmed.starts_with("---") && trimmed.chars().all(|c| c == '-' || c.is_whitespace()) {
            lines.push(Line::from(vec![Span::styled(
                "─".repeat(40),
                Style::default().fg(theme::colors::BG_LIGHT),
            )]));
            continue;
        }

        // H1 header
        if trimmed.starts_with("# ") {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![Span::styled(
                trimmed[2..].to_string(),
                Style::default()
                    .fg(theme::colors::ACCENT)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )]));
            lines.push(Line::from(""));
            continue;
        }

        // H2 header
        if trimmed.starts_with("## ") {
            lines.push(Line::from(vec![Span::styled(
                trimmed[3..].to_string(),
                Style::default()
                    .fg(theme::colors::PRIMARY)
                    .add_modifier(Modifier::BOLD),
            )]));
            lines.push(Line::from(""));
            continue;
        }

        // H3 header
        if trimmed.starts_with("### ") {
            lines.push(Line::from(vec![Span::styled(
                trimmed[4..].to_string(),
                Style::default()
                    .fg(theme::colors::INFO)
                    .add_modifier(Modifier::BOLD),
            )]));
            continue;
        }

        // Table row (starts with |)
        if trimmed.starts_with('|') {
            // Check if it's a separator row (|---|---|)
            if trimmed.contains("---") {
                lines.push(Line::from(vec![Span::styled(
                    trimmed.replace("-", "─").replace("|", "│"),
                    Style::default().fg(theme::colors::BG_LIGHT),
                )]));
            } else {
                // Regular table row
                lines.push(Line::from(vec![Span::styled(
                    trimmed.replace("|", "│"),
                    Style::default().fg(theme::colors::TEXT),
                )]));
            }
            continue;
        }

        // Bullet list
        if trimmed.starts_with("- ") {
            lines.push(Line::from(vec![
                Span::styled("  • ", Style::default().fg(theme::colors::INFO)),
                Span::raw(render_inline_formatting(&trimmed[2..])),
            ]));
            continue;
        }

        // Box drawing characters - render with special color
        if trimmed.contains('┌')
            || trimmed.contains('│')
            || trimmed.contains('└')
            || trimmed.contains('├')
            || trimmed.contains('╭')
            || trimmed.contains('╰')
        {
            lines.push(Line::from(vec![Span::styled(
                trimmed.to_string(),
                Style::default().fg(theme::colors::INFO),
            )]));
            continue;
        }

        // Regular text with inline formatting
        let formatted = render_inline_formatting(trimmed);
        lines.push(Line::from(formatted));
    }

    lines
}

/// Handle inline formatting like **bold** and `code`
fn render_inline_formatting(text: &str) -> String {
    // For now, just strip ** markers for bold (can't easily style inline in ratatui)
    // and keep `code` as-is
    text.replace("**", "").to_string()
}

/// Basic Rust syntax highlighting for code blocks
fn highlight_rust_line(line: &str) -> Vec<Span<'static>> {
    let mut spans = Vec::new();

    // Add leading indent
    spans.push(Span::raw("  ".to_string()));

    // Check for comment first (entire line is comment)
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
                // Flush current token
                if !current_token.is_empty() {
                    spans.push(style_token(&current_token));
                    current_token.clear();
                }
                // Collect string
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
                // Flush current token
                if !current_token.is_empty() {
                    spans.push(style_token(&current_token));
                    current_token.clear();
                }
                // Check for -> arrow
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

    // Flush remaining token
    if !current_token.is_empty() {
        spans.push(style_token(&current_token));
    }

    spans
}

/// Style a token based on whether it's a keyword, type, or identifier
fn style_token(token: &str) -> Span<'static> {
    // Rust keywords
    const KEYWORDS: &[&str] = &[
        "fn", "let", "mut", "const", "if", "else", "match", "loop", "while", "for", "in", "return",
        "break", "continue", "struct", "enum", "impl", "trait", "pub", "mod", "use", "self",
        "Self", "super", "crate", "where", "async", "await", "move", "ref", "static", "type",
        "unsafe", "extern", "dyn", "as",
    ];

    // Rust types
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
        // Numbers
        Span::styled(
            token.to_string(),
            Style::default().fg(theme::colors::WARNING),
        )
    } else if token.starts_with(char::is_uppercase) {
        // Types (PascalCase)
        Span::styled(
            token.to_string(),
            Style::default().fg(theme::colors::ACCENT),
        )
    } else {
        // Regular identifier
        Span::styled(token.to_string(), Style::default().fg(theme::colors::TEXT))
    }
}
