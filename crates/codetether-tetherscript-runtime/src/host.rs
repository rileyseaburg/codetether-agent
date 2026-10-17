//! Execute hooks in a capability host with the selected grants.
use crate::Host;
use anyhow::Result;
use serde_json::Value;
use tetherscript::plugin::{PluginHost, TetherScriptAuthority};

use codetether_tetherscript_values::{json_to_tetherscript, tetherscript_to_json};

use super::browser::BrowserGrant;
use super::outcome::{self, TetherScriptOutcome};
use crate::ComputerGrant;

/// Run via PluginHost with optional host capabilities granted.
pub fn run<H: Host>(
    source_name: String,
    source: String,
    hook: String,
    args: Vec<Value>,
    browser: BrowserGrant,
    computer: ComputerGrant,
    progress_id: Option<String>,
) -> Result<TetherScriptOutcome> {
    let source = codetether_tetherscript_process::inject(source);
    let mut plugin =
        host::<H>(browser, computer, progress_id).load_source(&source_name, &source)?;
    let ts_args: Vec<_> = args.into_iter().map(json_to_tetherscript).collect();
    let call = plugin.call(&hook, &ts_args)?;
    let tether_val = call.value.clone();
    Ok(TetherScriptOutcome {
        output: outcome::output(call.stdout, &tether_val),
        success: outcome::is_success(&tether_val),
        value: tetherscript_to_json(&tether_val),
    })
}

fn host<H: Host>(
    browser: BrowserGrant,
    computer: ComputerGrant,
    progress_id: Option<String>,
) -> PluginHost {
    let mut host = PluginHost::new();
    host.grant("tetherscript", TetherScriptAuthority::new());
    host.grant(
        "codetether_process",
        codetether_tetherscript_process::ProcessAuthority::new(progress_id),
    );
    super::host_grants::grant_browser(&mut host, browser);
    super::host_grants::grant_computer::<H>(&mut host, computer);
    host
}
