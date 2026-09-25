use super::command::SMOKE_ARGS;
use super::command::parse_version;

#[test]
fn parses_version_from_bwrap_output() {
    assert_eq!(
        parse_version(b"bubblewrap 0.9.0\n").as_deref(),
        Some("0.9.0")
    );
}

#[test]
fn smoke_probe_does_not_require_network_namespace() {
    assert!(!SMOKE_ARGS.contains(&"--unshare-net"));
}

#[test]
fn smoke_probe_includes_dynamic_loader_roots() {
    for path in ["/lib", "/lib64"] {
        assert!(
            SMOKE_ARGS.windows(3).any(|args| args == ["--ro-bind-try", path, path]),
            "missing read-only loader root {path}"
        );
    }
}