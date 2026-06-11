pub(super) fn policy_name(name: &str) -> &str {
    match name {
        "run_command" => "bash",
        "write_file" => "write",
        _ => name,
    }
}

#[cfg(test)]
#[path = "tool_policy_names_tests.rs"]
mod tests;
