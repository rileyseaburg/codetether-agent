//! Answer classification regression.

use super::*;

#[test]
fn parse_line_numbered_matches() {
    let answer = "42:async fn foo()\n100:pub struct Bar\n";
    let format = FinalAnswerFormat::parse(answer);
    match format {
        FinalAnswerFormat::LineNumberedMatches { matches } => {
            assert_eq!(matches.len(), 2);
            assert_eq!(matches[0], (42, "async fn foo()".to_string()));
            assert_eq!(matches[1], (100, "pub struct Bar".to_string()));
        }
        _ => panic!("Expected LineNumberedMatches"),
    }
}
