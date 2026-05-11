use anyhow::Result;

use super::offline_args::OfflineCommand;
use crate::browser::offline::{auth_trace, cookie_diff, explain_cors, record, replay};

pub fn execute(cmd: &OfflineCommand, json: bool) -> Result<()> {
    let output = dispatch(cmd)?;
    println!("{}", super::run::format_output(&output, json));
    Ok(())
}

fn dispatch(cmd: &OfflineCommand) -> Result<String> {
    match cmd {
        OfflineCommand::AuthTrace { url, max_redirects } => auth_trace::run(url, *max_redirects),
        OfflineCommand::CookieDiff { before, after } => cookie_diff::run(before, after),
        OfflineCommand::ExplainCors { url, origin, method } => explain_cors::run(url, origin, method),
        OfflineCommand::Record { url, out } => record::run(url, out),
        OfflineCommand::Replay { capture } => replay::run(capture),
    }
}
