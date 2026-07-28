use std::{fs, process::Command};

#[test]
fn browser_grant_is_available_in_default_vm() {
    let script = std::env::temp_dir().join(format!(
        "tetherscript-browser-grant-{}.tether",
        std::process::id()
    ));
    fs::write(
        &script,
        "fn main(){let d=browser.describe()? println(d.endpoint) println(len(d.scopes))}",
    )
    .unwrap();

    let out = Command::new(env!("CARGO_BIN_EXE_tetherscript"))
        .args([
            "run",
            "--grant-browser",
            "http://127.0.0.1:1/browser",
            "--browser-scope",
            "all",
            script.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    let _ = fs::remove_file(&script);

    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "http://127.0.0.1:1/browser\n11\n"
    );
}
