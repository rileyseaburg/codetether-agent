#[test]
fn browser_docs_do_not_delegate_to_external_browser_engines() {
    let texts = [
        include_str!("../docs/agent-browser-contract.md"),
        include_str!("../docs/browser-capability-api.md"),
        include_str!("../src/browser_agent.rs"),
    ];

    for text in texts {
        let lower = text.to_ascii_lowercase();
        assert!(!lower.contains("playwright"));
        assert!(!lower.contains("chromium"));
        assert!(!lower.contains("chrome"));
        assert!(!lower.contains("cdp"));
    }
}
