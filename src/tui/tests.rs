#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::{
        AUTOCHAT_QUICK_DEMO_TASK, agent_avatar, agent_profile, command_with_optional_args,
        estimate_cost, extract_semantic_handoff_from_rlm, format_agent_identity,
        format_relay_handoff_line, is_easy_go_command, is_retryable_provider_error,
        is_secure_environment_from_values, match_slash_command_hint, minio_fallback_endpoint,
        next_go_model, normalize_easy_command, normalize_for_convergence,
        normalize_local_model_ref, normalize_minio_endpoint, normalize_provider_alias,
        smart_switch_model_key, view_mode_help_rows,
    };

    #[test]
    fn command_with_optional_args_handles_bare_command() {
        assert_eq!(command_with_optional_args("/spawn", "/spawn"), Some(""));
    }

    #[test]
    fn command_with_optional_args_handles_arguments() {
        assert_eq!(
            command_with_optional_args("/spawn planner you plan", "/spawn"),
            Some("planner you plan")
        );
    }

    #[test]
    fn command_with_optional_args_ignores_prefix_collisions() {
        assert_eq!(command_with_optional_args("/spawned", "/spawn"), None);
    }

    #[test]
    fn command_with_optional_args_ignores_autochat_prefix_collisions() {
        assert_eq!(command_with_optional_args("/autochatty", "/autochat"), None);
    }

    #[test]
    fn command_with_optional_args_trims_leading_whitespace_in_args() {
        assert_eq!(
            command_with_optional_args("/kill    local-agent-1", "/kill"),
            Some("local-agent-1")
        );
    }

    #[test]
    fn slash_hint_includes_protocol_command() {
        let hint = match_slash_command_hint("/protocol");
        assert!(hint.contains("/protocol"));
    }

    #[test]
    fn slash_hint_includes_autochat_command() {
        let hint = match_slash_command_hint("/autochat");
        assert!(hint.contains("/autochat"));
    }

    #[test]
    fn slash_hint_includes_autochat_local_command() {
        let hint = match_slash_command_hint("/autochat-local");
        assert!(hint.contains("/autochat-local"));
    }

    #[test]
    fn slash_hint_includes_file_command() {
        let hint = match_slash_command_hint("/file");
        assert!(hint.contains("/file"));
    }

    #[test]
    fn view_mode_help_rows_include_every_view_mode() {
        let rows = view_mode_help_rows();
        assert_eq!(rows.len(), 9);
        assert!(rows.iter().any(|row| row.contains("Chat")));
        assert!(rows.iter().any(|row| row.contains("Swarm")));
        assert!(rows.iter().any(|row| row.contains("Ralph")));
        assert!(rows.iter().any(|row| row.contains("Bus Log")));
        assert!(rows.iter().any(|row| row.contains("Protocol")));
        assert!(rows.iter().any(|row| row.contains("Session Picker")));
        assert!(rows.iter().any(|row| row.contains("Model Picker")));
        assert!(rows.iter().any(|row| row.contains("Agent Picker")));
        assert!(rows.iter().any(|row| row.contains("File Picker")));
    }

    #[test]
    fn normalize_local_model_ref_adds_prefix() {
        assert_eq!(
            normalize_local_model_ref("qwen3.5-9b"),
            "local_cuda/qwen3.5-9b"
        );
        assert_eq!(
            normalize_local_model_ref("local_cuda/qwen3.5-9b"),
            "local_cuda/qwen3.5-9b"
        );
    }

    #[test]
    fn normalize_easy_command_maps_go_to_autochat() {
        assert_eq!(
            normalize_easy_command("/go build a calculator"),
            "/autochat 3 build a calculator"
        );
    }

    #[test]
    fn normalize_easy_command_maps_go_count_and_task() {
        assert_eq!(
            normalize_easy_command("/go 4 build a calculator"),
            "/autochat 4 build a calculator"
        );
    }

    #[test]
    fn normalize_easy_command_maps_go_count_only_to_demo_task() {
        assert_eq!(
            normalize_easy_command("/go 4"),
            format!("/autochat 4 {AUTOCHAT_QUICK_DEMO_TASK}")
        );
    }

    #[test]
    fn slash_hint_handles_command_with_args() {
        let hint = match_slash_command_hint("/go 4");
        assert!(hint.contains("/go"));
    }

    #[test]
    fn parse_autochat_args_supports_default_count() {
        let parsed = crate::autochat::parse_autochat_request(
            "build a calculator",
            3,
            AUTOCHAT_QUICK_DEMO_TASK,
        );
        assert_eq!(
            parsed.map(|value| (value.agent_count, value.task)),
            Some((3, "build a calculator".to_string()))
        );
    }

    #[test]
    fn parse_autochat_args_supports_explicit_count() {
        let parsed = crate::autochat::parse_autochat_request(
            "4 build a calculator",
            3,
            AUTOCHAT_QUICK_DEMO_TASK,
        );
        assert_eq!(
            parsed.map(|value| (value.agent_count, value.task)),
            Some((4, "build a calculator".to_string()))
        );
    }

    #[test]
    fn parse_autochat_args_count_only_uses_quick_demo_task() {
        let parsed = crate::autochat::parse_autochat_request("4", 3, AUTOCHAT_QUICK_DEMO_TASK);
        assert_eq!(
            parsed.map(|value| (value.agent_count, value.task)),
            Some((4, AUTOCHAT_QUICK_DEMO_TASK.to_string()))
        );
    }

    #[test]
    fn normalize_for_convergence_ignores_case_and_punctuation() {
        let a = normalize_for_convergence("Done! Next Step: Add tests.");
        let b = normalize_for_convergence("done next step add tests");
        assert_eq!(a, b);
    }

    #[test]
    fn extract_semantic_handoff_prefers_semantic_payload_answer() {
        let payload = r#"{"kind":"semantic","file":"relay_handoff","answer":"Ship phase 1, test API edge cases, then hand off to infra."}"#;
        assert_eq!(
            extract_semantic_handoff_from_rlm(payload),
            "Ship phase 1, test API edge cases, then hand off to infra."
        );
    }

    #[test]
    fn extract_semantic_handoff_falls_back_to_trimmed_raw_answer() {
        assert_eq!(
            extract_semantic_handoff_from_rlm("  plain relay output  "),
            "plain relay output"
        );
    }

    #[test]
    fn agent_avatar_is_stable_and_ascii() {
        let avatar = agent_avatar("planner");
        assert_eq!(avatar, agent_avatar("planner"));
        assert!(avatar.is_ascii());
        assert!(avatar.starts_with('[') && avatar.ends_with(']'));
    }

    #[test]
    fn relay_handoff_line_shows_avatar_labels() {
        let line = format_relay_handoff_line("relay-1", 2, "planner", "coder");
        assert!(line.contains("relay relay-1"));
        assert!(line.contains("@planner"));
        assert!(line.contains("@coder"));
        assert!(line.contains('['));
    }

    #[test]
    fn relay_handoff_line_formats_user_sender() {
        let line = format_relay_handoff_line("relay-2", 1, "user", "planner");
        assert!(line.contains("[you]"));
        assert!(line.contains("@planner"));
    }

    #[test]
    fn planner_profile_has_expected_personality() {
        let profile = agent_profile("auto-planner");
        assert_eq!(profile.codename, "Strategist");
        assert!(profile.profile.contains("decomposition"));
    }

    #[test]
    fn formatted_identity_includes_codename() {
        let identity = format_agent_identity("auto-coder");
        assert!(identity.contains("@auto-coder"));
        assert!(identity.contains("‹Forge›"));
    }

    #[test]
    fn normalize_minio_endpoint_strips_login_path() {
        assert_eq!(
            normalize_minio_endpoint("http://192.168.50.223:9001/login"),
            "http://192.168.50.223:9001"
        );
    }

    #[test]
    fn normalize_minio_endpoint_adds_default_scheme() {
        assert_eq!(
            normalize_minio_endpoint("192.168.50.223:9000"),
            "http://192.168.50.223:9000"
        );
    }

    #[test]
    fn fallback_endpoint_maps_console_port_to_s3_port() {
        assert_eq!(
            minio_fallback_endpoint("http://192.168.50.223:9001"),
            Some("http://192.168.50.223:9000".to_string())
        );
        assert_eq!(minio_fallback_endpoint("http://192.168.50.223:9000"), None);
    }

    #[test]
    fn secure_environment_detection_respects_explicit_flags() {
        assert!(is_secure_environment_from_values(Some(true), None, None));
        assert!(!is_secure_environment_from_values(
            Some(false),
            Some(true),
            Some("secure")
        ));
    }

    #[test]
    fn secure_environment_detection_uses_environment_name_fallback() {
        assert!(is_secure_environment_from_values(None, None, Some("PROD")));
        assert!(is_secure_environment_from_values(
            None,
            None,
            Some("production")
        ));
        assert!(!is_secure_environment_from_values(None, None, Some("dev")));
    }

    #[test]
    fn minimax_m25_pricing_estimate_matches_rates() {
        let cost = estimate_cost("minimax/MiniMax-M2.5", 1_000_000, 1_000_000)
            .expect("MiniMax M2.5 cost should be available");
        assert!((cost - 1.5).abs() < 1e-9); // $0.3 + $1.2 = $1.5
    }

    #[test]
    fn minimax_m25_highspeed_pricing() {
        let cost = estimate_cost("MiniMax-M2.5-highspeed", 1_000_000, 1_000_000)
            .expect("MiniMax M2.5 Highspeed cost should be available");
        assert!((cost - 3.0).abs() < 1e-9); // $0.6 + $2.4 = $3.0
    }

    #[test]
    fn easy_go_command_detects_go_and_team_aliases() {
        assert!(is_easy_go_command("/go build indexing"));
        assert!(is_easy_go_command("/team 4 implement auth"));
        assert!(!is_easy_go_command("/autochat build indexing"));
    }

    #[test]
    fn next_go_model_toggles_between_glm_and_minimax() {
        assert_eq!(
            next_go_model(Some("zai/glm-5")),
            "minimax-credits/MiniMax-M2.5-highspeed"
        );
        assert_eq!(
            next_go_model(Some("z-ai/glm-5")),
            "minimax-credits/MiniMax-M2.5-highspeed"
        );
        assert_eq!(
            next_go_model(Some("minimax-credits/MiniMax-M2.5-highspeed")),
            "zai/glm-5"
        );
        assert_eq!(
            next_go_model(Some("unknown/model")),
            "minimax-credits/MiniMax-M2.5-highspeed"
        );
    }

    #[test]
    fn smart_switch_marks_transient_provider_errors_retryable() {
        assert!(is_retryable_provider_error(
            "OpenAI API error (429 Too Many Requests): rate limit exceeded"
        ));
        assert!(is_retryable_provider_error(
            "Gemini returned protocol error code 469 with no text payload"
        ));
        assert!(is_retryable_provider_error(
            "Anthropic API error: unknown error, 520 (1000) (Some(\"api_error\"))"
        ));
        assert!(is_retryable_provider_error(
            "Anthropic API error: unknown error, 798 (1000) (Some(\"api_error\"))"
        ));
    }

    #[test]
    fn smart_switch_ignores_non_retryable_bad_request_errors() {
        assert!(!is_retryable_provider_error(
            "OpenAI API error (400 Bad Request): Instructions are required"
        ));
    }

    #[test]
    fn smart_switch_normalizes_provider_aliases_and_model_keys() {
        assert_eq!(normalize_provider_alias("z-ai"), "zai");
        assert_eq!(
            smart_switch_model_key("  Minimax-Credits/MiniMax-M2.5-Highspeed "),
            "minimax-credits/minimax-m2.5-highspeed".to_string()
        );
    }
}
