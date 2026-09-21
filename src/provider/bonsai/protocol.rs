//! Execute the actual TetherScript prompt contract in a sandboxed plugin host.
use crate::provider::CompletionRequest;
use anyhow::{Context, Result};
pub(super) fn render(request: &CompletionRequest) -> Result<String> {
    #[cfg(feature = "tetherscript")]
    {
        use crate::tool::tetherscript::convert::{json_to_tetherscript, tetherscript_to_json};
        let host = tetherscript::plugin::PluginHost::new();
        let mut plugin = host.load_source(
            "bonsai_chat.tether",
            include_str!("../../../examples/tetherscript/bonsai_chat.tether"),
        )?;
        let argument = serde_json::json!({"messages":request.messages,"tools":request.tools});
        let result = plugin.call("render", &[json_to_tetherscript(argument)])?;
        let value = tetherscript_to_json(&result.value);
        value
            .as_str()
            .map(str::to_owned)
            .with_context(|| format!("Bonsai prompt contract rejected request: {value}"))
    }
    #[cfg(not(feature = "tetherscript"))]
    anyhow::bail!("Bonsai requires the tetherscript feature");
}
