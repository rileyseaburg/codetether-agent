//! Audit-trail retention of encrypted reasoning ciphertext.
//!
//! Compliance requirement: opaque reasoning signatures returned by
//! encrypted-thinking models (Claude Fable 5 / Opus 4.7) must be retained
//! **verbatim** so reasoning token spend can be attributed and the
//! ciphertext re-verified later. Each block is:
//!
//! 1. Emitted as a structured `tracing` event under the `audit` target
//!    (captured by log files even when the global audit log is absent,
//!    e.g. TUI / `run` mode).
//! 2. Appended to the global [`AuditLog`](crate::audit::AuditLog) (ring
//!    buffer + JSONL file) when initialized — server / forage mode.

use crate::audit::{AuditCategory, AuditOutcome, try_audit_log};

/// Record one encrypted reasoning signature, verbatim.
///
/// `source` distinguishes the non-streaming (`"converse"`) and streaming
/// (`"converse-stream"`) paths; `block_index` is the Converse content-block
/// index when known.
pub fn record_signature(source: &'static str, block_index: Option<u64>, signature: &str) {
    tracing::info!(
        target: "audit",
        provider = "bedrock",
        source,
        block_index,
        signature_len = signature.len(),
        signature = %signature,
        "Encrypted reasoning signature retained"
    );

    let Some(log) = try_audit_log() else { return };
    let Ok(handle) = tokio::runtime::Handle::try_current() else {
        return;
    };
    let detail = serde_json::json!({
        "provider": "bedrock",
        "source": source,
        "block_index": block_index,
        "signature": signature,
    });
    handle.spawn(async move {
        log.log(
            AuditCategory::Session,
            "bedrock.encrypted_reasoning",
            AuditOutcome::Success,
            None,
            Some(detail),
        )
        .await;
    });
}
