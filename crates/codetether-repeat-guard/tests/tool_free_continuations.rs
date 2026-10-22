mod support;
use codetether_provider_core::Role;
use support::*;

#[test]
fn repeated_plans_across_restart_neither_advance_evidence_nor_reset_failures() {
    let mut ledger = Ledger::default();
    for i in 0..3 {
        outcome(&mut ledger, i, "edit", json!({}), false, "target missing");
    }
    let observations = ledger.observations;
    let novel = ledger.novel_observations;
    for i in 0..20 {
        let continuation = text(
            "Continue working toward the active thread goal.\n<objective>Repair target</objective>",
        );
        ledger.human_input(&continuation);
        ledger.observe_message(&continuation, 6 + i * 2);
        let mut plan = text("I will inspect the target and repair it next.");
        plan.role = Role::Assistant;
        ledger.observe_message(&plan, 7 + i * 2);
        ledger = serde_json::from_str(&serde_json::to_string(&ledger).unwrap()).unwrap();
        assert_eq!(ledger.observations, observations);
        assert_eq!(ledger.novel_observations, novel);
        assert!(ledger.blocked_edit("edit", &json!({})).is_some());
    }
    outcome(
        &mut ledger,
        100,
        "read",
        json!({"path":"target"}),
        true,
        "new contents",
    );
    assert_eq!(ledger.novel_observations, novel + 1);
    assert!(ledger.blocked_edit("edit", &json!({})).is_none());
}
