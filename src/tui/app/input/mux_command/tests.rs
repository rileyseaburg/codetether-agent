use std::path::{Path, PathBuf};

use super::action::{Action, parse};

#[test]
fn parses_ls() {
    assert_eq!(parse("ls", Path::new("/work")).unwrap(), Action::List);
}

#[test]
fn defaults_new_session_to_tui_workspace() {
    assert_eq!(
        parse("new backend", Path::new("/work")).unwrap(),
        Action::New {
            name: "backend".into(),
            workspace: PathBuf::from("/work")
        }
    );
}

#[test]
fn preserves_spaces_in_window_workspace() {
    assert_eq!(
        parse("window backend sibling project", Path::new("/work")).unwrap(),
        Action::Window {
            name: "backend".into(),
            workspace: PathBuf::from("/work/sibling project"),
        }
    );
}
