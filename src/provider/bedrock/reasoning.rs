//! Reasoning-content (extended thinking) blocks from the Converse API.
//!
//! Claude Fable 5 / Opus 4.7 return encrypted reasoning: blocks whose
//! `reasoningText` carries only a `signature` (opaque ciphertext) and no
//! plaintext. Both shapes are accepted here; the signature is preserved on
//! [`ContentPart::Thinking`] and logged so token spend on invisible
//! reasoning is observable.

use crate::provider::ContentPart;
use serde::Deserialize;

/// `reasoningContent` block: `{"reasoningText": {"text"?, "signature"?}}`.
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
mod tests {
    use super::ReasoningContentBlock;
    use crate::provider::ContentPart;

    #[test]
    fn signature_only_block_is_preserved() {
        let block: ReasoningContentBlock =
            serde_json::from_str(r#"{"reasoningText":{"signature":"CAIS=="}}"#).unwrap();
        let part = block.to_content_part().unwrap();
        assert!(matches!(
            part,
            ContentPart::Thinking { ref text, ref signature }
                if text.is_empty() && signature.as_deref() == Some("CAIS==")
        ));
    }
}
