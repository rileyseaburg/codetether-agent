//! `/goal auto [on|off]`: toggle adopting chat prompts as verified goals.

use crate::session::tasks::runtime::{auto_goal_enabled, set_auto_goal};
use anyhow::{Result, anyhow};

pub(super) fn run(arg: &str) -> Result<String> {
    match arg {
        "" | "status" => {}
        "on" => set_auto_goal(true),
        "off" => set_auto_goal(false),
        other => return Err(anyhow!("usage: /goal auto [on|off]; got `{other}`")),
    }
    Ok(if auto_goal_enabled() {
        "Auto-goal on: each new prompt becomes the session goal; the verifier model must accept completion".into()
    } else {
        "Auto-goal off: prompts are ordinary turns; use /goal set for verified goals".into()
    })
}
