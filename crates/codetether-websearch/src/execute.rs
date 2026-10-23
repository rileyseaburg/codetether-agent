//! execute for web search.
use super::*;
impl WebSearchTool {
    pub(crate) async fn run(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        if p.query.trim().is_empty() {
            return Ok(ToolResult::error("Query cannot be empty"));
        }
        let configured = std::env::var(BACKEND_ENV).ok();
        let order = match backend_order(configured.as_deref()) {
            Ok(order) => order,
            Err(failure) => return Ok(report::config_error(&p.query, &failure)),
        };
        let max = p.max_results.clamp(1, 20);
        let mut attempts = Vec::with_capacity(order.len());
        for backend in order {
            let attempt = self.attempt(backend, &p.query, max).await;
            let done = attempt.outcome.is_ok();
            attempts.push(attempt);
            if done {
                break;
            }
        }
        Ok(report::render(&p.query, &attempts))
    }
}
