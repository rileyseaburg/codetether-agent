use anyhow::Result;
use serde_json::Value;
use tetherscript::plugin::{PluginHost, TetherScriptAuthority};

use crate::tool::tetherscript::convert::{json_to_tetherscript, tetherscript_to_json};

use super::browser::BrowserGrant;
use super::computer::ComputerGrant;
use super::outcome::{self, TetherScriptOutcome};

/// Run via PluginHost with optional host capabilities granted.
pub fn run(
    source_name: String,
    source: String,
    hook: String,
    args: Vec<Value>,
    browser: BrowserGrant,
    computer: ComputerGrant,
) -> Result<TetherScriptOutcome> {
    let mut plugin = host(browser, computer).load_source(&source_name, &source)?;
    let ts_args: Vec<_> = args.into_iter().map(json_to_tetherscript).collect();
    let call = plugin.call(&hook, &ts_args)?;
    let tether_val = call.value.clone();
    Ok(TetherScriptOutcome {
        output: outcome::output(call.stdout, &tether_val),
        success: outcome::is_success(&tether_val),
        value: tetherscript_to_json(&tether_val),
    })
}

fn host(browser: BrowserGrant, computer: ComputerGrant) -> PluginHost {
    let mut host = PluginHost::new();
    host.grant("tetherscript", TetherScriptAuthority::new());
    super::host_grants::grant_browser(&mut host, browser);
    super::host_grants::grant_computer(&mut host, computer);
    host
}
