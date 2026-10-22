use crate::clamp_tokens;
#[test]
fn long_early_sections_do_not_erase_pending_action_or_latest_correction() {
    let source = format!(
        "## PRIMARY GOAL\nBuild the dev funnels.\n## FINDINGS\n{}\n## CORRECTIONS\nUse the local server.\n## NEXT ACTION\nRun the existing Playwright test.",
        "Large source detail. ".repeat(2000)
    );
    let bounded = clamp_tokens(&source, 512);
    assert!(bounded.contains("Build the dev funnels."));
    assert!(bounded.contains("Use the local server."));
    assert!(bounded.contains("Run the existing Playwright test."));
    assert!(bounded.len() <= 512 * 4);
    assert!(bounded.contains("excerpt omitted"));
}
#[test]
fn unstructured_unicode_summary_retains_both_ends() {
    let source = format!("Task: build funnels. {} Next: run test.", "🦀".repeat(2000));
    let bounded = clamp_tokens(&source, 128);
    assert!(bounded.starts_with("Task: build funnels."));
    assert!(bounded.contains("Next: run test."));
    assert!(bounded.len() <= 512);
}
