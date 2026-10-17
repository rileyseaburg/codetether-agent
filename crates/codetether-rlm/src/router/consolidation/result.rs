//! Completion formatting and token metrics without timing noise.
use crate::router::types::LoopOutcome;
#[test]
fn result_snapshot() {
    let mut values = Vec::new();
    for (answer, aborted, finished) in [
        ("", false, false),
        ("finished answer", false, true),
        ("partial", true, false),
    ] {
        let outcome = LoopOutcome {
            final_answer: finished.then(|| answer.into()),
            iterations: 2,
            subcalls: 1,
            aborted,
            last_error: Some("fixture".into()),
        };
        let result = super::super::auto_process_emit::build_result(
            answer,
            42,
            &outcome,
            std::time::Instant::now(),
            uuid::Uuid::nil(),
        );
        assert_eq!(
            result.stats.output_tokens,
            crate::RlmChunker::estimate_tokens(answer)
        );
        assert_eq!(result.success, !aborted && finished);
        let mut value = serde_json::to_value(result).unwrap();
        value["stats"]["elapsed_ms"] = 0.into();
        values.push(value);
    }
    println!("RESULTS={}", serde_json::to_string(&values).unwrap());
}
