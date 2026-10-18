use super::{Cli, Command};
use crate::config::AccessMode;
use clap::Parser;

#[test]
fn run_yolo_resolves_to_full_access_mode() {
    use super::run_config::effective_access_mode;
    let cli = Cli::parse_from(["codetether", "run", "hello", "--yolo"]);

    match cli.command {
        Some(Command::Run(args)) => {
            assert_eq!(effective_access_mode(&args), Some(AccessMode::Full));
        }
        other => panic!("expected run command, got {other:?}"),
    }
}
