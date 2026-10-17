//! Feedback formatting is independent of any filesystem operation.
use crate::feedback::render_with_dir;

#[test]
fn failure_feedback_includes_directory_and_empty_output_marker() {
    let output = render_with_dir(
        "read",
        false,
        Some(std::path::Path::new("/workspace")),
        "  ",
    );
    assert!(output.contains("- status: failure"));
    assert!(output.contains("- working_dir: /workspace"));
    assert!(output.contains("- next: inspect the error"));
    assert!(output.ends_with("Output:\n<empty tool output>"));
}
