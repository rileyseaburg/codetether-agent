use super::parse_models;

#[test]
fn parses_listed_and_hidden_models_ignoring_unknown_fields() {
    let body = br#"{"models":[
        {"slug":"gpt-6.1-sol","display_name":"GPT-6.1 Sol","visibility":"list",
         "context_window":400000,"priority":1,"shell_type":"shell_command"},
        {"slug":"codex-internal","visibility":"hide"}
    ]}"#;
    let models = parse_models(body).unwrap();
    assert_eq!(models.len(), 2);
    assert_eq!(models[0].slug, "gpt-6.1-sol");
    assert_eq!(models[0].context_window, Some(400_000));
    assert!(models[0].is_listed());
    assert!(!models[1].is_listed());
}

#[test]
fn rejects_non_catalog_bodies() {
    assert!(parse_models(b"{\"detail\":\"Unauthorized\"}").is_err());
}

#[test]
fn parses_account_reasoning_and_service_tiers() {
    let models = parse_models(
        br#"{"models":[{"slug":"gpt-6.1-sol",
      "supported_reasoning_levels":[{"effort":"high"},{"effort":"ultra"}],
      "service_tiers":[{"id":"priority","name":"Fast"}]}]}"#,
    )
    .unwrap();
    assert_eq!(models[0].supported_reasoning_levels[1].effort, "ultra");
    assert_eq!(models[0].service_tiers[0].id, "priority");
}
