//! Mocked MCP response regressions for non-image output.

use super::result;
use serde_json::json;

#[test]
fn mcp_non_image_results_preserve_output_without_image_metadata() {
    for is_error in [false, true] {
        for content in [
            json!([]),
            json!([
                {"type": "text", "text": "plain"},
                {"type": "resource", "resource": {"uri": "file:///blob", "blob": "AQID"}}
            ]),
        ] {
            let empty = content.as_array().unwrap().is_empty();
            let response = serde_json::from_value(json!({
                "content": content, "isError": is_error
            }))
            .unwrap();
            let converted = result(response);
            assert_eq!(converted.success, !is_error);
            assert!(converted.metadata.is_empty());
            if empty {
                assert!(converted.output.is_empty());
            } else {
                let (text, resource) = converted.output.split_once('\n').unwrap();
                assert_eq!(text, "plain");
                assert_eq!(
                    serde_json::from_str::<serde_json::Value>(resource).unwrap(),
                    json!({"uri": "file:///blob", "blob": "AQID"})
                );
            }
        }
    }
}
