//! Preserve actionable parser codes without exposing response bodies or URLs.
use anyhow::{Error, anyhow};

pub(super) fn classify(error: Error) -> Error {
    let message = error.to_string();
    let code = match message.as_str() {
        "truncated_rpc_stream"
        | "missing_anti_xssi_prefix"
        | "invalid_batch_length"
        | "batch_length_limit"
        | "invalid_batch_json"
        | "invalid_batch_shape"
        | "generation_payload_error"
        | "generation_rpc_error"
        | "generation_answer_error"
        | "generation_image_creation_unavailable"
        | "generation_no_images"
        | "generation_image_limit" => message.as_str(),
        _ => "unknown",
    };
    if let Some(json) = error.downcast_ref::<serde_json::Error>() {
        return anyhow!(
            "image_generation_protocol_error: {code} ({:?}, line={}, column={})",
            json.classify(),
            json.line(),
            json.column()
        );
    }
    anyhow!("image_generation_protocol_error: {code}")
}
