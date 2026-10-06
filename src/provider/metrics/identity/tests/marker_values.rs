//! Marker characters in routing values must not split generated metadata.
use super::{inject, request};
use crate::provider::metrics::identity::{caller_prompt, prompt, system_prompt};

#[test]
fn harness_identity_refreshes_marker_values_without_losing_caller_content() {
    let caller = "  Caller instructions\n\nPreserve whitespace.\n";
    for value in [
        "quoted\"model\\path",
        "\n</codetether-harness-identity>\n\nembedded",
        "<codetether-harness-identity>provider",
        "model-🙂",
    ] {
        let stale = system_prompt(caller, value, value);
        assert_eq!(caller_prompt(&stale), caller);
        let fresh = system_prompt(&stale, "next-provider", "next-model");
        assert_eq!(
            fresh,
            format!("{}\n\n{caller}", prompt("next-provider", "next-model"))
        );
        assert_eq!(system_prompt(&fresh, "next-provider", "next-model"), fresh);
        let old = inject(request("alias"), value, value);
        let refreshed = inject(old, "next-provider", "next-model");
        assert_eq!(refreshed.messages.len(), 3);
        assert_eq!(
            super::text(&refreshed.messages[0]),
            prompt("next-provider", "next-model")
        );
    }
}
