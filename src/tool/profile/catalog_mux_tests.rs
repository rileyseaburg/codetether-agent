use super::super::retain_mux_manager_tools;
use super::definition;

#[test]
fn mux_manager_profile_exposes_only_mux_control() {
    let retained = retain_mux_manager_tools(
        ["wait_agent", "agent", "mux_control"]
            .into_iter()
            .map(definition)
            .collect(),
    );
    let names: Vec<_> = retained.iter().map(|tool| tool.name.as_str()).collect();
    assert_eq!(names, ["mux_control"]);
}
