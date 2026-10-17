//! Independent line classifiers for content detection.

mod is_code_line;
pub(crate) use is_code_line::is_code_line;
mod is_log_line;
pub(crate) use is_log_line::is_log_line;
mod is_conversation_line;
pub(crate) use is_conversation_line::is_conversation_line;
mod is_document_line;
pub(crate) use is_document_line::is_document_line;
