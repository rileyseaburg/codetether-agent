//! `codetether search` subcommand.
//!
//! Thin wrapper over [`crate::search::run_router_search`] that formats the
//! result for humans or emits JSON.

use std::sync::Arc;

use anyhow::Result;

use crate::cli::SearchArgs;
use crate::provider::ProviderRegistry;
use crate::search::{model::DEFAULT_ROUTER_MODEL, run_router_search};

/// Execute the `search` subcommand.
///
/// Picks up the router model from `--router-model`, falling back to the
/// `CODETETHER_SEARCH_ROUTER_MODEL` env var and finally [`DEFAULT_ROUTER_MODEL`].
pub async fn execute(args: SearchArgs) -> Result<()> {
    let providers = Arc::new(ProviderRegistry::from_vault().await?);
    let router_model = args
        .router_model
        .clone()
        .or_else(|| std::env::var("CODETETHER_SEARCH_ROUTER_MODEL").ok())
        .unwrap_or_else(|| DEFAULT_ROUTER_MODEL.to_string());
    let top_n = args.top_n.max(1);
    let result = run_router_search(providers, &router_model, &args.query, top_n).await?;
    if args.json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        super::search_render::render_human(&result);
    }
    Ok(())
}
