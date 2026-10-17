//! Codex chat and Responses message encoding and response decoding.
//!
//! [CodexProtocol] encodes messages and tools. [reasoning] preserves opaque
//! checkpoints; request repair and anchors operate on projected input only.
//! [ResponsesSseParser] retains stream state, [CompletionCollector] builds
//! buffered responses, and [ws_event] reports WebSocket lifecycle decisions.
use codetether_provider_core::{
    CompletionResponse, ContentPart, FinishReason, Message, Role, StreamChunk, ToolDefinition,
    Usage,
};
use serde_json::{Value, json};
mod chat;
mod images;
mod pairing;
pub mod reasoning;
mod request_anchor;
mod responses;
mod text;
pub use images::UserImageFormat;
/// Stateless message encoding and response decoding for Codex transports.
///
/// # Examples
/// ```
/// assert!(codetether_codex_protocol::CodexProtocol::convert_messages(&[]).is_empty());
/// ```
pub struct CodexProtocol;

mod collector;
pub mod event_error;
pub mod output_item;
mod parser;
mod parser_state;
pub mod ws_event;
use anyhow::Result;
pub use collector::CompletionCollector;
use futures::StreamExt;
pub use parser_state::{ResponsesSseParser, ResponsesToolState};
use std::collections::HashMap;

pub mod request;
