//! Backend variants held by a constructed [`ThinkerClient`](super::ThinkerClient).

use reqwest::Client;
use std::sync::{Arc, Mutex};

use super::CandleThinker;
use crate::provider::bedrock::BedrockProvider;

/// A constructed, ready-to-use backend runtime.
#[derive(Clone)]
pub(super) enum ThinkerClientBackend {
    /// OpenAI-compatible HTTP endpoint.
    OpenAICompat { http: Client },
    /// Shared provider registry.
    Registry,
    /// In-process Candle runtime, serialized behind a mutex.
    Candle { runtime: Arc<Mutex<CandleThinker>> },
    /// Amazon Bedrock Converse.
    Bedrock { provider: Arc<BedrockProvider> },
}
