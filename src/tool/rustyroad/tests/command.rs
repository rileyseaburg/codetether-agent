//! Child-only project/environment construction without process-global mutation.

use super::super::command::command;
use std::{ffi::OsStr, path::Path};

#[test]
fn rustyroad_child_environment_and_cwd_are_explicit() {
    let before = (
        std::env::current_dir().unwrap(),
        std::env::var_os("ENVIRONMENT"),
    );
    let command = command(Path::new("/explicit/project"), "test");
    let command = command.as_std();
    assert_eq!(command.get_program(), "rustyroad-mcp");
    assert_eq!(
        command.get_current_dir(),
        Some(Path::new("/explicit/project"))
    );
    for (key, expected) in [
        ("ENVIRONMENT", Some("test")),
        ("ENV", None),
        ("RUSTYROAD_PROJECT_DIR", Some("/explicit/project")),
    ] {
        let value = command.get_envs().find(|(name, _)| *name == key).unwrap().1;
        assert_eq!(value, expected.map(OsStr::new));
    }
    assert_eq!(
        before,
        (
            std::env::current_dir().unwrap(),
            std::env::var_os("ENVIRONMENT")
        )
    );
}
