//! Models where the `temperature` parameter must not be sent.
//!
//! Newer Claude 5 and GPT-6 models removed temperature support. Sending
//! `temperature` to these models returns HTTP 400.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::session::helper::provider::temperature::temperature_is_deprecated;
//!
//! assert!(temperature_is_deprecated("claude-opus-5"));
//! assert!(temperature_is_deprecated("claude-haiku-5-5"));
//! assert!(temperature_is_deprecated("gpt-6-astra"));
//! assert!(temperature_is_deprecated("claude-opus-4-7"));
//! assert!(!temperature_is_deprecated("claude-opus-4-6"));
//! ```

/// Whether `temperature` is deprecated for `model`.
///
/// # Arguments
///
/// * `model` — Model identifier, any casing, optionally provider-qualified.
///
/// # Returns
///
/// `true` when the request must omit `temperature`.
pub fn temperature_is_deprecated(model: &str) -> bool {
    let m = model.to_ascii_lowercase();
    m.contains("gpt-6")
        || m.contains("gpt_6")
        || m.contains("fable-5")
        || m.contains("fable_5")
        || m.contains("sonnet-5")
        || m.contains("sonnet_5")
        || m.contains("haiku-5")
        || m.contains("haiku_5")
        || m.contains("opus-5")
        || m.contains("opus_5")
        || m.contains("5-fable")
        || m.contains("5-sonnet")
        || m.contains("5-haiku")
        || m.contains("5-opus")
        || m.contains("opus-4-7")
        || m.contains("opus-4.7")
        || m.contains("4.7-opus")
        || m.contains("4-7-opus")
        || m.contains("opus_4_7")
        || m.contains("opus_47")
}

#[cfg(test)]
#[path = "temperature_tests.rs"]
mod tests;
