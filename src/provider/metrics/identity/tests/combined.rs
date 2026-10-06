//! Caller instructions between identity examples must not be removed wholesale.

use super::{Role, inject, message, request, text};
use crate::provider::metrics::identity::prompt;

#[test]
fn harness_identity_preserves_combined_system_message() {
    let caller = format!(
        "{}\n\nKeep these caller instructions.\n\n{}",
        prompt("old-provider", "old-model"),
        prompt("example-provider", "example-model")
    );
    let mut original = request("new-model");
    original.messages[0] = message(Role::System, &caller);
    let injected = inject(original.clone(), "new-provider", "new-model");
    assert_eq!(
        text(&injected.messages[0]),
        prompt("new-provider", "new-model")
    );
    assert_eq!(text(&injected.messages[1]), caller);
    assert_eq!(injected.messages.len(), original.messages.len() + 1);
    let refreshed = inject(injected, "next-provider", "next-model");
    assert_eq!(text(&refreshed.messages[1]), caller);
    assert_eq!(refreshed.messages.len(), original.messages.len() + 1);
    assert_eq!(
        text(&refreshed.messages[0]),
        prompt("next-provider", "next-model")
    );
}
