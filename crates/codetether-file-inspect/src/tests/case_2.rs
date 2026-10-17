//! Existing traversal regression case.
use super::*;

#[tokio::test]
async fn test_build_tree_success_with_temp_dir() {
    // Create a temporary directory structure
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create some files and directories
    tokio::fs::create_dir(temp_path.join("subdir"))
        .await
        .expect("Failed to create subdir");
    tokio::fs::write(temp_path.join("file1.txt"), "content")
        .await
        .expect("Failed to write file1");
    tokio::fs::write(temp_path.join("subdir").join("file2.txt"), "content")
        .await
        .expect("Failed to write file2");

    let mut output = Vec::new();
    let mut file_count = 0;
    let mut dir_count = 0;

    let result = build_tree(
        temp_path,
        "",
        0,
        3,
        false,
        false,
        false, // Don't respect gitignore for this test
        &mut output,
        &mut file_count,
        &mut dir_count,
    )
    .await;

    // Should succeed
    assert!(
        result.is_ok(),
        "Expected success for valid directory: {:?}",
        result
    );

    // Should have found files and directories
    assert!(
        file_count >= 2,
        "Should have found at least 2 files, found: {}",
        file_count
    );
    assert!(
        dir_count >= 1,
        "Should have found at least 1 directory, found: {}",
        dir_count
    );
    assert!(!output.is_empty(), "Output should not be empty");
}
