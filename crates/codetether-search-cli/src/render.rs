//! Human-readable formatter for [`codetether_search::RouterResult`].
//!
//! Prints one section per backend with a header, output body, and a
//! short metadata summary.

use codetether_search::RouterResult;

/// Print a [`RouterResult`] to stdout in human form.
///
/// # Arguments
/// * `result` - Ordered search backend results and router metadata.
/// # Returns
/// Completes after printing each backend and any non-null metadata.
/// # Examples
/// ```no_run
/// # fn example(result: &codetether_search::RouterResult) {
/// codetether_search_cli::render_human(result);
/// # }
/// ```
pub fn render_human(result: &RouterResult) {
    println!("Query: {}", result.query);
    println!("Router: {}\n", result.router_model);
    if result.runs.is_empty() {
        println!("(no backends selected)");
        return;
    }
    for (idx, run) in result.runs.iter().enumerate() {
        let status = if run.success { "ok" } else { "error" };
        println!(
            "── #{n} {backend} [{status}] ──",
            n = idx + 1,
            backend = run.backend.id(),
        );
        println!("{}", run.output.trim_end());
        if !run.metadata.is_null() {
            println!("meta: {}", run.metadata);
        }
        println!();
    }
}
