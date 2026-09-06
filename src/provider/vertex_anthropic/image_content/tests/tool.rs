//! Tool images stay inside the correct result, including error and empty text.
use super::super::fixtures::*;
use crate::provider::Role;

#[test]
fn distinct_tool_images_preserve_association_and_error_text() {
    let out = convert(&[
        message(Role::Assistant, vec![call("a"), call("b")]),
        message(
            Role::Tool,
            vec![
                result("a", "capture"),
                image("data:image/png;base64,YQ=="),
                result("b", "Error: partial capture"),
                image("data:image/jpeg;base64,Yg=="),
            ],
        ),
    ]);
    let messages = &out["messages"];
    assert_eq!(messages.as_array().unwrap().len(), 2);
    let results = &messages[1]["content"];
    assert_eq!(results.as_array().unwrap().len(), 2);
    for (i, id, data) in [(0, "a", "YQ=="), (1, "b", "Yg==")] {
        assert_eq!(results[i]["type"], "tool_result");
        assert_eq!(results[i]["tool_use_id"], id);
        assert_eq!(results[i]["content"][1]["source"]["data"], data);
    }
    assert_eq!(results[1]["content"][0]["text"], "Error: partial capture");
}
#[test]
fn image_only_tool_result_has_no_empty_text_block() {
    let out = convert(&[message(
        Role::Tool,
        vec![result("a", ""), image("data:image/png;base64,YQ==")],
    )]);
    let blocks = &out["messages"][0]["content"][0]["content"];
    assert_eq!(blocks.as_array().unwrap().len(), 1);
    assert_eq!(blocks[0]["type"], "image");
}
