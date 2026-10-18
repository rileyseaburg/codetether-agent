//! Shared native transform and provider-policy hooks for context derivation.
pub(super) fn native() -> codetether_context_legacy::Hooks {
    codetether_context_legacy::Hooks {
        apply: codetether_context_compression::experimental::apply_all,
        repair: codetether_context_compression::experimental::pairing::repair_orphans,
        // Model resolution uses only the provider policy, never the event type.
        resolver: codetether_application_providers::background::Background::<
            codetether_native_providers::registry::application::Native,
            (),
        >::resolve_model,
    }
}
