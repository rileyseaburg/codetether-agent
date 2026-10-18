use crate::{
    matching::{best_workspace_match, shared_path_suffix_score},
    title::summarize_title,
    types::WorkspaceSummary,
};

#[test]
fn picks_longest_workspace_prefix_match() {
    let workspaces = vec![
        WorkspaceSummary {
            id: "root".to_string(),
            path: Some("/home/riley".to_string()),
        },
        WorkspaceSummary {
            id: "repo".to_string(),
            path: Some("/home/riley/A2A-Server-MCP".to_string()),
        },
    ];

    let matched = best_workspace_match("/home/riley/A2A-Server-MCP/subdir", &workspaces)
        .expect("expected workspace match");
    assert_eq!(matched.id, "repo");
}

#[test]
fn summarize_title_truncates_long_prompts() {
    let prompt = "x".repeat(120);
    let title = summarize_title(&prompt);
    assert!(title.ends_with("..."));
    assert_eq!(title.chars().count(), 83);
}

#[test]
fn shared_path_suffix_score_handles_container_prefix_remap() {
    assert_eq!(
        shared_path_suffix_score(
            "/home/riley/A2A-Server-MCP",
            "/app/home/riley/A2A-Server-MCP"
        ),
        3
    );
}

#[test]
fn falls_back_to_unique_suffix_match() {
    let workspaces = vec![
        WorkspaceSummary {
            id: "repo".to_string(),
            path: Some("/app/home/riley/A2A-Server-MCP".to_string()),
        },
        WorkspaceSummary {
            id: "other".to_string(),
            path: Some("/srv/spotlessbinco".to_string()),
        },
    ];

    let matched = best_workspace_match("/home/riley/A2A-Server-MCP", &workspaces)
        .expect("expected suffix workspace match");
    assert_eq!(matched.id, "repo");
}
