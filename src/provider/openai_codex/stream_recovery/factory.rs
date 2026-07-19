//! Backend-specific factories for retryable Codex HTTP streams.

#[path = "factory/chatgpt.rs"]
mod chatgpt_backend;
#[path = "factory/openai.rs"]
mod openai_backend;

pub(in crate::provider::openai_codex) use chatgpt_backend::start as chatgpt_http;
pub(in crate::provider::openai_codex) use openai_backend::start as openai_http;
