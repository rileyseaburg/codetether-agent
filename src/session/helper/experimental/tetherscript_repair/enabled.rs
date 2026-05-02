//! Tetherscript-powered repair implementation.
//!
//! Loads `deepseek_repair.tether` and calls `repair_msg` on each
//! assistant message with tool_calls but no Thinking block.

use super::super::ExperimentalStats;
use crate::provider::{ContentPart, Message, Role};
use crate::tool::tetherscript::convert::{json_to_tetherscript, tetherscript_to_json};
use tetherscript::plugin::{PluginHost, TetherScriptAuthority};

const REPAIR_SOURCE: &str =
    include_str!("../../../../../examples/tetherscript/deepseek_repair.tether");

pub fn repair_reasoning(messages: &mut Vec<Message>) -> ExperimentalStats {
    let stats = ExperimentalStats::default();
    let mut host = PluginHost::new();
    host.grant("tetherscript", TetherScriptAuthority::new());
    let mut plugin = match host.load_source("deepseek_repair.tether", REPAIR_SOURCE) {
        Ok(p) => p,
        Err(e) => {
            tracing::warn!("tetherscript repair load failed: {e}");
            return stats;
        }
    };

    for msg in messages.iter_mut() {
        if msg.role != Role::Assistant {
            continue;
        }
        let has_calls = msg
            .content
            .iter()
            .any(|p| matches!(p, ContentPart::ToolCall { .. }));
        if !has_calls {
            continue;
        }
        let ds = super::convert::build_ds_msg(msg);
        match plugin.call("repair_msg", &[json_to_tetherscript(ds)]) {
            Ok(call) => {
                let fixed = tetherscript_to_json(&call.value);
                super::convert::apply_ds_repair(msg, &fixed);
            }
            Err(e) => tracing::warn!("tetherscript repair_msg failed: {e}"),
        }
    }
    stats
}
