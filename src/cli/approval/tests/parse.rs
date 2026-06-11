use super::super::args::{ApprovalArgs, ApprovalCommand};
use clap::Parser;

#[test]
fn clap_accepts_operator_approval_shape() {
    let args = ApprovalArgs::try_parse_from([
        "approval", "approve", "abc", "--actor", "ops", "--reason", "reviewed",
    ])
    .expect("parse approval args");
    match args.command {
        ApprovalCommand::Approve(args) => assert_eq!(args.id, "abc"),
        ApprovalCommand::List | ApprovalCommand::Show { .. } | ApprovalCommand::Deny(_) => {
            panic!("wrong subcommand")
        }
    }
}
