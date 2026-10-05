use crate::provider::bedrock::resolve_model_id;

#[test]
fn sonnet_5_5_aliases_resolve_to_global_profile() {
    for alias in [
        "claude-sonnet-5.5",
        "claude-sonnet-5-5",
        "sonnet-5.5",
        "anthropic.claude-sonnet-5-5",
        "us.anthropic.claude-sonnet-5-5",
        "global.anthropic.claude-sonnet-5-5",
    ] {
        assert_eq!(
            resolve_model_id(alias),
            "global.anthropic.claude-sonnet-5-5",
            "{alias}"
        );
    }
}

#[test]
fn sonnet_5_profiles_pass_through() {
    for id in [
        "us.anthropic.claude-sonnet-5",
        "global.anthropic.claude-sonnet-5",
    ] {
        assert_eq!(resolve_model_id(id), id);
    }
}
