//! Diagnostic mapping for Bedrock tool-pairing validation failures.
//!
//! Bedrock rejects a request whose assistant `toolUse` block has no matching
//! `toolResult` with: `Expected toolResult blocks at messages.N.content for
//! the following Ids: <id>`. [`crate::audit`] repairs the
//! body before send, so reaching this error means a code path built or mutated
//! `messages` without passing through that audit. Say so explicitly instead of
//! leaving an opaque permanent 400.

/// Whether the error body is a tool-result pairing validation failure.
///
/// # Arguments
/// * `body` - HTTP error body.
///
/// # Returns
/// Whether the body contains the known pairing failure message.
///
/// # Examples
/// ```
/// assert!(codetether_bedrock_protocol::audit::pairing_error::is_pairing_failure("Expected toolResult blocks"));
/// ```
pub fn is_pairing_failure(body: &str) -> bool {
    body.contains("Expected toolResult blocks")
}

/// Append pairing-specific diagnosis to a Bedrock error message.
///
/// # Arguments
/// * `base` - Original error message.
///
/// # Returns
/// The message with the existing pairing diagnosis appended.
///
/// # Examples
/// ```
/// assert!(codetether_bedrock_protocol::audit::pairing_error::guidance("error").contains("audit::enforce"));
/// ```
pub fn guidance(base: &str) -> String {
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
///
/// # Arguments
/// * `base` - Original error message.
/// * `body` - HTTP error body.
///
/// # Returns
/// The original message or its annotated pairing diagnosis.
///
/// # Examples
/// ```
/// assert_eq!(codetether_bedrock_protocol::audit::pairing_error::annotate("error", "other"), "error");
/// ```
pub fn annotate(base: &str, body: &str) -> String {
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
