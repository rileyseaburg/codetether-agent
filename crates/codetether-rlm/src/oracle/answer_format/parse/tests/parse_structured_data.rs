//! Answer classification regression.

use super::*;

#[test]
fn parse_structured_data() {
    let answer = r#"{"name": "foo", "args": ["x", "y"]}"#;
    let format = FinalAnswerFormat::parse(answer);
    match format {
        FinalAnswerFormat::StructuredData { data } => {
            assert_eq!(data["name"], "foo");
        }
        _ => panic!("Expected StructuredData"),
    }
}
