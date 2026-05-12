//! Tests for FinalAnswerFormat parsing.

use crate::oracle::answer_format::FinalAnswerFormat;

#[test]
fn parse_line_numbered_matches() {
    let answer = "42:async fn foo()\n100:pub struct Bar\n";
    match FinalAnswerFormat::parse(answer) {
        FinalAnswerFormat::LineNumberedMatches { matches } => {
            assert_eq!(matches.len(), 2);
            assert_eq!(matches[0], (42, "async fn foo()".to_string()));
            assert_eq!(matches[1], (100, "pub struct Bar".to_string()));
        }
        _ => panic!("Expected LineNumberedMatches"),
    }
}

#[test]
fn parse_count_result() {
    match FinalAnswerFormat::parse("Found 15 async functions") {
        FinalAnswerFormat::CountResult { count } => assert_eq!(count, 15),
        _ => panic!("Expected CountResult"),
    }
}

#[test]
fn parse_structured_data() {
    let answer = r#"{"name": "foo", "args": ["x", "y"]}"#;
    match FinalAnswerFormat::parse(answer) {
        FinalAnswerFormat::StructuredData { data } => assert_eq!(data["name"], "foo"),
        _ => panic!("Expected StructuredData"),
    }
}

#[test]
fn parse_free_form_text() {
    let answer = "This function handles error cases";
    match FinalAnswerFormat::parse(answer) {
        FinalAnswerFormat::FreeFormText { text } => assert!(text.contains("error cases")),
        _ => panic!("Expected FreeFormText"),
    }
}
