use super::{line_limit, render};
use crate::swarm::tool_policy::{SystemPromptInput, system_prompt};

#[test]
fn extracts_explicit_nonblank_line_limit() {
    let policy = "### 50-Line File Limit\n- STRICT 50-line maximum per file";
    assert_eq!(line_limit(policy), Some(50));
}

#[test]
fn renders_objective_srp_and_measurement_contract() {
    let prompt = render(false, Some(50));
    assert!(prompt.contains("at most 50 non-comment, nonblank lines"));
    assert!(prompt.contains("one clear responsibility"));
    assert!(prompt.contains("Run the repository file-limit gate"));
    assert!(prompt.contains("each changed file with its measured code-line count"));
    assert!(prompt.contains("Report every exception explicitly"));
}

#[test]
fn system_prompt_accepts_an_exact_limit() {
    let prompt = system_prompt(SystemPromptInput {
        specialty: "refactor",
        subtask_id: "one",
        working_dir: ".",
        model: "provider/model",
        prd_filename: "prd.json",
        agents_md: "",
        line_limit: Some(50),
        read_only: false,
    });
    assert!(prompt.contains("at most 50 non-comment, nonblank lines"));
}

#[test]
fn does_not_burden_read_only_tasks() {
    assert!(render(true, Some(50)).is_empty());
}

#[test]
fn ignores_unrelated_numbers() {
    assert_eq!(line_limit("Use Rust 2024 and retry 3 times"), None);
}
