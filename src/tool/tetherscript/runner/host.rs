use std::rc::Rc;

use anyhow::Result;
use serde_json::Value;
use tetherscript::browser_cap::BrowserAuthority;
use tetherscript::capability::Authority;
use tetherscript::plugin::{PluginHost, TetherScriptAuthority};

use crate::tool::tetherscript::convert::{json_to_tetherscript, tetherscript_to_json};

use super::browser::BrowserGrant;
use super::outcome::{self, TetherScriptOutcome};

/// Run via PluginHost with optional browser capability granted.
pub fn run(
    source_name: String,
    source: String,
    hook: String,
    args: Vec<Value>,
    browser: BrowserGrant,
) -> Result<TetherScriptOutcome> {
    let mut plugin = host(browser).load_source(&source_name, &source)?;
    let ts_args: Vec<_> = args.into_iter().map(json_to_tetherscript).collect();
    let call = plugin.call(&hook, &ts_args)?;
    let tether_val = call.value.clone();
    Ok(TetherScriptOutcome {
        output: outcome::output(call.stdout, &tether_val),
        success: outcome::is_success(&tether_val),
        value: tetherscript_to_json(&tether_val),
    })
}

fn host(browser: BrowserGrant) -> PluginHost {
    let mut host = PluginHost::new();
    host.grant("tetherscript", TetherScriptAuthority::new());
    if let Some(endpoint) = &browser.endpoint {
        host.grant("browser", browser_authority(endpoint, &browser));
    }
    host
}

fn browser_authority(endpoint: &str, browser: &BrowserGrant) -> Rc<dyn Authority> {
    let scopes = if browser.scopes.is_empty() {
        BrowserAuthority::all_scopes()
    } else {
        browser.scopes.clone()
    };
    BrowserAuthority::new(endpoint, browser.origins.clone(), scopes)
}
