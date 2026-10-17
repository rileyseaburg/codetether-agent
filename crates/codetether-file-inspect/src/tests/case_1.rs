//! Existing traversal regression case.
use super::*;

#[tokio::test]
async fn test_build_tree_no_partial_output_on_error() {
    // Use a path that doesn't exist
    let non_existent = PathBuf::from("/another/nonexistent/path");
    let mut output = Vec::new();
    let mut file_count = 0;
    let mut dir_count = 0;

    let initial_output_len = output.len();
    let initial_file_count = file_count;
    let initial_dir_count = dir_count;

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

    // Verify error was returned
    assert!(result.is_err());

    // Verify output was not modified (no partial tree building)
    assert_eq!(
        output.len(),
        initial_output_len,
        "Output should not be modified on error"
    );
    assert_eq!(
        file_count, initial_file_count,
        "File count should not be modified on error"
    );
    assert_eq!(
        dir_count, initial_dir_count,
        "Dir count should not be modified on error"
    );
}
