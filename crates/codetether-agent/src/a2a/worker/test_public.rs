#[test]
fn resolve_worker_id_prefers_env() {
    let original = std::env::var("CODETETHER_WORKER_ID").ok();
    unsafe {
        std::env::set_var("CODETETHER_WORKER_ID", "harvester-test-worker");
    }
    let resolved = super::resolve_worker_id();
    match original {
        Some(value) => unsafe {
            std::env::set_var("CODETETHER_WORKER_ID", value);
        },
        None => unsafe {
            std::env::remove_var("CODETETHER_WORKER_ID");
        },
    }
    assert_eq!(resolved, "harvester-test-worker");
}

#[test]
fn advertised_interfaces_include_http_and_bus_urls() {
    let interfaces =
        codetether_a2a_worker_core::advertised_interfaces(Some("http://worker.test:8080/"));
    assert_eq!(interfaces["http"]["base_url"], "http://worker.test:8080");
    assert_eq!(
        interfaces["bus"]["stream_url"],
        "http://worker.test:8080/v1/bus/stream"
    );
    assert_eq!(
        interfaces["bus"]["publish_url"],
        "http://worker.test:8080/v1/bus/publish"
    );
}

#[test]
fn advertised_interfaces_omit_empty_public_url() {
    assert_eq!(
        codetether_a2a_worker_core::advertised_interfaces(None),
        serde_json::json!({})
    );
    assert_eq!(
        codetether_a2a_worker_core::advertised_interfaces(Some("   ")),
        serde_json::json!({})
    );
    assert!(super::build_worker_http_client().is_ok());
}
