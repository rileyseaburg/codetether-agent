//! Existing traversal regression case.
use super::*;

#[tokio::test]
async fn test_build_tree_propagates_read_dir_error() {
    // Use a path that doesn't exist to trigger an error
    let non_existent = PathBuf::from("/nonexistent/path/that/does/not/exist");
    let mut output = Vec::new();
    let mut file_count = 0;
    let mut dir_count = 0;

    let result = build_tree(
        &non_existent,
        "",
        0,
        3,
        false,
        false,
        true,
        &mut output,
        &mut file_count,
        &mut dir_count,
    )
    .await;

    // Should return an error, not Ok(())
    assert!(result.is_err(), "Expected error for non-existent directory");

    // Error message should contain context about the path
    let err = result.unwrap_err();
    let err_msg = err.to_string();
    assert!(
        err_msg.contains("Failed to read directory"),
        "Error should contain context message, got: {err_msg}"
    );
    assert!(
        err_msg.contains("/nonexistent/path/that/does/not/exist"),
        "Error should contain the path, got: {err_msg}"
    );
}
