#[test]
fn tool_image_packed_results_keep_call_association_and_drop_orphans() {
    let result = |id: &str| ContentPart::ToolResult {
        tool_call_id: id.into(), content: "image inspected".into(),
    };
    let message = Message {
        role: Role::Tool,
        content: vec![
            result("first"), user_image_part(USER_IMAGE_DATA_URL),
            result("orphan"), user_image_part("https://example.com/do-not-forward.png"),
            result("second"), user_image_part(USER_IMAGE_REMOTE_URL),
        ],
    };
    let calls = std::collections::HashSet::from(["first".into(), "second".into()]);
    let mut input = Vec::new();
    OpenAiCodexProvider::append_responses_tool(&message, &mut input, &calls);
    assert_eq!(input.len(), 2);
    assert_eq!(input[0]["call_id"], "first");
    assert_eq!(input[0]["output"][0]["image_url"], USER_IMAGE_DATA_URL);
    assert_eq!(input[1]["call_id"], "second");
    assert_eq!(input[1]["output"][0]["image_url"], USER_IMAGE_REMOTE_URL);
    for output in &input {
        assert_eq!(output["output"].as_array().unwrap().len(), 2);
    }
    assert!(!serde_json::to_string(&input).unwrap().contains("do-not-forward"));
}