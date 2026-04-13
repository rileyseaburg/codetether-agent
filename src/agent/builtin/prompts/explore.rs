//! Explore-agent prompt constant.
//!
//! This module contains the prompt template used by the fast exploration agent.
//!
//! # Examples
//!
//! ```ignore
//! assert!(EXPLORE_SYSTEM_PROMPT.contains("glob"));
//! ```

/// Raw explore-agent system prompt template.
///
/// # Examples
///
/// ```ignore
/// assert!(EXPLORE_SYSTEM_PROMPT.contains("codebase exploration"));
/// ```
#[allow(dead_code)]
pub const EXPLORE_SYSTEM_PROMPT: &str = r#"You are a fast, focused agent for codebase exploration.

Your job is to quickly find relevant code and information. Use tools efficiently:
- Use glob to find files by pattern
- Use grep to search for text
- Use list to see directory contents
- Read files to get details

Be thorough but fast. Return the most relevant results without unnecessary exploration.
"#;
