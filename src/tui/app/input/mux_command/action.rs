//! Parsing for compact positional `/mux` commands.

use std::path::{Path, PathBuf};

use super::action_helpers::{id_action, split, workspace_action};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Action {
    Help,
    List,
    New { name: String, workspace: PathBuf },
    Window { name: String, workspace: PathBuf },
    Select { name: String, id: u64 },
    Close { name: String, id: u64 },
    Kill { name: String },
}

pub(super) fn parse(arguments: &str, cwd: &Path) -> Result<Action, String> {
    let (verb, rest) = split(arguments);
    match verb {
        "" | "help" => Ok(Action::Help),
        "ls" | "list" if rest.is_empty() => Ok(Action::List),
        "new" => workspace_action(rest, cwd, |name, workspace| Action::New { name, workspace }),
        "window" => workspace_action(rest, cwd, |name, workspace| Action::Window {
            name,
            workspace,
        }),
        "select" => id_action(rest, |name, id| Action::Select { name, id }),
        "close" => id_action(rest, |name, id| Action::Close { name, id }),
        "kill" if !rest.is_empty() && !rest.contains(char::is_whitespace) => {
            Ok(Action::Kill { name: rest.into() })
        }
        _ => Err("Usage: /mux <ls|new|window|select|close|kill>".into()),
    }
}
