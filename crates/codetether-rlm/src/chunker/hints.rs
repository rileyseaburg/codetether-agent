//! Processing hints based on content type.

use super::types::ContentType;

/// Get processing hints based on content type.
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
        ContentType::Mixed => "Mixed content detected. Analyze the structure first, then extract key information.",
    }
}