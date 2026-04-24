use super::{BrowserCtlArgs, BrowserCtlCommand};
use serde_json::{Value, json};

pub fn needs_attach(command: &BrowserCtlCommand) -> bool {
    !matches!(
        command,
        BrowserCtlCommand::Start { .. } | BrowserCtlCommand::Stop
    )
}

pub fn attach(args: &BrowserCtlArgs) -> Value {
    json!({
        "action": "start",
        "headless": true,
        "ws_url": args.ws_url,
    })
}

pub fn command(args: &BrowserCtlArgs) -> Value {
    match &args.command {
        BrowserCtlCommand::Start {
            headless,
            executable_path,
            user_data_dir,
        } => json!({
            "action": "start",
            "headless": headless,
            "executable_path": executable_path,
            "user_data_dir": user_data_dir,
            "ws_url": args.ws_url,
        }),
        BrowserCtlCommand::Stop => json!({"action": "stop"}),
        BrowserCtlCommand::Health => json!({"action": "health"}),
        BrowserCtlCommand::List => json!({"action": "tabs"}),
        BrowserCtlCommand::Open { url } => json!({"action": "goto", "url": url}),
        BrowserCtlCommand::Snapshot => json!({"action": "snapshot"}),
        BrowserCtlCommand::Eval {
            expression,
            timeout_ms,
        } => json!({
            "action": "eval",
            "expression": expression,
            "timeout_ms": timeout_ms,
        }),
        BrowserCtlCommand::Screenshot { path, full_page } => json!({
            "action": "screenshot",
            "path": path,
            "full_page": full_page,
        }),
    }
}
