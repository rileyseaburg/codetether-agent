//! Explicit full-access scope for non-policy TetherScript fixtures.

use crate::approval::{test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::Tool;
use crate::tool::tetherscript::TetherScriptPluginTool;
use serde_json::Value;

pub(super) async fn execute(tool: &TetherScriptPluginTool, args: Value) -> crate::tool::ToolResult {
    let _lock = lock_env();
    let _env = ScopedEnv::access(AccessMode::Full);
    let _network = crate::tool::network_access::test_env::Network::set("1");
    tool.execute(args)
        .await
        .expect("TetherScript tool execution")
}
