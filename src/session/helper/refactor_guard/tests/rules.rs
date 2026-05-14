use super::super::{rules, types::*};

use super::support::file;

#[test]
fn new_oversized_file_is_rejected() {
    let mut report = GuardReport::new(vec![file("x.tsx", None, &"x\n".repeat(81), 80)]);
    rules::apply(&mut report);
    assert_eq!(report.violations.len(), 1);
}

#[test]
fn oversized_file_can_shrink() {
    let mut report = GuardReport::new(vec![file(
        "x.tsx",
        Some(&"x\n".repeat(100)),
        &"x\n".repeat(90),
        80,
    )]);
    rules::apply(&mut report);
    assert!(report.violations.is_empty());
}

#[test]
fn wrapper_displacement_is_rejected() {
    let wrapper = "import { VideoEditorPageRoot } from './VideoEditorPageRoot';\nexport const VideoEditorPage = () => <VideoEditorPageRoot />;";
    let mut report = GuardReport::new(vec![
        file("VideoEditorPage.tsx", Some(&"x\n".repeat(100)), wrapper, 80),
        file("VideoEditorPageRoot.tsx", None, &"x\n".repeat(100), 80),
    ]);
    rules::apply(&mut report);
    assert!(
        report
            .violations
            .iter()
            .any(|v| v.message.contains("became a wrapper"))
    );
}

#[test]
fn real_extraction_under_limit_is_allowed() {
    let mut report = GuardReport::new(vec![file("Part.tsx", None, &"x\n".repeat(40), 80)]);
    rules::apply(&mut report);
    assert!(report.violations.is_empty());
}
