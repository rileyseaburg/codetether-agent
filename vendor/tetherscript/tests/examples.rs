use std::process::Command;

fn run_example(path: &str) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_tetherscript"))
        .args(["run", path])
        .output()
        .unwrap_or_else(|err| panic!("failed to run {path}: {err}"))
}

fn assert_example(path: &str, expected_stdout: &str) {
    let output = run_example(path);
    assert!(
        output.status.success(),
        "{path} failed with status {:?}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected_stdout);
}

#[test]
fn hello_output_matches_golden() {
    assert_example(
        "examples/hello.tether",
        include_str!("examples/hello.stdout"),
    );
}

#[test]
fn fib_output_matches_golden() {
    assert_example("examples/fib.tether", include_str!("examples/fib.stdout"));
}

#[test]
fn closures_output_matches_golden() {
    assert_example(
        "examples/closures.tether",
        include_str!("examples/closures.stdout"),
    );
}

#[test]
fn tethercstp_browser_renders_react() {
    assert_example(
        "examples/tethercstp_browser.tether",
        include_str!("examples/tethercstp_browser.stdout"),
    );
}

#[test]
fn use_after_move_reports_error() {
    let output = run_example("examples/use_after_move.tether");
    assert!(!output.status.success(), "use_after_move should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("use of moved value `xs`"),
        "unexpected stderr:\n{stderr}"
    );
}
