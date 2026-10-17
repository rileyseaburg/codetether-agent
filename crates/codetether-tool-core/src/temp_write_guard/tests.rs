use super::{denied_reason, denied_result};

#[test]
fn denies_common_temp_roots() {
    for path in [
        "/tmp/scratch.txt",
        "/var/tmp/x",
        "/dev/shm/y",
        "/private/tmp/z",
        "/private/var/folders/ab/cd/T/scratch",
    ] {
        assert!(denied_reason(path).is_some(), "{path} must be denied");
    }
}

#[test]
fn denies_traversal_back_into_temp() {
    assert!(denied_reason("/tmp/../tmp/sneaky.txt").is_some());
    assert!(denied_reason("/var/tmp/./nested/../f").is_some());
}

#[test]
fn allows_workspace_paths_that_merely_mention_tmp() {
    assert!(denied_reason("src/main.rs").is_none());
    assert!(denied_reason("./tmpl/template.rs").is_none());
    assert!(denied_reason("/home/dev/repo/tmp/build.log").is_none());
    assert!(denied_reason("/home/dev/tmpfile.txt").is_none());
}

#[test]
fn denies_windows_temp_locations() {
    assert!(denied_reason("C:\\Users\\dev\\AppData\\Local\\Temp\\x.txt").is_some());
    assert!(denied_reason("C:\\Windows\\Temp\\y.txt").is_some());
}

#[test]
fn refusal_is_structured_and_names_the_path() {
    let blocked = denied_result("write", "/tmp/evidence.json").expect("must be blocked");
    assert!(!blocked.success);
    assert_eq!(
        blocked.metadata.get("error_code").and_then(|v| v.as_str()),
        Some("TEMP_DIR_WRITE_BLOCKED")
    );
    assert!(blocked.output.contains("/tmp/evidence.json"));
}

#[test]
fn clean_paths_produce_no_result() {
    assert!(denied_result("write", "src/lib.rs").is_none());
}
