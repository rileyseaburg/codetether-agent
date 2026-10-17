use super::{
    CompletionRequest, CompletionResponse, ContentPart, Message, ModelInfo, Provider, Role,
    StreamChunk, ToolDefinition,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
#[cfg(test)]
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use futures::stream::BoxStream;
use futures::{SinkExt, StreamExt};
use reqwest::{Client, StatusCode};
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::http::Request;

use thinking_level::ThinkingLevel;
use transport_health::TransportHealth;
use turn_state::TurnStateStore;
use ws_pool::WsPool;
