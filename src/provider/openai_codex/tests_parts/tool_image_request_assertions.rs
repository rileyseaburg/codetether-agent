fn assert_tool_image_wire(body: Value) {
    let wire: Value = serde_json::from_slice(&serde_json::to_vec(&body).unwrap()).unwrap();
    let input = wire["input"].as_array().unwrap();
    assert!(input.iter().filter(|item| item["role"] == "user").all(|item| {
        item["content"].as_array().is_none_or(|parts| {
            parts.iter().all(|part| part["type"] != "input_image")
        })
    }));
    let outputs: Vec<_> = input
        .iter()
        .filter(|item| item["type"] == "function_call_output")
        .collect();
    assert_eq!(outputs.len(), 2);
    for (output, (id, url)) in outputs.iter().zip([
        ("pixel-call", USER_IMAGE_DATA_URL),
        ("remote-call", USER_IMAGE_REMOTE_URL),
    ]) {
        assert_eq!(output["call_id"], id);
        let parts = output["output"].as_array().unwrap();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0]["type"], "input_image");
        assert_eq!(parts[0]["image_url"], url);
        assert!(!parts[1]["text"].as_str().unwrap().contains("base64"));
    }
}