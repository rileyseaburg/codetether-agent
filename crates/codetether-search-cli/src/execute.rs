//! `codetether search` subcommand.
//!
//! Thin wrapper over [`codetether_search_router::run_router_search`] that formats the
//! result for humans or emits JSON.

use std::sync::Arc;

use anyhow::Result;

use crate::SearchArgs;
use codetether_memory_tool::AutoEmbedder;
use codetether_search_router::Registry;
use codetether_search_router::{model::DEFAULT_ROUTER_MODEL, run_router_search};

/// Execute the search subcommand with caller-owned providers and memory policy.
///
/// # Arguments
/// * `args` - Query, router override, backend limit and output format.
/// * `providers` - Configured provider registry.
/// # Returns
/// Unit after the selected search results have been printed.
/// # Errors
/// Returns router, backend or output-serialization errors.
/// # Examples
/// ```no_run
/// # async fn example<R: codetether_search_router::Registry, B: codetether_memory_tool::AutoEmbedder>(providers: std::sync::Arc<R>) -> anyhow::Result<()> {
/// let args = codetether_search_cli::SearchArgs { query: "find main".into(), router_model: None, top_n: 1, json: true };
/// codetether_search_cli::execute::<R, B>(args, providers).await
/// # }
/// ```
///
/// Picks up the router model from `--router-model`, falling back to the
/// `CODETETHER_SEARCH_ROUTER_MODEL` env var and finally [`DEFAULT_ROUTER_MODEL`].
pub async fn execute<R: Registry, B: AutoEmbedder>(
    args: SearchArgs,
    providers: Arc<R>,
) -> Result<()> {
    let router_model = args
        .router_model
        .clone()
        .or_else(|| std::env::var("CODETETHER_SEARCH_ROUTER_MODEL").ok())
        .unwrap_or_else(|| DEFAULT_ROUTER_MODEL.to_string());
    let top_n = args.top_n.max(1);
    let result = run_router_search::<R, B>(providers, &router_model, &args.query, top_n).await?;
    if args.json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        crate::render::render_human(&result);
    }
    Ok(())
}
