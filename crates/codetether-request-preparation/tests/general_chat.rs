//! General chat bypasses project instructions without removing tool access.
#[path = "general_chat/support.rs"]
mod support;
use codetether_request_preparation::build_provider_step_state;

#[test]
fn general_chat_is_project_neutral_for_all_provider_prompt_paths() {
    let host = support::Fixture {
        cwd: support::repository(),
        general: true,
    };
    for provider in ["openai", "groq", "lmstudio", "local_cuda"] {
        let state = build_provider_step_state(&host, provider, "model");
        assert!(
            state
                .system_prompt
                .contains("general-purpose conversational assistant")
        );
        assert!(!state.system_prompt.contains("PROJECT_ONLY_SHOULD_NOT_LEAK"));
        assert!(!state.system_prompt.contains("CODING_TASK_ONLY"));
        assert!(
            !state
                .system_prompt
                .contains(&host.cwd.display().to_string())
        );
        assert_eq!(state.tool_definitions.len(), 1);
        if provider == "local_cuda" {
            assert!(state.system_prompt.contains("<tool_call>"));
        } else {
            assert_eq!(state.advertised_tool_definitions.len(), 1);
        }
    }
}
#[test]
fn coding_sessions_keep_project_context_and_task_instructions() {
    let host = support::Fixture {
        cwd: support::repository(),
        general: false,
    };
    let state = build_provider_step_state(&host, "openai", "model");
    assert!(state.system_prompt.contains("PROJECT_ONLY_SHOULD_NOT_LEAK"));
    assert!(state.system_prompt.contains("CODING_TASK_ONLY"));
}
