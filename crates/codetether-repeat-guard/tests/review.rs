mod support;
use support::*;
#[test]
fn novelty_and_reload_cannot_postpone_review() {
    let mut ledger = Ledger::default();
    for i in 0..40 {
        outcome(
            &mut ledger,
            i,
            "read",
            json!({"path":i}),
            true,
            &format!("source {i}"),
        );
        ledger = serde_json::from_str(&serde_json::to_string(&ledger).unwrap()).unwrap();
        ledger.human_input(&text(
            "Continue working toward the active thread goal. <objective>research</objective>",
        ));
        let review = ledger.take_review(15);
        assert_eq!(review.is_some(), i == 14 || i == 29);
        if let Some(review) = review {
            assert!(review.contains("15 completed"));
            assert!(review.contains("0 failures, 0 exact repeats"));
        }
    }
    assert_eq!(ledger.novel_observations, 40);
}
