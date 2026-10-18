//! Forage task option conversion regression.
use super::build_forage_args;

#[test]
fn build_forage_args_defaults_prompt_to_moonshot_and_local_mode() {
    let args = build_forage_args(
        "Expand enterprise adoption in regulated markets",
        "forage task",
        &serde_json::Map::new(),
        Some("openrouter/z-ai/glm-5".to_string()),
    );
    assert_eq!(
        args.moonshots,
        vec!["Expand enterprise adoption in regulated markets".to_string()]
    );
    assert!(args.no_s3);
    assert_eq!(args.model.as_deref(), Some("openrouter/z-ai/glm-5"));
    assert_eq!(args.execution_engine, "run");
}
