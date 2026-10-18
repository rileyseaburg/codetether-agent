//! OpenAI chat API conversion and validation against shared provider contracts.
//!
//! Convert incoming content, messages and tool definitions with preserved API
//! errors; normalize model references and project provider replies to wire data.
mod error;
pub use error::{
    OpenAiApiError, OpenAiApiResult, openai_bad_request, openai_error, openai_internal_error,
};
mod model;
pub use model::{canonicalize_provider_name, make_openai_model_id, normalize_model_reference};
mod content_part;
pub use content_part::parse_openai_content_part;
mod content;
pub use content::{parse_openai_content_parts, parse_openai_tool_content};
mod tools;
pub use tools::convert_openai_tools;
mod response;
pub use response::convert_response_message;
mod finish;
pub use finish::convert_finish_reason;
mod message_assistant;
mod message_tool;
mod message_user;
mod messages;
pub use messages::convert_openai_messages;
