use super::light_prompt::prefers_light_system_prompt;

/// Environment state is process-global, so all cases run in one test to
/// avoid races with other parallel tests.
#[test]
fn light_prompt_selection_honors_provider_and_opt_in_flag() {
    unsafe { std::env::remove_var("CODETETHER_LIGHT_SYSTEM_PROMPT") };
    for provider in ["local-cuda", "local_cuda", "localcuda"] {
        assert!(prefers_light_system_prompt(provider));
    }
    assert!(!prefers_light_system_prompt("openai"));
    assert!(!prefers_light_system_prompt("huggingface"));

    unsafe { std::env::set_var("CODETETHER_LIGHT_SYSTEM_PROMPT", "1") };
    assert!(prefers_light_system_prompt("huggingface"));

    unsafe { std::env::set_var("CODETETHER_LIGHT_SYSTEM_PROMPT", "true") };
    assert!(prefers_light_system_prompt("openai"));

    unsafe { std::env::set_var("CODETETHER_LIGHT_SYSTEM_PROMPT", "0") };
    assert!(!prefers_light_system_prompt("openai"));

    unsafe { std::env::remove_var("CODETETHER_LIGHT_SYSTEM_PROMPT") };
}
