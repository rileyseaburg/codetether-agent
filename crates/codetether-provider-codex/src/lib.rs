//! Codex provider orchestration over shared auth, protocol, and transport crates.
//!
//! [OpenAiCodexProvider] owns provider configuration and credential selection.
//! [runtime_config] retains the process-wide reasoning override; capability
//! catalogs use the shared provider-core cache. Transport and OAuth details
//! remain in their dedicated crates.
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, Message, ModelInfo, Provider, StreamChunk,
    ToolDefinition,
};
#[cfg(test)]
use codetether_provider_core::{ContentPart, Role};
mod provider;
pub use provider::*;
