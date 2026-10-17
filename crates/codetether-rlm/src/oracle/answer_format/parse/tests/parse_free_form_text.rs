//! Answer classification regression.

use super::*;

#[test]
fn parse_free_form_text() {
    let answer = "This function handles error cases by using the ? operator";
    let format = FinalAnswerFormat::parse(answer);
    match format {
        FinalAnswerFormat::FreeFormText { text } => {
            assert!(text.contains("error cases"));
        }
        _ => panic!("Expected FreeFormText"),
    }
}
