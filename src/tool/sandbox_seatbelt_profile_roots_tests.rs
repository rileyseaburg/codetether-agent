use super::super::build;
use crate::tool::sandbox::SandboxPolicy;
use std::path::Path;

fn policy() -> SandboxPolicy {
    SandboxPolicy {
        allowed_paths: vec!["/workspace".into()],
        allow_exec: true,
        ..SandboxPolicy::default()
    }
}

#[test]
fn profile_grants_the_sandbox_home_even_when_temp_dir_differs() {
    let text = build(&policy(), Path::new("/workspace"), Path::new("/var/folders/xy"));
    assert!(
        text.contains("(allow file-write* (subpath \"/tmp\"))"),
        "sandbox HOME=/tmp must stay writable: {text}"
    );
    assert!(text.contains("(allow file-write* (subpath \"/var/folders/xy\"))"));
}

#[test]
fn profile_allows_writing_standard_streams() {
    let text = build(&policy(), Path::new("/workspace"), Path::new("/tmp"));
    assert!(text.contains("(literal \"/dev/null\")"));
    assert!(text.contains("(literal \"/dev/stderr\")"));
}