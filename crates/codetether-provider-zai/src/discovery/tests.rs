//! Preserve the two discovery decoders' existing model metadata rules.
use serde_json::json;

#[test]
fn application_trims_ids_preserves_names_and_omits_vision() {
    let entry = json!({"id": " glm-5-vision ", "name": " Display name "});
    let model = super::application_model::decode(&entry).unwrap();
    assert_eq!(model.id, "glm-5-vision");
    assert_eq!(model.name, "Display name");
    assert!(!model.supports_vision);
    assert_eq!(model.context_window, 200_000);
    assert!(super::application_model::decode(&json!("  ")).is_none());
}

#[test]
fn injected_client_preserves_raw_ids_and_infers_vision() {
    let models = super::injected_model::parse(
        &json!({"data": [{"id": " glm-5-vision ", "name": "Display name"}, ""]}),
    );
    assert_eq!(models.len(), 2);
    assert_eq!(models[0].id, " glm-5-vision ");
    assert_eq!(models[0].name, models[0].id);
    assert!(models[0].supports_vision);
    assert_eq!(models[1].id, "");
}
