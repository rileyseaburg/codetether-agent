//! Diagnostic mapping for Bedrock tool-pairing validation failures.
//!
//! Bedrock rejects a request whose assistant `toolUse` block has no matching
//! `toolResult` with: `Expected toolResult blocks at messages.N.content for
//! the following Ids: <id>`. [`crate::provider::bedrock::audit`] repairs the
//! body before send, so reaching this error means a code path built or mutated
//! `messages` without passing through that audit. Say so explicitly instead of
//! leaving an opaque permanent 400.

/// Whether the error body is a tool-result pairing validation failure.
pub(in crate::provider::bedrock) fn is_pairing_failure(body: &str) -> bool {
    body.contains("Expected toolResult blocks")
}

/// Append pairing-specific diagnosis to a Bedrock error message.
pub(in crate::provider::bedrock) fn guidance(base: &str) -> String {
    format!(
        "{base}\n\
         This is a tool-pairing validation failure: an assistant toolUse \
         block reached Bedrock without its matching toolResult. The request \
         body bypassed the pre-send pairing audit \
         (`provider::bedrock::audit::enforce`); the transcript itself is not \
         corrupt and retrying the same body will fail identically."
    )
}

/// Return `base` with pairing guidance appended when `body` warrants it.
pub(in crate::provider::bedrock) fn annotate(base: &str, body: &str) -> String {
    if is_pairing_failure(body) {
        return guidance(base);
    }
    base.to_string()
}

#[cfg(test)]
mod tests {
    use super::{guidance, is_pairing_failure};

    #[test]
    fn recognizes_the_bedrock_pairing_message() {
        assert!(is_pairing_failure(
            "{\"message\":\"Expected toolResult blocks at messages.0.content for the following Ids: call_70YI\"}"
        ));
        assert!(!is_pairing_failure("{\"message\":\"throttled\"}"));
    }

    #[test]
    fn guidance_names_the_audit_that_should_have_run() {
        let text = guidance("Bedrock stream error (400 Bad Request): ...");
        assert!(text.contains("audit::enforce"));
        assert!(text.contains("fail identically"));
    }
}
