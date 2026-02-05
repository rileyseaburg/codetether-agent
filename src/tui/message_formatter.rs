use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

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
                        lines.extend(self.render_code_block(&code_block_lines, &code_block_language));
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
            Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
        )));

        // Syntax highlighting for code
        for line in lines {
            let highlighted_line = self.highlight_code_line(line, language);
            let formatted_line = if line.trim().is_empty() {
                "│".to_string()
            } else {
                format!("│ {}", highlighted_line)
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

    /// Basic syntax highlighting for common languages
    fn highlight_code_line(&self, line: &str, language: &str) -> String {
        let lang = language.to_lowercase();
        match lang.as_str() {
            "rust" => self.highlight_rust(line),
            "python" | "py" => self.highlight_python(line),
            "javascript" | "js" | "typescript" | "ts" => self.highlight_javascript(line),
            "json" => self.highlight_json(line),
            "yaml" | "yml" => self.highlight_yaml(line),
            "toml" => self.highlight_toml(line),
            "bash" | "sh" | "shell" => self.highlight_shell(line),
            "sql" => self.highlight_sql(line),
            "markdown" | "md" => self.highlight_markdown(line),
            _ => line.to_string(), // No highlighting for unknown languages
        }
    }

    /// Basic Rust syntax highlighting
    fn highlight_rust(&self, line: &str) -> String {
        let mut result = String::new();
        let keywords = [
            "fn", "let", "mut", "pub", "use", "mod", "struct", "enum", "trait", "impl",
            "const", "static", "if", "else", "match", "for", "while", "loop", "return",
            "break", "continue", "unsafe", "async", "await", "dyn", "where", "type",
        ];
        let types = ["i32", "i64", "u32", "u64", "f32", "f64", "bool", "char", "str", "String", "Vec", "Option", "Result"];
        
        let words: Vec<&str> = line.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            let highlighted = if keywords.contains(word) {
                format!("\x1b[38;5;197m{word}\x1b[0m") // Magenta for keywords
            } else if types.contains(word) {
                format!("\x1b[38;5;81m{word}\x1b[0m") // Cyan for types
            } else if word.starts_with('"') && word.ends_with('"') {
                format!("\x1b[38;5;113m{word}\x1b[0m") // Green for strings
            } else if word.starts_with('\'') && word.ends_with('\'') {
                format!("\x1b[38;5;221m{word}\x1b[0m") // Yellow for char literals
            } else if word.starts_with("//") {
                format!("\x1b[38;5;242m{word}\x1b[0m") // Gray for comments
            } else {
                word.to_string()
            };
            
            if i > 0 {
                result.push(' ');
            }
            result.push_str(&highlighted);
        }
        
        // Handle non-whitespace parts
        if result.is_empty() {
            line.to_string()
        } else {
            result
        }
    }

    /// Basic Python syntax highlighting
    fn highlight_python(&self, line: &str) -> String {
        let mut result = String::new();
        let keywords = [
            "def", "class", "import", "from", "as", "if", "elif", "else", "for", "while",
            "try", "except", "finally", "with", "return", "yield", "lambda", "pass", "break",
            "continue", "True", "False", "None", "self", "super", "global", "nonlocal",
        ];
        
        let words: Vec<&str> = line.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            let highlighted = if keywords.contains(word) {
                format!("\x1b[38;5;197m{word}\x1b[0m")
            } else if word.starts_with('"') && word.ends_with('"') || word.starts_with('\'') && word.ends_with('\'') {
                format!("\x1b[38;5;113m{word}\x1b[0m")
            } else if word.starts_with("#") {
                format!("\x1b[38;5;242m{word}\x1b[0m")
            } else {
                word.to_string()
            };
            
            if i > 0 {
                result.push(' ');
            }
            result.push_str(&highlighted);
        }
        
        if result.is_empty() { line.to_string() } else { result }
    }

    /// JavaScript/TypeScript syntax highlighting
    fn highlight_javascript(&self, line: &str) -> String {
        let mut result = String::new();
        let keywords = [
            "function", "const", "let", "var", "if", "else", "for", "while", "do", "switch",
            "case", "break", "continue", "return", "class", "extends", "import", "export",
            "async", "await", "try", "catch", "finally", "typeof", "instanceof", "new", "this",
        ];
        
        let words: Vec<&str> = line.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            let highlighted = if keywords.contains(word) {
                format!("\x1b[38;5;197m{word}\x1b[0m")
            } else if word.starts_with('"') && word.ends_with('"') || word.starts_with('\'') && word.ends_with('\'') {
                format!("\x1b[38;5;113m{word}\x1b[0m")
            } else if word.starts_with("//") {
                format!("\x1b[38;5;242m{word}\x1b[0m")
            } else {
                word.to_string()
            };
            
            if i > 0 {
                result.push(' ');
            }
            result.push_str(&highlighted);
        }
        
        if result.is_empty() { line.to_string() } else { result }
    }

    /// JSON syntax highlighting
    fn highlight_json(&self, line: &str) -> String {
        let line = line.trim();
        let highlighted = line
            .replace("\"", "\x1b[38;5;113m\"\x1b[0m")
            .replace(":", "\x1b[38;5;197m:\x1b[0m")
            .replace("{", "\x1b[38;5;197m{\x1b[0m")
            .replace("}", "\x1b[38;5;197m}\x1b[0m")
            .replace("[", "\x1b[38;5;197m[\x1b[0m")
            .replace("]", "\x1b[38;5;197m]\x1b[0m");
        highlighted
    }

    /// YAML syntax highlighting
    fn highlight_yaml(&self, line: &str) -> String {
        let line = line.trim();
        if line.contains(':') {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                return format!("\x1b[38;5;197m{}:\x1b[0m\x1b[38;5;113m{}\x1b[0m", parts[0], parts[1]);
            }
        }
        line.to_string()
    }

    /// TOML syntax highlighting
    fn highlight_toml(&self, line: &str) -> String {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            return format!("\x1b[38;5;197m{}\x1b[0m", line);
        } else if line.contains('=') {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                return format!("\x1b[38;5;197m{}=\x1b[0m\x1b[38;5;113m{}\x1b[0m", parts[0], parts[1]);
            }
        }
        line.to_string()
    }

    /// Shell syntax highlighting
    fn highlight_shell(&self, line: &str) -> String {
        let line = line.trim();
        if line.starts_with('#') {
            format!("\x1b[38;5;242m{}\x1b[0m", line)
        } else if line.starts_with('$') || line.starts_with('>') {
            format!("\x1b[38;5;197m{}\x1b[0m", line)
        } else {
            line.to_string()
        }
    }

    /// SQL syntax highlighting
    fn highlight_sql(&self, line: &str) -> String {
        let mut result = String::new();
        let keywords = [
            "SELECT", "FROM", "WHERE", "INSERT", "UPDATE", "DELETE", "CREATE", "DROP",
            "ALTER", "TABLE", "INDEX", "JOIN", "INNER", "LEFT", "RIGHT", "FULL", "ON",
            "AND", "OR", "NOT", "NULL", "IS", "LIKE", "IN", "EXISTS", "BETWEEN", "ORDER BY",
            "GROUP BY", "HAVING", "LIMIT", "OFFSET", "ASC", "DESC",
        ];
        
        let words: Vec<&str> = line.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            let upper_word = word.to_uppercase();
            let highlighted = if keywords.contains(&upper_word.as_str()) {
                format!("\x1b[38;5;197m{word}\x1b[0m")
            } else if word.starts_with('"') && word.ends_with('"') || word.starts_with('\'') && word.ends_with('\'') {
                format!("\x1b[38;5;113m{word}\x1b[0m")
            } else if word.starts_with("--") {
                format!("\x1b[38;5;242m{word}\x1b[0m")
            } else {
                word.to_string()
            };
            
            if i > 0 {
                result.push(' ');
            }
            result.push_str(&highlighted);
        }
        
        if result.is_empty() { line.to_string() } else { result }
    }

    /// Markdown syntax highlighting
    fn highlight_markdown(&self, line: &str) -> String {
        let line = line.trim();
        if line.starts_with("#") {
            format!("\x1b[38;5;197m{}\x1b[0m", line)
        } else if line.starts_with("```") {
            format!("\x1b[38;5;242m{}\x1b[0m", line)
        } else if line.starts_with("*") || line.starts_with("-") {
            format!("\x1b[38;5;113m{}\x1b[0m", line)
        } else {
            line.to_string()
        }
    }

    /// Format inline text with markdown-like features
    fn format_inline_text(&self, text: &str, role: &str) -> String {
        let mut result = text.to_string();
        
        // Bold text **text**
        result = result.replace("**", "\x1b[1m");
        
        // Italic text *text*
        result = result.replace("*", "\x1b[3m");
        
        // Inline code `code`
        result = result.replace('`', "\x1b[7m");
        
        result
    }

    /// Wrap a line to fit within the given width
    fn wrap_line(&self, text: String, max_width: usize) -> Vec<Line<'static>> {
        if text.is_empty() {
            return vec![Line::from("")];
        }

        let mut lines = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut current_line = String::new();

        for word in words {
            let word_len = word.len();
            let line_len = current_line.len();
            
            if line_len + word_len + (if line_len > 0 { 1 } else { 0 }) <= max_width {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            } else {
                if !current_line.is_empty() {
                    lines.push(Line::from(current_line.clone()));
                    current_line.clear();
                }
                
                // Handle very long words
                if word_len > max_width {
                    let mut start = 0;
                    while start < word_len {
                        let end = (start + max_width).min(word_len);
                        let chunk = &word[start..end];
                        lines.push(Line::from(chunk.to_string()));
                        start = end;
                    }
                } else {
                    current_line.push_str(word);
                }
            }
        }

        if !current_line.is_empty() {
            lines.push(Line::from(current_line));
        }

        lines
    }
}