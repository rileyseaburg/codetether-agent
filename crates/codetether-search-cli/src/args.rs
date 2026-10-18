//! Search command argument schema.
use clap::Parser;

/// Search query, routing preferences and output format.
///
/// # Examples
/// ```
/// use clap::Parser;
/// let args = codetether_search_cli::SearchArgs::parse_from(["search", "find main", "--json"]);
/// assert!(args.json);
/// ```
#[derive(Parser, Debug)]
#[command(
    about = "LLM-routed search across grep/glob/web/memory/RLM backends",
    long_about = "Uses the configured LLM router (default zai/glm-5.1) to pick the best search backend for your query, runs it, and returns normalized results.",
    after_long_help = "Examples:\n  codetether search \"where is fn main\"\n  codetether search --json \"how to center a div\"\n  codetether search --top-n 3 --router-model zai/glm-5.1 \"latest rust async trait docs\""
)]
pub struct SearchArgs {
    /// Natural-language search query
    pub query: String,

    /// Maximum number of backends to run
    #[arg(long, default_value = "1")]
    pub top_n: usize,

    /// Override router model (default zai/glm-5.1 or CODETETHER_SEARCH_ROUTER_MODEL)
    #[arg(long)]
    pub router_model: Option<String>,

    /// Emit structured JSON instead of human output
    #[arg(long)]
    pub json: bool,
}
