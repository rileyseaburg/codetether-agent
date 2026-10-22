use clap::Parser;
use codetether_rust_build::{RustArgs, plan};

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    rust: RustArgs,
}

#[test]
fn clap_preserves_program_arguments() {
    let cli = Cli::try_parse_from([
        "rust",
        "run",
        "--package",
        "app",
        "--arg=--help",
        "--arg=two words",
    ])
    .unwrap();
    assert_eq!(cli.rust.toolchain, "stable");
    assert_eq!(
        plan(&cli.rust).unwrap().args,
        [
            "+stable",
            "ra-jit",
            "run",
            "--package",
            "app",
            "--",
            "--help",
            "two words"
        ]
    );
}

#[test]
fn clap_rejects_arbitrary_cargo_flags() {
    for argv in [
        vec!["rust", "build", "--release"],
        vec!["rust", "check", "--config", "bad"],
        vec!["rust", "enable"],
        vec!["rust", "run", "--features", "bad"],
    ] {
        assert!(Cli::try_parse_from(argv).is_err());
    }
}
