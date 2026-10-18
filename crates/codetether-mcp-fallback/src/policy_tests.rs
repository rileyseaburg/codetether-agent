use super::policy_name;

#[test]
fn fallback_mutating_tools_map_to_runtime_policy_names() {
    assert_eq!(policy_name("run_command"), "bash");
    assert_eq!(policy_name("write_file"), "write");
}

#[test]
fn registry_tool_names_are_preserved() {
    assert_eq!(policy_name("read"), "read");
}
