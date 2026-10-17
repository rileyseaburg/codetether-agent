#[test]
fn tool_image_metadata_reaches_http_and_websocket_requests() {
    for success in [true, false] {
        let request = tool_image_wire_request(success);
        assert_tool_image_wire(OpenAiCodexProvider::build_http_responses_body(&request));
        for backend in [ResponsesWsBackend::OpenAi, ResponsesWsBackend::ChatGptCodex] {
            assert_tool_image_wire(
                OpenAiCodexProvider::build_responses_ws_create_event_for_backend(
                    &request, "gpt-5.5", None, None, backend,
                ),
            );
        }
        assert!(
            request
                .messages
                .iter()
                .filter(|m| m.role == Role::User)
                .all(|message| {
                    message
                        .content
                        .iter()
                        .all(|part| !matches!(part, ContentPart::Image { .. }))
                })
        );
    }
}
