//! Live embedded-library consumer: executes the native `websearch` tool
//! through the public `Tool` trait, exactly as an embedding application does.
//!
//! ```text
//! cargo run -p codetether-websearch --example live_search -- "query"
//! ```
use codetether_tool_core::Tool;
use codetether_websearch::{EVIDENCE_PREFIX, WebSearchTool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let query = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "apartment complexes in Lavon TX official website".into());
    let tool = WebSearchTool::new();
    let result = tool
        .execute(serde_json::json!({"query": query, "max_results": 5}))
        .await?;
    println!("success={}\n{}", result.success, result.output);
    let evidence = result
        .output
        .lines()
        .find_map(|l| l.strip_prefix(EVIDENCE_PREFIX));
    println!("--- evidence parsed: {}", evidence.is_some());
    std::process::exit(if result.success { 0 } else { 1 });
}
