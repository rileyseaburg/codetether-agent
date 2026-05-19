//! Sync call helpers. `LoadedPlugin` is `!Send`, so these run
//! inside `spawn_blocking` in the async trait impl.

use super::runner::TetherScriptProvider;
use anyhow::{Context, Result};
use serde_json::Value;

impl TetherScriptProvider {
    pub(crate) fn call_sync(&self, hook: &str) -> Result<Value> {
        let f = crate::tool::tetherscript::convert::tetherscript_to_json;
        let h = hook.to_string();
        self.make_plugin()?
            .call(hook, &[])
            .with_context(|| h)
            .map(|r| f(&r.value))
    }

    pub(crate) fn call1_sync(&self, hook: &str, arg: Value) -> Result<Value> {
        let f = crate::tool::tetherscript::convert::tetherscript_to_json;
        let g = crate::tool::tetherscript::convert::json_to_tetherscript;
        let h = hook.to_string();
        self.make_plugin()?
            .call(hook, &[g(arg)])
            .with_context(|| h)
            .map(|r| f(&r.value))
    }
}
