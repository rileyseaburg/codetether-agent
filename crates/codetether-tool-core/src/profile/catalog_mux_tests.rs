use super::super::{retain_coding_tools, retain_mux_manager_tools};
use super::definition;

#[test]
fn legacy_mux_manager_profile_exposes_no_control_tools() {
    let retained = retain_mux_manager_tools(
        ["wait_agent", "agent", "mux_control"]
            .into_iter()
            .map(definition)
            .collect(),
    );
    assert!(retained.is_empty());
}

#[test]
fn coding_profile_excludes_mux_control() {
    let retained = retain_coding_tools(
        ["agent", "mux_control"]
            .into_iter()
            .map(definition)
            .collect(),
    );
    let names: Vec<_> = retained.iter().map(|tool| tool.name.as_str()).collect();
    assert_eq!(names, ["agent"]);
}
