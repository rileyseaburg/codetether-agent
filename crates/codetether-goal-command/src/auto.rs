//! `/goal auto [on|off]`: toggle adopting chat prompts as verified goals.
use crate::Host;

use anyhow::{Result, anyhow};

pub(super) fn run<H: Host>(arg: &str) -> Result<String> {
    match arg {
        "" | "status" => {}
        "on" => H::set_auto_goal(true),
        "off" => H::set_auto_goal(false),
        other => return Err(anyhow!("usage: /goal auto [on|off]; got `{other}`")),
    }
    Ok(if H::auto_goal_enabled() {
        "Auto-goal on: each new prompt becomes the session goal; the verifier model must accept completion".into()
    } else {
        "Auto-goal off: prompts are ordinary turns; use /goal set for verified goals".into()
    })
}
