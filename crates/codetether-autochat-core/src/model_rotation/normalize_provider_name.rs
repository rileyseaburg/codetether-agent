//! Auto-chat normalize provider name.
use super::*;

pub(super) fn normalize_provider_name(provider_name: &str) -> &str {
    if provider_name.eq_ignore_ascii_case("zhipuai") || provider_name.eq_ignore_ascii_case("z-ai") {
        "zai"
    } else {
        provider_name
    }
}
