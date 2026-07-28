use std::fs;
use std::process::Command;

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_tetherscript"))
        .args(args)
        .output()
        .unwrap_or_else(|err| panic!("failed to run tetherscript {args:?}: {err}"))
}

#[test]
fn render_cli_keeps_stable_agent_display_list() {
    let out = run(&[
        "render",
        "examples/browser.html",
        "examples/browser.css",
        "80",
    ]);

    assert!(out.status.success());
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "viewport 80x5\n  <main> @0,0 40x5\n    <h1> @1,1 38x2\n      \"Hello notebook\" @1,1 14x1\n    <p> @1,3 38x1\n      \"Rendered by TetherScript.\" @1,3 25x1\n"
    );
}

#[test]
fn raster_cli_keeps_stable_agent_image_surface() {
    let ppm = std::env::temp_dir().join(format!(
        "tetherscript-agent-browser-{}.ppm",
        std::process::id()
    ));
    let out = run(&[
        "raster",
        "examples/browser.html",
        ppm.to_str().unwrap(),
        "examples/browser.css",
        "80",
        "8",
        "1",
    ]);

    assert!(out.status.success());
    let bytes = fs::read(&ppm).unwrap();
    let _ = fs::remove_file(&ppm);
    assert!(bytes.starts_with(b"P6\n80 8\n255\n"));
    assert!(bytes.len() > 80 * 8 * 3);
}
