use super::detected;

#[test]
fn detects_redirect_into_temp() {
    assert_eq!(detected("echo hi > /tmp/out.txt").as_deref(), Some("/tmp/out.txt"));
    assert!(detected("cargo test 2>&1 | tee /var/tmp/log").is_some());
}

#[test]
fn detects_write_commands_targeting_temp() {
    assert!(detected("mkdir -p /tmp/work").is_some());
    assert!(detected("cp src/main.rs /tmp/backup.rs").is_some());
    assert!(detected("touch /dev/shm/flag").is_some());
}

#[test]
fn detects_temp_write_in_later_pipeline_segment() {
    assert!(detected("cd repo && mkdir /tmp/staging").is_some());
}

#[test]
fn allows_reading_from_temp() {
    assert!(detected("cat /tmp/existing.log").is_none());
    assert!(detected("ls -la /tmp").is_none());
    assert!(detected("grep needle /tmp/haystack").is_none());
}

#[test]
fn allows_write_commands_targeting_the_workspace() {
    assert!(detected("mkdir -p src/newmod").is_none());
    assert!(detected("cp a.rs b.rs").is_none());
    assert!(detected("echo hi > ./artifacts/out.txt").is_none());
}

#[test]
fn does_not_confuse_workspace_paths_that_start_with_tmp_text() {
    assert!(detected("mkdir -p tmpl/output").is_none());
    assert!(detected("touch tmpfile").is_none());
}