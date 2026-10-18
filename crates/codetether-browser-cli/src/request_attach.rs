//! Browser attachment selection and initial request parameters.
use crate::{BrowserCtlArgs, BrowserCtlCommand};
use serde_json::{Value, json};

pub(super) fn needs_attach(command: &BrowserCtlCommand) -> bool {
    use BrowserCtlCommand::*;
    !matches!(command, Start { .. } | Stop | Health | Offline { .. })
}

pub(super) fn attach(args: &BrowserCtlArgs) -> Value {
    json!({
        "action": "start",
        "headless": args.headless,
        "ws_url": args.ws_url,
    })
}
