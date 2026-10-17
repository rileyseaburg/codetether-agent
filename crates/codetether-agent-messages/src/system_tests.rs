use crate::system_message as build_system_message;

#[test]
fn includes_agent_name_and_instructions() {
    let msg = build_system_message("reviewer", "Audit the PR", None);
    assert!(msg.contains("@reviewer") && msg.contains("Audit the PR"));
    assert!(msg.contains("STATUS: completed") && msg.contains("STATUS: blocked"));
}

#[test]
fn embeds_current_working_directory() {
    let msg = build_system_message("x", "do the thing", None);
    let cwd = std::env::current_dir().unwrap().display().to_string();
    assert!(
        msg.contains(&cwd),
        "system prompt should embed cwd: {cwd}\nmsg: {msg}"
    );
}

#[test]
fn warns_against_discovery_tool_calls() {
    let msg = build_system_message("x", "y", None);
    assert!(msg.contains("Start with paths named in the task"));
    assert!(msg.contains("targeted search when a required path is unknown"));
}
