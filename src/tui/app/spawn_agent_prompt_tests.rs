use super::system_prompt;

#[test]
fn preserves_only_the_visible_mission() {
    let prompt = system_prompt("reviewer", "Review the dashboard");
    assert_eq!(prompt, "Review the dashboard");
    assert!(!prompt.contains("Report-back contract"));
}

#[test]
fn creates_a_default_mission_when_empty() {
    let prompt = system_prompt("reviewer", "");
    assert!(prompt.contains("working as a sub-agent"));
}
