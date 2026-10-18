#[test]
fn sol61_ultrafast_alias_resolves_for_both_transports() {
    for effort in [
        None,
        Some("low"),
        Some("medium"),
        Some("high"),
        Some("xhigh"),
        Some("max"),
        Some("ultra"),
    ] {
        let alias = effort.map_or_else(
            || "gpt-6.1-sol-ultrafast".to_string(),
            |effort| format!("gpt-6.1-sol-ultrafast:{effort}"),
        );
        let request = reasoning_probe_request(&alias);
        let (model, level, tier) =
            OpenAiCodexProvider::resolve_model_and_reasoning_effort_and_service_tier(&alias);
        assert_eq!(model, "gpt-6.1-sol");
        assert_eq!(tier, Some(CodexServiceTier::Ultrafast));
        assert_eq!(
            codetether_provider_core::codex::resolved_model_identity(&alias),
            model
        );
        let http = OpenAiCodexProvider::build_http_responses_body(&request);
        let ws =
            OpenAiCodexProvider::build_responses_ws_create_event(&request, &model, level, tier);
        for body in [http, ws] {
            assert_eq!(body["model"], "gpt-6.1-sol");
            assert_eq!(body["service_tier"], "ultrafast");
            if let Some(effort) = effort {
                assert_eq!(
                    body["reasoning"]["effort"],
                    if effort == "ultra" { "max" } else { effort }
                );
            }
        }
    }
}
