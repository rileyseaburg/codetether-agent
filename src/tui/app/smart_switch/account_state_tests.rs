//! Tests for account-exhaustion detection and provider filtering.

use super::account_state::is_provider_account_exhausted;
use super::provider_filter::filter_out_provider;
use super::retry::maybe_schedule_smart_switch_retry;

const GLM_EXPIRED: &str = "429 Too Many Requests {\"error\":{\"code\":\"1309\",\
\"message\":\"Your GLM Coding Plan package has expired and is temporarily \
unavailable. You can resume after renewing the subscription.\"}}";

#[test]
fn glm_expired_is_account_exhaustion() {
    assert!(is_provider_account_exhausted(GLM_EXPIRED));
}

#[test]
fn plain_rate_limit_is_not_account_exhaustion() {
    assert!(!is_provider_account_exhausted("429 rate limit, retry soon"));
}

#[test]
fn filter_drops_exhausted_provider_candidates() {
    let candidates = vec![
        "zai/glm-5.1".to_string(),
        "zai/glm-5".to_string(),
        "minimax/MiniMax-M3".to_string(),
    ];
    let kept = filter_out_provider(candidates, Some("zai"));
    assert_eq!(kept, vec!["minimax/MiniMax-M3".to_string()]);
}

#[test]
fn expired_zai_plan_switches_off_zai() {
    let available = vec!["zai".to_string(), "minimax".to_string()];
    let retry = maybe_schedule_smart_switch_retry(
        GLM_EXPIRED,
        Some("zai/glm-5"),
        Some("zai"),
        &available,
        "hello",
        0,
        &[],
    )
    .expect("should schedule a retry");
    assert!(
        !retry.target_model.starts_with("zai/"),
        "expired zai plan must not retry on zai, got {}",
        retry.target_model
    );
}
