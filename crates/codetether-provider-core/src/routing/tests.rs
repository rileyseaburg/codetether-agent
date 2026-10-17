use super::parse_model_string;

#[test]
fn parses_provider_model() {
    assert_eq!(
        parse_model_string("openai/gpt-4o"),
        (Some("openai"), "gpt-4o")
    );
}

#[test]
fn parses_bare_model() {
    assert_eq!(parse_model_string("gpt-4o"), (None, "gpt-4o"));
}

#[test]
fn parses_nested_provider() {
    assert_eq!(
        parse_model_string("openrouter/openai/gpt-5-codex"),
        (Some("openrouter"), "openai/gpt-5-codex")
    );
}
