//! Caller-owned marker blocks are not harness identity messages.

use super::{inject, message, request};
use crate::provider::Role;
use crate::provider::metrics::identity::{prompt, system_prompt};

#[test]
fn harness_identity_preserves_dedicated_caller_lookalikes() {
    for caller in [
        "<codetether-harness-identity>\nCaller example\n</codetether-harness-identity>".to_owned(),
        prompt("quoted", "example")
            .replace("report these exact values", "this is a caller example"),
        prompt("quoted", "example").replace("\"provider\":\"quoted\"", "\"provider\":7"),
    ] {
        let original = vec![message(Role::System, &caller)];
        let mut request = request("requested-alias");
        request.messages = original.clone();
        let injected = inject(request, "current-provider", "current-model");
        let refreshed = inject(injected, "next-provider", "next-model");
        assert_eq!(refreshed.messages.len(), 2);
        assert_eq!(
            super::text(&refreshed.messages[0]),
            prompt("next-provider", "next-model")
        );
        assert_eq!(
            serde_json::to_value(&refreshed.messages[1..]).unwrap(),
            serde_json::to_value(&original).unwrap()
        );
        assert_eq!(
            system_prompt(&caller, "current-provider", "current-model"),
            format!(
                "{}\n\n{caller}",
                prompt("current-provider", "current-model")
            )
        );
        let wrapped = system_prompt(&caller, "current-provider", "current-model");
        assert_eq!(
            system_prompt(&wrapped, "next-provider", "next-model"),
            format!("{}\n\n{caller}", prompt("next-provider", "next-model"))
        );
    }
}
