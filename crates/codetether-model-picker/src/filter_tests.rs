//! Model matching regression tests.

use super::matching;

fn catalog() -> Vec<String> {
    vec![
        "bedrock/global.anthropic.claude-fable-5".to_string(),
        "bedrock/us.anthropic.claude-fable-5".to_string(),
        "zai/glm-5".to_string(),
    ]
}

#[test]
fn multi_term_filter_matches_non_adjacent_tokens() {
    let models = catalog();
    let hits = matching(&models, "bedrock fable");
    assert_eq!(
        hits,
        vec![
            "bedrock/global.anthropic.claude-fable-5",
            "bedrock/us.anthropic.claude-fable-5"
        ]
    );
}

#[test]
fn empty_filter_matches_all_and_extra_spaces_ignored() {
    let models = catalog();
    assert_eq!(matching(&models, "   ").len(), 3);
    assert_eq!(matching(&models, "zai   glm"), vec!["zai/glm-5"]);
}
