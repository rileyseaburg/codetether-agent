//! Tests for [`crate::session::index_produce::summary_gate`].

use crate::rlm::{RlmResult, RlmStats};

use super::summary_gate::try_bounded_summary;

fn rlm_result(success: bool, body: &str, input_tokens: usize, output_tokens: usize) -> RlmResult {
    RlmResult {
        processed: body.to_string(),
        stats: RlmStats {
            input_tokens,
            output_tokens,
            ..Default::default()
        },
        success,
        error: None,
        trace: None,
        trace_id: None,
    }
}

#[test]
fn rejects_unsuccessful_results() {
    let r = rlm_result(false, "ignored", 1024, 256);
    assert!(try_bounded_summary(&r, 512).is_none());
}

#[test]
fn rejects_tiny_output_for_large_input() {
    let r = rlm_result(true, "summary", 770_000, 12);
    assert!(try_bounded_summary(&r, 512).is_none());
}

#[test]
fn rejects_production_770k_to_180_case() {
    let r = rlm_result(true, "body irrelevant for the count", 770_000, 180);
    assert!(try_bounded_summary(&r, 512).is_none());
}

#[test]
fn accepts_plausible_output() {
    let body = "a real-looking summary that exceeds the degraded floor".repeat(4);
    let r = rlm_result(true, &body, 10_000, 80);
    assert!(try_bounded_summary(&r, 512).is_some());
}

#[test]
fn rejects_empty_body_even_when_successful() {
    let r = rlm_result(true, "   \n\n   ", 10_000, 80);
    assert!(try_bounded_summary(&r, 512).is_none());
}
