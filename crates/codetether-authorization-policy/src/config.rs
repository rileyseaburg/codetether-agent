//! Environment-driven OPA routing and failure policy.

/// OPA sidecar URL.  Defaults to the K8s sidecar address.
pub(crate) fn opa_url() -> String {
    std::env::var("OPA_URL").unwrap_or_else(|_| "http://localhost:8181".to_string())
}

/// OPA query path for the combined authz + api-key scope policy.
pub(crate) fn opa_path() -> String {
    std::env::var("OPA_AUTHZ_PATH").unwrap_or_else(|_| "v1/data/api_keys/allow".to_string())
}

/// Whether to fail open (allow) when OPA is unreachable.
pub(crate) fn fail_open() -> bool {
    std::env::var("OPA_FAIL_OPEN").is_ok_and(|value| value.eq_ignore_ascii_case("true"))
}

pub(crate) fn sidecar_disabled(local_mode: bool, opa_enabled: Option<&str>) -> bool {
    local_mode || opa_enabled.is_some_and(|value| value.eq_ignore_ascii_case("false"))
}

/// Whether to evaluate policies locally without an OPA sidecar.
pub(crate) fn local_mode() -> bool {
    let opa_enabled = std::env::var("OPA_ENABLED").ok();
    sidecar_disabled(
        std::env::var("OPA_LOCAL_MODE").is_ok_and(|value| value.eq_ignore_ascii_case("true")),
        opa_enabled.as_deref(),
    )
}
