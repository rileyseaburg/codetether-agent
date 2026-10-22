//! The catalog constructs the pure planner without application/runtime hooks.
use crate::{Host, NativeTool};
use codetether_tool_core::Tool;
use codetether_tool_registry::catalog::ToolKind;
use std::sync::Arc;

struct NoRuntime;
impl Host for NoRuntime {
    fn create(&self, _: NativeTool) -> Arc<dyn Tool> {
        panic!("pure Rust planning must not initialize a runtime")
    }
}
#[test]
fn rust_build_plan_is_first_party_and_has_no_execution_authority() {
    let tool = crate::create(ToolKind::RustBuildPlan, &NoRuntime).unwrap();
    assert_eq!(tool.id(), "rust_build_plan");
}
