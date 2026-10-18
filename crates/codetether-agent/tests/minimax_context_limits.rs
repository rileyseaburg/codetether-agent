use codetether_agent::provider::limits::context_window_for_model;

#[test]
fn minimax_m3_formats_get_million_token_context() {
    assert_eq!(context_window_for_model("minimax/m3"), 1_000_000);
    assert_eq!(context_window_for_model("MiniMax-M3"), 1_000_000);
    assert_eq!(context_window_for_model("minimaxm3"), 1_000_000);
}

#[test]
fn other_minimax_three_models_keep_minimax_default() {
    assert_eq!(context_window_for_model("minimax/llama3"), 256_000);
    assert_eq!(context_window_for_model("minimax/glm3"), 256_000);
}
