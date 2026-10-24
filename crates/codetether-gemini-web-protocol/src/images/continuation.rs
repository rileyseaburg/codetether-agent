use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};

/// Server-issued identifiers needed to refine an existing image conversation.
///
/// JSON uses the camelCase keys from Node's private conversation files.
/// Snake-case aliases are accepted on input. Deserialization does not validate
/// identifiers; call [`Self::validate`] before using externally supplied state.
///
/// # Examples
/// ```
/// use codetether_gemini_web_protocol::images::Continuation;
/// let state = Continuation {
///     conversation_id: "c_example".into(), response_id: "r_example".into(),
///     choice_id: "rc_example".into(),
/// };
/// assert!(state.validate().is_ok());
/// assert_eq!(serde_json::to_value(&state).unwrap()["choiceId"], "rc_example");
/// ```
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Continuation {
    /// Conversation identifier, beginning with `c_`.
    #[serde(alias = "conversation_id")]
    pub conversation_id: String,
    /// Response identifier, beginning with `r_`.
    #[serde(alias = "response_id")]
    pub response_id: String,
    /// Candidate identifier, beginning with `rc_`.
    #[serde(alias = "choice_id")]
    pub choice_id: String,
}

impl Continuation {
    /// Check prefixes and bounded ASCII identifier suffixes.
    ///
    /// # Errors
    /// Returns a fixed `image_state_invalid` error, never echoing invalid input.
    pub fn validate(&self) -> Result<()> {
        ensure!(valid(&self.conversation_id, "c_"), "image_state_invalid");
        ensure!(valid(&self.response_id, "r_"), "image_state_invalid");
        ensure!(valid(&self.choice_id, "rc_"), "image_state_invalid");
        Ok(())
    }
}

fn valid(value: &str, prefix: &str) -> bool {
    value.strip_prefix(prefix).is_some_and(|suffix| {
        (1..=128).contains(&suffix.len())
            && suffix
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    })
}
