//! Worker capability and interface helpers.

/// Default A2A control-plane URL used by worker callers.
pub const DEFAULT_A2A_SERVER_URL: &str = "https://api.codetether.run";

const BASE_WORKER_CAPABILITIES: &[&str] = &[
    "forage", "ralph", "swarm", "rlm", "a2a", "mcp", "grpc", "grpc-web", "jsonrpc",
];

/// Returns the capability list advertised during worker registration.
pub fn worker_capabilities() -> Vec<String> {
    let mut capabilities = BASE_WORKER_CAPABILITIES
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    if is_knative() {
        push_capability(&mut capabilities, "knative");
    }
    append_env_capabilities(&mut capabilities, "CODETETHER_WORKER_CAPABILITIES");
    append_env_capabilities(&mut capabilities, "A2A_WORKER_CAPABILITIES");
    capabilities
}

/// Builds the advertised HTTP and bus interface URLs for this worker.
pub fn advertised_interfaces(public_url: Option<&str>) -> serde_json::Value {
    let Some(base_url) = public_url.map(str::trim).filter(|value| !value.is_empty()) else {
        return serde_json::json!({});
    };
    let base_url = base_url.trim_end_matches('/');
    serde_json::json!({
        "http": { "base_url": base_url },
        "bus": {
            "stream_url": format!("{base_url}/v1/bus/stream"),
            "publish_url": format!("{base_url}/v1/bus/publish"),
        },
    })
}

fn is_knative() -> bool {
    std::env::var("KNATIVE_SERVICE")
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes"
            )
        })
        .unwrap_or(false)
}

fn append_env_capabilities(capabilities: &mut Vec<String>, env_var: &str) {
    let Ok(raw) = std::env::var(env_var) else {
        return;
    };
    for capability in raw.split(|ch: char| ch == ',' || ch.is_whitespace()) {
        push_capability(capabilities, capability);
    }
}

fn push_capability(capabilities: &mut Vec<String>, capability: &str) {
    let capability = capability.trim();
    if capability.is_empty() || capabilities.iter().any(|existing| existing == capability) {
        return;
    }
    capabilities.push(capability.to_string());
}

#[cfg(test)]
mod tests {
    use super::worker_capabilities;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn with_worker_capability_env(
        codetether_caps: Option<&str>,
        a2a_caps: Option<&str>,
        knative: Option<&str>,
        test: impl FnOnce(),
    ) {
        let _guard = ENV_LOCK.lock().unwrap();
        let original_codetether = std::env::var("CODETETHER_WORKER_CAPABILITIES").ok();
        let original_a2a = std::env::var("A2A_WORKER_CAPABILITIES").ok();
        let original_knative = std::env::var("KNATIVE_SERVICE").ok();
        unsafe {
            match codetether_caps {
                Some(value) => std::env::set_var("CODETETHER_WORKER_CAPABILITIES", value),
                None => std::env::remove_var("CODETETHER_WORKER_CAPABILITIES"),
            }
            match a2a_caps {
                Some(value) => std::env::set_var("A2A_WORKER_CAPABILITIES", value),
                None => std::env::remove_var("A2A_WORKER_CAPABILITIES"),
            }
            match knative {
                Some(value) => std::env::set_var("KNATIVE_SERVICE", value),
                None => std::env::remove_var("KNATIVE_SERVICE"),
            }
        }

        test();

        unsafe {
            match original_codetether {
                Some(value) => std::env::set_var("CODETETHER_WORKER_CAPABILITIES", value),
                None => std::env::remove_var("CODETETHER_WORKER_CAPABILITIES"),
            }
            match original_a2a {
                Some(value) => std::env::set_var("A2A_WORKER_CAPABILITIES", value),
                None => std::env::remove_var("A2A_WORKER_CAPABILITIES"),
            }
            match original_knative {
                Some(value) => std::env::set_var("KNATIVE_SERVICE", value),
                None => std::env::remove_var("KNATIVE_SERVICE"),
            }
        }
    }

    #[test]
    fn worker_capabilities_include_env_additions_without_duplicates() {
        with_worker_capability_env(
            Some("persistent,persistent-workspace git-clone"),
            Some("persistent,repo-cache"),
            None,
            || {
                let capabilities = worker_capabilities();
                assert!(capabilities.iter().any(|cap| cap == "persistent"));
                assert!(capabilities.iter().any(|cap| cap == "persistent-workspace"));
                assert!(capabilities.iter().any(|cap| cap == "git-clone"));
                assert!(capabilities.iter().any(|cap| cap == "repo-cache"));
                assert_eq!(
                    capabilities
                        .iter()
                        .filter(|cap| cap.as_str() == "persistent")
                        .count(),
                    1
                );
            },
        );
    }

    #[test]
    fn worker_capabilities_include_knative_when_enabled() {
        with_worker_capability_env(None, None, Some("true"), || {
            let capabilities = worker_capabilities();
            assert!(capabilities.iter().any(|cap| cap == "knative"));
        });
    }
}
