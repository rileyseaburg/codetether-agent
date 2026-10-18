use super::Cli;
use clap::CommandFactory;

#[test]
fn root_command_retains_application_package_identity() {
    let command = Cli::command();
    assert_eq!(command.get_version(), Some(env!("CARGO_PKG_VERSION")));
    assert_eq!(
        command.get_about().map(ToString::to_string),
        Some(env!("CARGO_PKG_DESCRIPTION").to_string())
    );
}
