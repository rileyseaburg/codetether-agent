//! Answer classification regression.

use super::*;

#[test]
fn parse_count_result() {
    let answer = "Found 15 async functions";
    let format = FinalAnswerFormat::parse(answer);
    match format {
        FinalAnswerFormat::CountResult { count } => assert_eq!(count, 15),
        _ => panic!("Expected CountResult"),
    }
}
