use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use std::sync::LazyLock;
use syntect::{
    easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet, util::LinesWithEndings,
};

/// Global syntax set and theme set for syntax highlighting
static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);

/// Enhanced message formatter with syntax highlighting and improved styling
pub struct MessageFormatter {
    max_width: usize,
}

impl MessageFormatter {
    pub fn new(max_width: usize) -> Self {
        Self { max_width }
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

        // Use syntect for syntax highlighting
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

    /// Advanced syntax highlighting using syntect
    fn highlight_code_block_syntect(&self, lines: &[String], language: &str) -> Vec<String> {
        let syntax_set = &*SYNTAX_SET;
        let theme_set = &*THEME_SET;

        // Use a dark theme suitable for terminal
        let theme = &theme_set.themes["base16-ocean.dark"];

        // Find the appropriate syntax
        let syntax = if language.is_empty() {
            syntax_set.find_syntax_plain_text()
        } else {
            syntax_set
                .find_syntax_by_token(language)
                .unwrap_or_else(|| syntax_set.find_syntax_plain_text())
        };

        let mut highlighter = HighlightLines::new(syntax, theme);

        let mut highlighted_lines = Vec::new();
        let code = lines.join("\n");

        for line in LinesWithEndings::from(&code) {
            let ranges = match highlighter.highlight_line(line, syntax_set) {
                Ok(r) => r,
                Err(_) => {
                    // On error, just return plain text
                    highlighted_lines.push(line.trim_end().to_string());
                    continue;
                }
            };
            let mut line_result = String::new();

            for (style, text) in ranges {
                let fg_color = style.foreground;
                let _color = Color::Rgb(fg_color.r, fg_color.g, fg_color.b);

                // Convert the styled text to a ratatui-compatible string
                // Since we can't use actual terminal colors in ratatui spans easily,
                // we'll use the closest ratatui colors
                let _ratatui_color = self.map_syntect_color_to_ratatui(&fg_color);

                // For now, we'll just return the text without ANSI codes
                // since ratatui handles styling through its own Span system
                line_result.push_str(text);
            }

            highlighted_lines.push(line_result.trim_end().to_string());
        }

        highlighted_lines
    }

    /// Map syntect colors to ratatui colors (simplified for now)
    fn map_syntect_color_to_ratatui(&self, color: &syntect::highlighting::Color) -> Color {
        // Simple mapping - we'll use the closest ratatui color
        Color::Rgb(color.r, color.g, color.b)
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

    /// Wrap text to fit within width
    fn wrap_line(&self, spans: Vec<Span<'static>>, _width: usize) -> Vec<Line<'static>> {
        if spans.is_empty() {
            return vec![Line::from("")];
        }

        // Simple wrapping - for now, just return as single line
        vec![Line::from(spans)]
    }
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
}
