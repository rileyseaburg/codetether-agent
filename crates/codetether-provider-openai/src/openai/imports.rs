//! Shared transport types used by the implementation modules.

//! OpenAI provider implementation

pub(super) use anyhow::Result;
pub(super) use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionMessageToolCalls, ChatCompletionRequestMessage, ChatCompletionTool,
        ChatCompletionTools, CreateChatCompletionRequestArgs, FinishReason as OpenAIFinishReason,
        FunctionObjectArgs,
    },
};
pub(super) use async_trait::async_trait;
pub(super) use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ContentPart, EmbeddingRequest, EmbeddingResponse,
    FinishReason, Message, ModelInfo, Provider, Role, StreamChunk, ToolDefinition, Usage,
};
pub(super) use futures::StreamExt;
pub(super) use reqwest::Client as HttpClient;
