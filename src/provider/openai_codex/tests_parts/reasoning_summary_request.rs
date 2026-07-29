#[test]
fn http_body_requests_reasoning_summary() {
    let request = reasoning_probe_request("gpt-5.6-luna:high");
    let body = OpenAiCodexProvider::build_http_responses_body(&request);
    assert_reasoning_shape(
        &body,
        "summary keeps the stream warm during long reasoning phases",
    );
}

#[test]
fn ws_create_event_requests_reasoning_summary() {
    let request = reasoning_probe_request("gpt-5.6-luna");
    let event = OpenAiCodexProvider::build_responses_ws_create_event(
        &request,
        "gpt-5.6-luna",
        Some(ThinkingLevel::High),
        None,
    );
    assert_reasoning_shape(&event, "both transports must request the same shape");
}

#[test]
fn reasoning_summary_events_refresh_the_watchdog() {
    let mut chunks = Vec::new();
    OpenAiCodexProvider::parse_responses_event(
        &mut ResponsesSseParser::default(),
        &json!({
            "type": "response.reasoning_summary_text.delta",
            "delta": "considering the constraints",
        }),
        &mut chunks,
    );
    let matched = matches!(
        &chunks[..],
        [StreamChunk::Thinking(text)] if text == "considering the constraints"
    );
    assert!(matched, "summary deltas must be Thinking activity: {chunks:?}");
}
