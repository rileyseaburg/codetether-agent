//! Monarch uses the standard auth command tree, without credential arguments.
use crate::{AuthArgs, AuthCommand, MonarchCommand};
use clap::Parser;

#[test]
fn monarch_auth_defaults_to_login_and_supports_lifecycle_commands() {
    let args = AuthArgs::try_parse_from(["auth", "monarch"]).unwrap();
    assert!(matches!(args.command, AuthCommand::Monarch(args) if args.command.is_none()));
    for action in ["login", "status", "logout"] {
        let args = AuthArgs::try_parse_from(["auth", "monarch", action]).unwrap();
        let AuthCommand::Monarch(args) = args.command else {
            panic!("wrong auth route")
        };
        match action {
            "login" => assert!(matches!(args.command, Some(MonarchCommand::Login))),
            "status" => assert!(matches!(args.command, Some(MonarchCommand::Status))),
            "logout" => assert!(matches!(args.command, Some(MonarchCommand::Logout))),
            _ => unreachable!(),
        }
    }
    assert!(AuthArgs::try_parse_from(["auth", "monarch", "--token", "fixture-only"]).is_err());
}
