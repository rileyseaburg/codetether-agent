//! Regression for parse_ast_payload.

use super::*;

#[test]
fn parse_ast_payload() {
    let json = r#"{
            "kind": "ast",
            "file": "src/main.rs",
            "query": "functions",
            "results": [
                {"name": "process", "args": ["input: &str"], "return_type": "Result<String>"}
            ]
        }"#;

    let payload = FinalPayload::parse(json);
    match payload {
        FinalPayload::Ast(p) => {
            assert_eq!(p.file, "src/main.rs");
            assert_eq!(p.query, "functions");
            assert_eq!(p.results.len(), 1);
            assert_eq!(p.results[0].name, "process");
        }
        _ => panic!("Expected Ast payload"),
    }
}
