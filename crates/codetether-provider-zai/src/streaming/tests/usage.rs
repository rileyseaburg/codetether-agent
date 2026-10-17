//! Keep the application and injected stream usage conventions separate.
use super::*;

#[test]
fn usage_modes_preserve_total_and_uncached_prompt_counts() {
    let text = "data: {\"choices\":[],\"usage\":{\"prompt_tokens\":120,\"completion_tokens\":45,\"total_tokens\":165,\"prompt_tokens_details\":{\"cached_tokens\":64}}}\n";
    let mut application = super::super::state::State {
        total_prompt_usage: true,
        ..Default::default()
    };
    let mut injected = super::super::state::State::default();
    super::super::buffer::append(&mut application, text);
    super::super::buffer::append(&mut injected, text);
    assert_eq!(application.final_usage.as_ref().unwrap().prompt_tokens, 120);
    assert_eq!(injected.final_usage.as_ref().unwrap().prompt_tokens, 56);
    assert_eq!(
        application.final_usage.as_ref().unwrap().cache_read_tokens,
        Some(64)
    );
}
