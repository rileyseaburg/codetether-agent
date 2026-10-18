use std::io::Write;

#[test]
fn open_preserves_existing_diagnostics() {
    let directory = tempfile::tempdir().expect("temporary directory should exist");
    std::fs::write(directory.path().join("tui.log"), "earlier\n").expect("seed log should write");

    let mut file = super::open(directory.path()).expect("log should open");
    writeln!(file, "later").expect("new diagnostic should append");
    drop(file);

    let log = std::fs::read_to_string(directory.path().join("tui.log"))
        .expect("combined log should be readable");
    assert_eq!(log, "earlier\nlater\n");
}
