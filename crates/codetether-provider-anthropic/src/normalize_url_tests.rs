use super::normalize_minimax_anthropic_base_url;

#[test]
fn normalizes_minimax_url() {
    assert_eq!(
        normalize_minimax_anthropic_base_url("https://api.minimax.io/v1"),
        "https://api.minimax.io/anthropic"
    );
    assert_eq!(
        normalize_minimax_anthropic_base_url("https://custom.host/custom"),
        "https://custom.host/custom"
    );
}
