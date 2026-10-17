//! Reasoning-content (extended thinking) blocks from the Converse API.
//!
//! Claude Fable 5 / Opus 4.7 return encrypted reasoning: blocks whose
//! `reasoningText` carries only a `signature` (opaque ciphertext) and no
//! plaintext. Both shapes are accepted here; the signature is preserved on
//! [`ContentPart::Thinking`] and logged so token spend on invisible
//! reasoning is observable.

use codetether_provider_core::ContentPart;
use serde::Deserialize;

/// `reasoningContent` block: `{"reasoningText": {"text"?, "signature"?}}`.
///
/// # Examples
/// ```
/// let block: codetether_bedrock_protocol::reasoning::ReasoningContentBlock =
///     serde_json::from_str(r#"{"reasoningText":{"text":"working"}}"#)?;
/// assert!(block.to_content_part().is_some());
/// # Ok::<(), serde_json::Error>(())
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReasoningContentBlock {
    #[serde(default)]
    reasoning_text: Option<ReasoningText>,
}

#[derive(Debug, Default, Deserialize)]
struct ReasoningText {
    #[serde(default)]
    text: String,
    #[serde(default)]
    signature: Option<String>,
}

impl ReasoningContentBlock {
    /// Convert to a [`ContentPart::Thinking`], keeping the encrypted
    /// signature. Returns `None` for blocks with neither text nor signature.
    pub fn to_content_part(&self) -> Option<ContentPart> {
        let rt = self.reasoning_text.as_ref()?;
        if let Some(sig) = &rt.signature {
            super::reasoning_audit::record_signature("converse", None, sig);
        }
        if rt.text.is_empty() && rt.signature.is_none() {
            return None;
        }
        Some(ContentPart::Thinking {
            text: rt.text.clone(),
            signature: rt.signature.clone(),
        })
    }
}
#[cfg(test)]
#[path = "reasoning/tests.rs"]
mod tests;
