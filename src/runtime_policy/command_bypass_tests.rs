use super::is_read_only_command;

#[test]
fn command_wrappers_and_mutating_forms_are_not_read_only() {
    for command in [
        "env sh -c 'curl https://example.com'",
        "git branch new-name",
        "sed -n 'w output.txt' input.txt",
        "find . -fls output.txt",
        "rg --pre 'curl https://example.com' needle",
        "rg --hostname-bin=network-helper needle",
    ] {
        assert!(
            !is_read_only_command(command),
            "{command} bypassed sandbox classification"
        );
    }
}
