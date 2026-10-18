//! Model filter and selected-row regression coverage.
use crate::ModelPickerState;

#[test]
fn model_filter_limits_visible_models() {
    let mut state = ModelPickerState::default();
    state.available_models = vec![
        "zai/glm-5".to_string(),
        "openai/gpt-4o".to_string(),
        "openrouter/qwen/qwen3-coder".to_string(),
    ];
    state.model_filter_push('g');
    state.model_filter_push('p');
    state.model_filter_push('t');
    assert_eq!(state.filtered_models(), vec!["openai/gpt-4o"]);
    assert_eq!(state.selected_model(), Some("openai/gpt-4o"));
}
