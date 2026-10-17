//! execute for bounded grep searches.
use super::*;
pub(super) async fn execute(args: Value) -> Result<ToolResult> {
    let options = match options::parse(&args) {
        Ok(options) => options,
        Err(result) => return Ok(result),
    };
    let regex = if options.is_regex {
        Regex::new(options.pattern)?
    } else {
        Regex::new(&regex::escape(options.pattern))?
    };
    let include_pattern = options
        .include
        .and_then(|pattern| glob::Pattern::new(pattern).ok());
    let scanned = scan::scan(&options, &regex, include_pattern).await;
    render::render(scanned, &options)
}
