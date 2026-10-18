use super::{
    CompletionRequest, CompletionResponse, Message, ModelInfo, Provider,
    StreamChunk, ToolDefinition,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
#[cfg(test)]
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use futures::stream::BoxStream;
#[cfg(test)]
use super::{ContentPart, Role};
#[cfg(test)]
use futures::{SinkExt, StreamExt};
use reqwest::{Client, StatusCode};
use serde_json::Value;
#[cfg(test)]
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
#[cfg(test)]
use tokio_tungstenite::tungstenite::http::Request;

use thinking_level::ThinkingLevel;
use transport_health::TransportHealth;
use turn_state::TurnStateStore;
use ws_pool::WsPool;
