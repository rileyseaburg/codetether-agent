//! Parsing for compact positional `/mux` commands.

use std::path::Path;

use super::action::Action;
use super::action_helpers::{id_action, split, take_no_worktree, workspace_action};

pub(super) fn parse(arguments: &str, cwd: &Path) -> Result<Action, String> {
    let (verb, rest) = split(arguments);
    match verb {
        "" | "help" => Ok(Action::Help),
        "ls" | "list" if rest.is_empty() => Ok(Action::List),
        "new" => new(rest, cwd),
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

fn new(rest: &str, cwd: &Path) -> Result<Action, String> {
    let (rest, no_worktree) = take_no_worktree(rest);
    workspace_action(&rest, cwd, |name, workspace| Action::New {
        name,
        workspace,
        no_worktree,
    })
}
