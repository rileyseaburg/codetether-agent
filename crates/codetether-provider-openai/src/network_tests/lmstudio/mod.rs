//! Loopback-only LM Studio capacity and request-budget contracts.

use super::*;

const MODEL: &str = "google/gemma-4-12b-qat";

fn metadata(capacity: usize) -> Reply {
    reply(
        200,
        "application/json",
        json!({"data": [{
            "id": MODEL, "type": "llm", "state": "loaded",
            "max_context_length": 262144, "loaded_context_length": capacity,
        }]})
        .to_string(),
    )
}

fn provider(url: String) -> OpenAIProvider {
    OpenAIProvider::with_base_url_optional_key(None, url, "lmstudio").unwrap()
}

mod discovery;
mod live;
mod overflow;
mod requests;
mod tool_stream;
