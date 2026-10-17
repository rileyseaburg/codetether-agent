//! Focused provider construction, adapters, and tool-protocol retry.
mod ask;
mod ask_validated;
mod build_freq;
mod build_request;
mod compact_body_snippet;
mod cookie_header;
mod extract_protocol_error_code;
mod extract_protocol_request_id;
mod extract_text;
mod extract_tool_calls;
mod format_protocol_error;
mod get_or_refresh_tokens;
mod get_session_tokens;
mod invalidate_tokens;
mod new;
