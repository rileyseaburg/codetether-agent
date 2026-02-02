//! Semantic chunking for large contexts
//!
//! Splits content intelligently at natural boundaries and prioritizes
//! chunks for token budget selection.

use serde::{Deserialize, Serialize};

/// Content type for optimized processing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Code,
    Documents,
    Logs,
    Conversation,
    Mixed,
}

/// A chunk of content with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub content: String,
    #[serde(rename = "type")]
    pub chunk_type: ChunkType,
    pub start_line: usize,
    pub end_line: usize,
    pub tokens: usize,
    /// Higher = more important to keep
    pub priority: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChunkType {
    Code,
    Text,
    ToolOutput,
    Conversation,
}

/// Options for chunking
#[derive(Debug, Clone)]
pub struct ChunkOptions {
    /// Maximum tokens per chunk
    pub max_chunk_tokens: usize,
    /// Number of recent lines to always preserve
    pub preserve_recent: usize,
}

impl Default for ChunkOptions {
    fn default() -> Self {
        Self {
            max_chunk_tokens: 4000,
            preserve_recent: 100,
        }
    }
}

/// Semantic chunker for large contexts
pub struct RlmChunker;

impl RlmChunker {
    /// Detect the primary type of content for optimized processing
    pub fn detect_content_type(content: &str) -> ContentType {
        let lines: Vec<&str> = content.lines().collect();
        let sample_size = lines.len().min(200);
        
        // Sample from head and tail
        let sample: Vec<&str> = lines.iter()
            .take(sample_size / 2)
            .chain(lines.iter().rev().take(sample_size / 2))
            .copied()
            .collect();

        let mut code_indicators = 0;
        let mut log_indicators = 0;
        let mut conversation_indicators = 0;
        let mut document_indicators = 0;

        for line in &sample {
            let trimmed = line.trim();

            // Code indicators
            if Self::is_code_line(trimmed) {
                code_indicators += 1;
            }

            // Log indicators
            if Self::is_log_line(trimmed) {
                log_indicators += 1;
            }

            // Conversation indicators
            if Self::is_conversation_line(trimmed) {
                conversation_indicators += 1;
            }

            // Document indicators
            if Self::is_document_line(trimmed) {
                document_indicators += 1;
            }
        }

        let total = code_indicators + log_indicators + conversation_indicators + document_indicators;
        if total == 0 {
            return ContentType::Mixed;
        }

        let threshold = (total as f64 * 0.3) as usize;

        if conversation_indicators > threshold {
            ContentType::Conversation
        } else if log_indicators > threshold {
            ContentType::Logs
        } else if code_indicators > threshold {
            ContentType::Code
        } else if document_indicators > threshold {
            ContentType::Documents
        } else {
            ContentType::Mixed
        }
    }

    fn is_code_line(line: &str) -> bool {
        // Function/class/import definitions
        let patterns = [
            "function", "class ", "def ", "const ", "let ", "var ",
            "import ", "export ", "async ", "fn ", "impl ", "struct ",
            "enum ", "pub ", "use ", "mod ", "trait ",
        ];
        
        if patterns.iter().any(|p| line.starts_with(p)) {
            return true;
        }

        // Brace-only or semicolon-only lines
        if matches!(line, "{" | "}" | "(" | ")" | ";" | "{}" | "};") {
            return true;
        }

        // Comment lines
        if line.starts_with("//") || line.starts_with("#") || 
           line.starts_with("*") || line.starts_with("/*") {
            return true;
        }

        false
    }

    fn is_log_line(line: &str) -> bool {
        // ISO date prefix
        if line.len() >= 10 && line.chars().take(4).all(|c| c.is_ascii_digit()) 
            && line.chars().nth(4) == Some('-') {
            return true;
        }

        // Time prefix [HH:MM
        if line.starts_with('[') && line.len() > 5 
            && line.chars().nth(1).map_or(false, |c| c.is_ascii_digit()) {
            return true;
        }

        // Log level prefixes
        let log_levels = ["INFO", "DEBUG", "WARN", "ERROR", "FATAL", "TRACE"];
        for level in log_levels {
            if line.starts_with(level) || line.contains(&format!(" {} ", level)) {
                return true;
            }
        }

        false
    }

    fn is_conversation_line(line: &str) -> bool {
        let patterns = [
            "[User]:", "[Assistant]:", "[Human]:", "[AI]:",
            "User:", "Assistant:", "Human:", "AI:",
            "[Tool ", "<user>", "<assistant>", "<system>",
        ];
        patterns.iter().any(|p| line.starts_with(p))
    }

    fn is_document_line(line: &str) -> bool {
        // Markdown headers
        if line.starts_with('#') && line.chars().nth(1).map_or(false, |c| c == ' ' || c == '#') {
            return true;
        }

        // Bold text
        if line.starts_with("**") && line.contains("**") {
            return true;
        }

        // Blockquotes
        if line.starts_with("> ") {
            return true;
        }

        // List items
        if line.starts_with("- ") && line.len() > 3 {
            return true;
        }

        // Long prose lines without code terminators
        if line.len() > 80 && !line.ends_with('{') && !line.ends_with(';') 
            && !line.ends_with('(') && !line.ends_with(')') && !line.ends_with('=') {
            return true;
        }

        false
    }

    /// Get processing hints based on content type
    pub fn get_processing_hints(content_type: ContentType) -> &'static str {
        match content_type {
            ContentType::Code => {
                "This appears to be source code. Focus on:\n\
                 - Function/class definitions and their purposes\n\
                 - Import statements and dependencies\n\
                 - Error handling patterns\n\
                 - Key algorithms and logic flow"
            }
            ContentType::Logs => {
                "This appears to be log output. Focus on:\n\
                 - Error and warning messages\n\
                 - Timestamps and event sequences\n\
                 - Stack traces and exceptions\n\
                 - Key events and state changes"
            }
            ContentType::Conversation => {
                "This appears to be conversation history. Focus on:\n\
                 - User's original request/goal\n\
                 - Key decisions made\n\
                 - Tool calls and their results\n\
                 - Current state and pending tasks"
            }
            ContentType::Documents => {
                "This appears to be documentation or prose. Focus on:\n\
                 - Main topics and structure\n\
                 - Key information and facts\n\
                 - Actionable items\n\
                 - References and links"
            }
            ContentType::Mixed => {
                "Mixed content detected. Analyze the structure first, then extract key information."
            }
        }
    }

    /// Estimate token count (roughly 4 chars per token)
    pub fn estimate_tokens(text: &str) -> usize {
        (text.len() + 3) / 4
    }

    /// Split content into semantic chunks
    pub fn chunk(content: &str, options: Option<ChunkOptions>) -> Vec<Chunk> {
        let opts = options.unwrap_or_default();
        let lines: Vec<&str> = content.lines().collect();
        let mut chunks = Vec::new();

        // Find semantic boundaries
        let boundaries = Self::find_boundaries(&lines);

        let mut current_chunk: Vec<&str> = Vec::new();
        let mut current_type = ChunkType::Text;
        let mut current_start = 0;
        let mut current_priority: u8 = 1;

        for (i, line) in lines.iter().enumerate() {
            // Check if we hit a boundary
            if let Some((boundary_type, boundary_priority)) = boundaries.get(&i) {
                if !current_chunk.is_empty() {
                    let content = current_chunk.join("\n");
                    let tokens = Self::estimate_tokens(&content);

                    // If chunk is too big, split it
                    if tokens > opts.max_chunk_tokens {
                        let sub_chunks = Self::split_large_chunk(
                            &current_chunk, current_start, current_type, opts.max_chunk_tokens
                        );
                        chunks.extend(sub_chunks);
                    } else {
                        chunks.push(Chunk {
                            content,
                            chunk_type: current_type,
                            start_line: current_start,
                            end_line: i.saturating_sub(1),
                            tokens,
                            priority: current_priority,
                        });
                    }

                    current_chunk = Vec::new();
                    current_start = i;
                    current_type = *boundary_type;
                    current_priority = *boundary_priority;
                }
            }

            current_chunk.push(line);

            // Boost priority for recent lines
            if i >= lines.len().saturating_sub(opts.preserve_recent) {
                current_priority = current_priority.max(8);
            }
        }

        // Final chunk
        if !current_chunk.is_empty() {
            let content = current_chunk.join("\n");
            let tokens = Self::estimate_tokens(&content);

            if tokens > opts.max_chunk_tokens {
                let sub_chunks = Self::split_large_chunk(
                    &current_chunk, current_start, current_type, opts.max_chunk_tokens
                );
                chunks.extend(sub_chunks);
            } else {
                chunks.push(Chunk {
                    content,
                    chunk_type: current_type,
                    start_line: current_start,
                    end_line: lines.len().saturating_sub(1),
                    tokens,
                    priority: current_priority,
                });
            }
        }

        chunks
    }

    /// Find semantic boundaries in content
    fn find_boundaries(lines: &[&str]) -> std::collections::HashMap<usize, (ChunkType, u8)> {
        let mut boundaries = std::collections::HashMap::new();

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // User/Assistant message markers
            if trimmed.starts_with("[User]:") || trimmed.starts_with("[Assistant]:") {
                boundaries.insert(i, (ChunkType::Conversation, 5));
                continue;
            }

            // Tool output markers
            if trimmed.starts_with("[Tool ") {
                let priority = if trimmed.contains("FAILED") || trimmed.contains("error") { 7 } else { 3 };
                boundaries.insert(i, (ChunkType::ToolOutput, priority));
                continue;
            }

            // Code block markers
            if trimmed.starts_with("```") {
                boundaries.insert(i, (ChunkType::Code, 4));
                continue;
            }

            // File path markers
            if trimmed.starts_with('/') || trimmed.starts_with("./") || trimmed.starts_with("~/") {
                boundaries.insert(i, (ChunkType::Code, 4));
                continue;
            }

            // Function/class definitions
            let def_patterns = ["function", "class ", "def ", "async function", "export", "fn ", "impl ", "struct ", "enum "];
            if def_patterns.iter().any(|p| trimmed.starts_with(p)) {
                boundaries.insert(i, (ChunkType::Code, 5));
                continue;
            }

            // Error markers
            if trimmed.to_lowercase().starts_with("error") || 
               trimmed.to_lowercase().contains("error:") ||
               trimmed.starts_with("Exception") || 
               trimmed.contains("FAILED") {
                boundaries.insert(i, (ChunkType::Text, 8));
                continue;
            }

            // Section headers
            if trimmed.starts_with('#') && trimmed.len() > 2 && trimmed.chars().nth(1) == Some(' ') {
                boundaries.insert(i, (ChunkType::Text, 6));
                continue;
            }
        }

        boundaries
    }

    /// Split a large chunk into smaller pieces
    fn split_large_chunk(
        lines: &[&str],
        start_line: usize,
        chunk_type: ChunkType,
        max_tokens: usize,
    ) -> Vec<Chunk> {
        let mut chunks = Vec::new();
        let mut current: Vec<&str> = Vec::new();
        let mut current_tokens = 0;
        let mut current_start = start_line;

        for (i, line) in lines.iter().enumerate() {
            let line_tokens = Self::estimate_tokens(line);

            if current_tokens + line_tokens > max_tokens && !current.is_empty() {
                chunks.push(Chunk {
                    content: current.join("\n"),
                    chunk_type,
                    start_line: current_start,
                    end_line: start_line + i - 1,
                    tokens: current_tokens,
                    priority: 3,
                });
                current = Vec::new();
                current_tokens = 0;
                current_start = start_line + i;
            }

            current.push(line);
            current_tokens += line_tokens;
        }

        if !current.is_empty() {
            chunks.push(Chunk {
                content: current.join("\n"),
                chunk_type,
                start_line: current_start,
                end_line: start_line + lines.len() - 1,
                tokens: current_tokens,
                priority: 3,
            });
        }

        chunks
    }

    /// Select chunks to fit within a token budget
    /// Prioritizes high-priority chunks and recent content
    pub fn select_chunks(chunks: &[Chunk], max_tokens: usize) -> Vec<Chunk> {
        let mut sorted: Vec<_> = chunks.iter().cloned().collect();
        
        // Sort by priority (desc), then by line number (desc for recent)
        sorted.sort_by(|a, b| {
            match b.priority.cmp(&a.priority) {
                std::cmp::Ordering::Equal => b.start_line.cmp(&a.start_line),
                other => other,
            }
        });

        let mut selected = Vec::new();
        let mut total_tokens = 0;

        for chunk in sorted {
            if total_tokens + chunk.tokens <= max_tokens {
                selected.push(chunk.clone());
                total_tokens += chunk.tokens;
            }
        }

        // Re-sort by line number for coherent output
        selected.sort_by_key(|c| c.start_line);

        selected
    }

    /// Reassemble selected chunks into a single string
    pub fn reassemble(chunks: &[Chunk]) -> String {
        if chunks.is_empty() {
            return String::new();
        }

        let mut parts = Vec::new();
        let mut last_end: Option<usize> = None;

        for chunk in chunks {
            // Add separator if there's a gap
            if let Some(end) = last_end {
                if chunk.start_line > end + 1 {
                    let gap = chunk.start_line - end - 1;
                    parts.push(format!("\n[... {} lines omitted ...]\n", gap));
                }
            }
            parts.push(chunk.content.clone());
            last_end = Some(chunk.end_line);
        }

        parts.join("\n")
    }

    /// Intelligently compress content to fit within token budget
    pub fn compress(content: &str, max_tokens: usize, options: Option<ChunkOptions>) -> String {
        let chunks = Self::chunk(content, options);
        let selected = Self::select_chunks(&chunks, max_tokens);
        Self::reassemble(&selected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_code() {
        let content = r#"
fn main() {
    println!("Hello, world!");
}

impl Foo {
    pub fn new() -> Self {
        Self {}
    }
}
"#;
        assert_eq!(RlmChunker::detect_content_type(content), ContentType::Code);
    }

    #[test]
    fn test_detect_conversation() {
        let content = r#"
[User]: Can you help me with this?

[Assistant]: Of course! What do you need?

[User]: I want to implement a feature.
"#;
        assert_eq!(RlmChunker::detect_content_type(content), ContentType::Conversation);
    }

    #[test]
    fn test_compress() {
        let content = "line\n".repeat(1000);
        let compressed = RlmChunker::compress(&content, 100, None);
        let tokens = RlmChunker::estimate_tokens(&compressed);
        assert!(tokens <= 100 || compressed.contains("[..."));
    }
}
