//! tool for OKR tool dispatch.
use super::*;
#[async_trait]
impl Tool for OkrTool {
    fn id(&self) -> &str {
        "okr"
    }

    fn name(&self) -> &str {
        "OKR Manager"
    }

    fn description(&self) -> &str {
        "Manage Objectives and Key Results (OKRs) and their execution runs. \
         Actions: create_okr, get_okr, update_okr, delete_okr, list_okrs, query_okrs, \
         create_run, get_run, update_run, delete_run, list_runs, query_runs, stats."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        execute::execute(params).await
    }
}
