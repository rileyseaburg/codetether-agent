use codetether_agent::tool::undo::UndoTool;
use codetether_agent::tool::Tool;
use serde_json::json;
use std::process::Command;
use tempfile::tempdir;

#[tokio::test]
async fn test_undo_tool_preview() {
    let temp_dir = tempdir().unwrap();
    let repo_path = temp_dir.path();
    
    // Initialize git repo
    Command::new("git")
        .args(["init"])
        .current_dir(repo_path)
        .output()
        .unwrap();
    
    Command::new("git")
        .args(["config", "user.email", "test@test.com"])
        .current_dir(repo_path)
        .output()
        .unwrap();
    
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo_path)
        .output()
        .unwrap();
    
    // Create initial commit
    std::fs::write(repo_path.join("README.md"), "# Initial").unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(repo_path)
        .output()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(repo_path)
        .output()
        .unwrap();
    
    // Create another commit
    std::fs::write(repo_path.join("README.md"), "# Modified").unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(repo_path)
        .output()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "Second commit"])
        .current_dir(repo_path)
        .output()
        .unwrap();
    
    // Change to the repo directory
    std::env::set_current_dir(repo_path).unwrap();
    
    let tool = UndoTool;
    let result = tool.execute(json!({"preview": true})).await.unwrap();
    
    assert!(result.success);
    assert!(result.output.contains("Would undo"));
    assert!(result.output.contains("Second commit"));
}

#[tokio::test]
async fn test_undo_tool_not_git_repo() {
    let temp_dir = tempdir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    let tool = UndoTool;
    let result = tool.execute(json!({})).await.unwrap();
    
    assert!(!result.success);
    assert!(result.output.contains("Not in a git repository"));
}

#[tokio::test]
async fn test_undo_tool_multiple_steps() {
    let temp_dir = tempdir().unwrap();
    let repo_path = temp_dir.path();
    
    // Initialize git repo
    Command::new("git")
        .args(["init"])
        .current_dir(repo_path)
        .output()
        .unwrap();
    
    Command::new("git")
        .args(["config", "user.email", "test@test.com"])
        .current_dir(repo_path)
        .output()
        .unwrap();
    
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo_path)
        .output()
        .unwrap();
    
    // Create multiple commits
    for i in 1..=3 {
        std::fs::write(repo_path.join("file.txt"), format!("Content {}", i)).unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(repo_path)
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", &format!("Commit {}", i)])
            .current_dir(repo_path)
            .output()
            .unwrap();
    }
    
    std::env::set_current_dir(repo_path).unwrap();
    
    let tool = UndoTool;
    let result = tool.execute(json!({"steps": 2, "preview": true})).await.unwrap();
    
    assert!(result.success);
    assert!(result.output.contains("Would undo 2 commit(s)"));
    assert!(result.output.contains("Commit 3"));
    assert!(result.output.contains("Commit 2"));
}