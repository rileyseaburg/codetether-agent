use super::apply;

const CODEX: &str = "openai-codex";

#[test]
fn folds_tier_and_effort_into_suffixes() {
    let got = apply(CODEX, "gpt-6-astra", Some("ultrafast"), Some("high")).unwrap();
    assert_eq!(got, "gpt-6-astra-ultrafast:high");
    let got = apply(CODEX, "gpt-5.5", Some("priority"), None).unwrap();
    assert_eq!(got, "gpt-5.5-fast");
    let got = apply(CODEX, "gpt-5.5:low", None, Some("xhigh")).unwrap();
    assert_eq!(got, "gpt-5.5:xhigh");
    let got = apply(CODEX, "gpt-6-astra-fast:low", Some("default"), None).unwrap();
    assert_eq!(got, "gpt-6-astra:low");
}

#[test]
fn passes_through_when_unset() {
    assert_eq!(
        apply("anthropic", "claude-x", None, None).unwrap(),
        "claude-x"
    );
    assert_eq!(
        apply(CODEX, "gpt-5.5:high", None, None).unwrap(),
        "gpt-5.5:high"
    );
}

#[test]
fn rejects_unsupported_requests() {
    assert!(apply("anthropic", "claude-x", None, Some("high")).is_err());
    assert!(apply(CODEX, "gpt-5.5", Some("ultrafast"), None).is_err());
    assert!(apply(CODEX, "gpt-5.5", None, Some("ultra")).is_err());
    assert!(apply(CODEX, "gpt-5.5", Some("warp"), None).is_err());
}
