//! Shared checkouts must not turn session outcomes into other sessions' context.
#[test]
fn cwd_prompt_does_not_import_sibling_writebacks_or_session_beliefs() {
    let dir = tempfile::tempdir().unwrap();
    let writebacks = dir.path().join(".codetether/memory-writeback");
    std::fs::create_dir_all(&writebacks).unwrap();
    for (id, value) in [
        ("ads-session", "google-ads-private-note"),
        ("quote-session", "quote-private-note"),
    ] {
        std::fs::write(
            writebacks.join(format!("{id}.json")),
            format!(r#"[{{"level":"static/local","value":"{value}"}}]"#),
        )
        .unwrap();
    }
    let item = super::scope_item::ScopeItem {
        deliverable: "google-ads-private-belief".into(),
        status: "proven".into(),
        evidence_level: Some("static/local".into()),
        evidence: vec![],
    };
    let private = super::palace_belief::from_item(&item, "ads-session");
    let mut shared = super::palace_belief::from_item(&item, "quote-session");
    shared.asserted_by = "user".into();
    shared.belief_key = "shared-convention".into();
    shared.claim = "shared project coding convention".into();
    codetether_cognition_core::palace::save_project_beliefs(dir.path(), &[private, shared])
        .unwrap();

    let prompt = super::append_guardrails_for_cwd("base".into(), dir.path(), true);
    assert!(!prompt.contains("google-ads-private-note"));
    assert!(!prompt.contains("quote-private-note"));
    assert!(!prompt.contains("google-ads-private-belief"));
    assert!(!prompt.contains("Recent proven deliverables:"));
    assert!(prompt.contains("shared project coding convention"));
    assert!(writebacks.join("ads-session.json").exists());
    assert!(writebacks.join("quote-session.json").exists());

    let denied = super::append_guardrails_for_cwd("base".into(), dir.path(), false);
    assert!(!denied.contains("shared project coding convention"));
}
