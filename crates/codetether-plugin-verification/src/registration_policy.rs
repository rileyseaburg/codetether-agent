//! registration policy for verified plugin registration.

pub(super) fn allow_unverified_content_registration() -> bool {
    std::env::var("CODETETHER_ALLOW_UNVERIFIED_PLUGIN_CONTENT")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}
