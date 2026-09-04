// Default selection follows the ordered shared Codex catalog.
#[test]
fn openai_codex_defaults_to_first_catalog_model() {
    let models = crate::provider::openai_codex::model_catalog::chatgpt_models();
    assert!(!models.is_empty());
    assert_eq!(
        default_model_for_provider("openai-codex"),
        models.first().copied()
    );
}
