//! Canonical provider aliases accepted by the binary's CLI handlers.

pub(super) fn normalize_provider_alias(name: &str) -> &str {
    match name {
        "local-cuda" | "localcuda" => "local_cuda",
        "zhipuai" => "zai",
        other => other,
    }
}
