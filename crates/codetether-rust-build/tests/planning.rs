use codetether_rust_build::{RustAction as A, RustArgs, plan};

#[test]
fn every_action_has_fixed_argv() {
    for (action, expected) in [
        (A::Build, vec!["+stable", "ra-jit", "build"]),
        (A::Run, vec!["+stable", "ra-jit", "run"]),
        (A::Test, vec!["+stable", "ra-jit", "test"]),
        (A::Check, vec!["+stable", "check"]),
        (A::Status, vec!["+stable", "ra-jit", "status"]),
        (A::Stop, vec!["+stable", "ra-jit", "stop"]),
        (A::Doctor, vec!["+stable", "--version"]),
    ] {
        let input = RustArgs {
            action,
            ..Default::default()
        };
        let result = plan(&input).unwrap();
        assert_eq!(result.program, "cargo");
        assert_eq!(result.args, expected);
        assert_eq!(result.cwd, input.cwd);
    }
}

#[test]
fn selectors_and_trailing_argv_are_separate() {
    let input = RustArgs {
        action: A::Run,
        package: Some("my-app".into()),
        bin: Some("app".into()),
        toolchain: "1.99.0".into(),
        cwd: "missing directory".into(),
        args: vec!["--config".into(), "a;echo hello".into()],
    };
    assert_eq!(
        plan(&input).unwrap().args,
        [
            "+1.99.0",
            "ra-jit",
            "run",
            "--package",
            "my-app",
            "--bin",
            "app",
            "--",
            "--config",
            "a;echo hello"
        ]
    );
}

#[test]
fn selectors_reject_option_and_control_injection() {
    for value in ["", "-x", "+stable", "stable\n", "a b", "a/b", "a;id", "a\0"] {
        for field in 0..3 {
            let mut input = RustArgs::default();
            match field {
                0 => input.toolchain = value.into(),
                1 => input.package = Some(value.into()),
                _ => input.bin = Some(value.into()),
            }
            assert!(plan(&input).is_err(), "accepted {value:?}");
        }
    }
}

#[test]
fn unsupported_arguments_fail_closed() {
    for action in [A::Build, A::Check, A::Status, A::Stop, A::Doctor] {
        let input = RustArgs {
            action,
            args: vec!["--release".into()],
            ..Default::default()
        };
        assert!(plan(&input).is_err());
    }
    for action in [A::Status, A::Stop, A::Doctor] {
        let input = RustArgs {
            action,
            package: Some("app".into()),
            ..Default::default()
        };
        assert!(plan(&input).is_err());
    }
}
