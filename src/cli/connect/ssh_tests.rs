//! Unit tests for the `codetether connect` SSH command builder.

use super::args::ConnectArgs;
use super::ssh;

fn sample_args() -> ConnectArgs {
    ConnectArgs {
        host: "vm.lan".into(),
        user: Some("riley".into()),
        port: 22,
        forward_port: 1455,
        remote_bin: "codetether".into(),
        provider: "bedrock".into(),
        remote_args: vec![],
        no_browser: false,
        skip_preflight: false,
    }
}

#[test]
fn target_includes_user_when_set() {
    assert_eq!(ssh::target(&sample_args()), "riley@vm.lan");
}

#[test]
fn target_is_bare_host_without_user() {
    let mut args = sample_args();
    args.user = None;
    assert_eq!(ssh::target(&args), "vm.lan");
}

#[test]
fn forward_spec_binds_loopback_both_sides() {
    assert_eq!(ssh::forward_spec(1455), "1455:127.0.0.1:1455");
}

#[test]
fn remote_command_runs_device_code_auth() {
    assert_eq!(
        ssh::remote_command(&sample_args()),
        "codetether auth bedrock --device-code"
    );
}

#[test]
fn remote_command_appends_extra_args() {
    let mut args = sample_args();
    args.remote_args = vec!["--save".into(), "--region".into(), "us-east-1".into()];
    assert_eq!(
        ssh::remote_command(&args),
        "codetether auth bedrock --device-code --save --region us-east-1"
    );
}
