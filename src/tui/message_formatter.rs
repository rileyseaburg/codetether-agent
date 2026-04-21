use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Enhanced message formatter with syntax highlighting and improved styling
pub struct MessageFormatter {
    max_width: usize,
}

impl MessageFormatter {
    pub fn new(max_width: usize) -> Self {
        Self { max_width }
    }

    /// Configured maximum wrap width for this formatter.
    pub fn max_width(&self) -> usize {
        self.max_width
    }

    /// Format message content with enhanced features
    pub fn format_content(&self, content: &str, role: &str) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        let mut in_code_block = false;
        let mut code_block_start = false;
        let mut code_block_language = String::new();
        let mut code_block_lines = Vec::new();

        for line in content.lines() {
            // Detect code blocks
            if line.trim().starts_with("```") {
                if in_code_block {
                    // End of code block - render with syntax highlighting
                    if !code_block_lines.is_empty() {
                        lines.extend(
                            self.render_code_block(&code_block_lines, &code_block_language),
                        );
                        code_block_lines.clear();
                        code_block_language.clear();
                    }
                    in_code_block = false;
                    code_block_start = false;
                } else {
                    // Start of code block - extract language
                    in_code_block = true;
                    code_block_start = true;
                    let lang = line.trim().trim_start_matches('`').trim();
                    code_block_language = lang.to_string();
                }
                continue;
            }

            if in_code_block {
                if code_block_start {
                    // First line after opening ``` might be language specifier
                    code_block_start = false;
                    if !line.trim().is_empty() && code_block_language.is_empty() {
                        code_block_language = line.trim().to_string();
                    } else {
                        code_block_lines.push(line.to_string());
                    }
                } else {
                    code_block_lines.push(line.to_string());
                }
                continue;
            }

            // Handle regular text with enhanced formatting
            if line.trim().is_empty() {
                lines.push(Line::from(""));
                continue;
            }

            // Handle markdown-like formatting
            let formatted_line = self.format_inline_text(line, role);
            lines.extend(self.wrap_line(formatted_line, self.max_width.saturating_sub(4)));
        }

        // Handle unclosed code blocks
        if !code_block_lines.is_empty() {
            lines.extend(self.render_code_block(&code_block_lines, &code_block_language));
        }

        if lines.is_empty() {
            lines.push(Line::from(""));
        }

        lines
    }

    /// Format an image as a simple placeholder line
    pub fn format_image(&self, url: &str, _mime_type: Option<&str>) -> Line<'static> {
        // Extract filename from URL for display
        let filename = url
            .split('/')
            .next_back()
            .unwrap_or("image")
            .split('?')
            .next()
            .unwrap_or("image");

        Line::from(vec![
            Span::styled("  🖼️  ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("[Image: {}]", filename),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::ITALIC),
            ),
        ])
    }

    /// Render a code block with syntax highlighting and styling
    fn render_code_block(&self, lines: &[String], language: &str) -> Vec<Line<'static>> {
        let mut result = Vec::new();
        let block_width = self.max_width.saturating_sub(4);

        // Header with language indicator
        let header = if language.is_empty() {
            "┌─ Code ─".to_string() + &"─".repeat(block_width.saturating_sub(9))
        } else {
            let lang_header = format!("┌─ {} Code ─", language);
            let header_len = lang_header.len();
            lang_header + &"─".repeat(block_width.saturating_sub(header_len))
        };

        result.push(Line::from(Span::styled(
            header,
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )));

        // Pass through code lines as-is
        let highlighted_lines = self.highlight_code_block_syntect(lines, language);

        for line in highlighted_lines {
            let formatted_line = if line.trim().is_empty() {
                "│".to_string()
            } else {
                format!("│ {}", line)
            };

            result.push(Line::from(Span::styled(
                formatted_line,
                Style::default().fg(Color::DarkGray),
            )));
        }

        result.push(Line::from(Span::styled(
            "└".to_string() + &"─".repeat(block_width.saturating_sub(1)),
            Style::default().fg(Color::DarkGray),
        )));

        result
    }

    fn highlight_code_block_syntect(&self, lines: &[String], _language: &str) -> Vec<String> {
        lines.iter().map(|l| l.trim_end().to_string()).collect()
    }

    /// Format inline text with basic markdown-like formatting
    fn format_inline_text(&self, line: &str, role: &str) -> Vec<Span<'static>> {
        let mut spans = Vec::new();
        let mut current = String::new();
        let mut in_bold = false;
        let mut in_italic = false;
        let mut in_code = false;

        let role_color = match role {
            "user" => Color::White,
            "assistant" => Color::Cyan,
            "system" => Color::Yellow,
            "tool" => Color::Green,
            _ => Color::White,
        };

        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '*' => {
                    if chars.peek() == Some(&'*') {
                        // Bold
                        if !current.is_empty() {
                            spans.push(Span::styled(
                                current.clone(),
                                Style::default().fg(role_color).add_modifier(if in_bold {
                                    Modifier::BOLD
                                } else {
                                    Modifier::empty()
                                }),
                            ));
                            current.clear();
                        }
                        chars.next(); // consume second '*'
                        in_bold = !in_bold;
                    } else {
                        // Italic
                        if !current.is_empty() {
                            spans.push(Span::styled(
                                current.clone(),
                                Style::default().fg(role_color).add_modifier(if in_italic {
                                    Modifier::ITALIC
                                } else {
                                    Modifier::empty()
                                }),
                            ));
                            current.clear();
                        }
                        in_italic = !in_italic;
                    }
                }
                '`' => {
                    if !current.is_empty() {
                        spans.push(Span::styled(
                            current.clone(),
                            Style::default().fg(role_color),
                        ));
                        current.clear();
                    }
                    in_code = !in_code;
                }
                _ => {
                    current.push(c);
                }
            }
        }

        if !current.is_empty() {
            spans.push(Span::styled(current, Style::default().fg(role_color)));
        }

        if spans.is_empty() {
            spans.push(Span::styled(
                line.to_string(),
                Style::default().fg(role_color),
            ));
        }

        spans
    }

    /// Greedy word-wrap a list of styled spans to `width` display columns.
    ///
    /// Preserves per-span [`Style`] as content splits across rows. Breaks on
    /// whitespace when possible; for overlong tokens (URLs, code without
    /// spaces) falls back to a hard char boundary. Uses [`UnicodeWidthStr`]
    /// for display width so CJK and emoji count correctly.
    ///
    /// # Arguments
    ///
    /// * `spans` — styled input spans for a single logical line.
    /// * `width` — target column width (display columns, not bytes).
    ///
    /// # Returns
    ///
    /// One or more [`Line<'static>`] values whose combined content equals
    /// the input (modulo whitespace collapsed at wrap points) and each of
    /// which has display width `<= width`. If `spans` is empty, returns a
    /// single empty line. If `width == 0`, returns the input unsplit.
    ///
    /// Invoked via [`MessageFormatter::format_content`]; tested indirectly
    /// by the unit tests in this module.
    fn wrap_line(&self, spans: Vec<Span<'static>>, width: usize) -> Vec<Line<'static>> {
        if spans.is_empty() {
            return vec![Line::from("")];
        }
        if width == 0 {
            return vec![Line::from(spans)];
        }

        let mut out: Vec<Line<'static>> = Vec::new();
        let mut cur: Vec<Span<'static>> = Vec::new();
        let mut cur_w: usize = 0;

        for span in spans {
            let style = span.style;
            let mut text = span.content.into_owned();
            while !text.is_empty() {
                let remaining = width.saturating_sub(cur_w);
                if remaining == 0 {
                    out.push(Line::from(std::mem::take(&mut cur)));
                    cur_w = 0;
                    continue;
                }
                let (taken, rest) = take_fit(&text, remaining, cur_w == 0);
                if taken.is_empty() {
                    // nothing fits on this row; flush and retry at col 0.
                    out.push(Line::from(std::mem::take(&mut cur)));
                    cur_w = 0;
                    continue;
                }
                cur_w += UnicodeWidthStr::width(taken.as_str());
                cur.push(Span::styled(taken, style));
                text = rest;
                if !text.is_empty() {
                    out.push(Line::from(std::mem::take(&mut cur)));
                    cur_w = 0;
                }
            }
        }
        if !cur.is_empty() {
            out.push(Line::from(cur));
        }
        if out.is_empty() {
            out.push(Line::from(""));
        }
        out
    }
}

/// Take the longest prefix of `text` whose display width fits in `width`.
///
/// Prefers breaking after the last whitespace inside the fitting prefix.
/// When no whitespace is available (e.g. a long URL), falls back to a hard
/// char-boundary split at the last character that still fits.
///
/// # Arguments
///
/// * `text` — UTF-8 input, possibly wider than `width`.
/// * `width` — maximum display columns the returned `taken` may occupy.
/// * `at_start` — if `true`, leading whitespace is trimmed before measuring
///   so wrapped continuation rows don't begin with a space.
///
/// # Returns
///
/// Tuple `(taken, rest)` where `taken` fits in `width` columns and
/// `rest` is the remainder to wrap onto following rows. If the whole
/// input fits, `rest` is empty.
fn take_fit(text: &str, width: usize, at_start: bool) -> (String, String) {
    let trimmed = if at_start { text.trim_start() } else { text };
    let mut end_byte = 0usize;
    let mut last_ws_byte: Option<usize> = None;
    let mut w: usize = 0;
    for (i, ch) in trimmed.char_indices() {
        let cw = UnicodeWidthChar::width(ch).unwrap_or(0);
        if w + cw > width {
            break;
        }
        w += cw;
        end_byte = i + ch.len_utf8();
        if ch.is_whitespace() {
            last_ws_byte = Some(end_byte);
        }
    }
    if end_byte == trimmed.len() {
        return (trimmed.to_string(), String::new());
    }
    let split = last_ws_byte.unwrap_or(end_byte).max(1).min(trimmed.len());
    let taken = trimmed[..split].trim_end().to_string();
    let rest = trimmed[split..].to_string();
    (taken, rest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_block_detection() {
        let formatter = MessageFormatter::new(80);
        let content = "```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```";
        let lines = formatter.format_content(content, "assistant");
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_syntax_highlighting() {
        let formatter = MessageFormatter::new(80);
        let lines = vec![
            "fn main() {".to_string(),
            "    println!(\"Hello!\");".to_string(),
            "}".to_string(),
        ];
        let highlighted = formatter.highlight_code_block_syntect(&lines, "rust");
        assert_eq!(highlighted.len(), 3);
    }
    #[test]
    fn take_fit_breaks_on_whitespace() {
        let (taken, rest) = take_fit("hello world foo", 8, true);
        assert_eq!(taken, "hello");
        assert_eq!(rest, "world foo");
    }

    #[test]
    fn take_fit_hard_breaks_long_token() {
        let (taken, rest) = take_fit("abcdefghij", 4, true);
        assert_eq!(taken, "abcd");
        assert_eq!(rest, "efghij");
    }

    #[test]
    fn take_fit_trims_leading_ws_at_start() {
        let (taken, rest) = take_fit("   hello", 8, true);
        assert_eq!(taken, "hello");
        assert!(rest.is_empty());
    }

    #[test]
    fn take_fit_whole_input_fits() {
        let (taken, rest) = take_fit("short", 10, true);
        assert_eq!(taken, "short");
        assert!(rest.is_empty());
    }

    #[test]
    fn wrap_line_empty_returns_single_blank() {
        let f = MessageFormatter::new(20);
        let out = f.wrap_line(vec![], 16);
        assert_eq!(out.len(), 1);
    }

    #[test]
    fn wrap_line_splits_at_whitespace() {
        let f = MessageFormatter::new(20);
        let spans = vec![Span::raw("hello world foo bar")];
        let out = f.wrap_line(spans, 10);
        assert!(out.len() >= 2);
        for line in &out {
            assert!(line.width() <= 10, "line too wide: {}", line.width());
        }
    }

    #[test]
    fn wrap_line_preserves_style_across_wraps() {
        let f = MessageFormatter::new(20);
        let styled = Style::default().add_modifier(Modifier::BOLD);
        let spans = vec![Span::styled("alpha beta gamma delta", styled)];
        let out = f.wrap_line(spans, 10);
        for line in &out {
            for span in &line.spans {
                assert_eq!(span.style, styled);
            }
        }
    }

    #[test]
    fn wrap_line_width_zero_is_noop() {
        let f = MessageFormatter::new(20);
        let spans = vec![Span::raw("anything")];
        let out = f.wrap_line(spans, 0);
        assert_eq!(out.len(), 1);
    }}
