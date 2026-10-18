//! Bind application factories to shared scalar tool construction.
use super::*;
use codetether_tool_construction::{Host, NativeTool};
struct Constructors;
impl Host for Constructors {
    fn create(&self, kind: NativeTool) -> Arc<dyn Tool> {
        files::create(kind)
            .or_else(|| context::create(kind))
            .or_else(|| execution::create(kind))
            .or_else(|| integrations::create(kind))
            .expect("application-bound catalog constructor")
    }
}
pub(super) fn create(kind: ToolKind) -> Arc<dyn Tool> {
    codetether_tool_construction::create(kind, &Constructors)
        .or_else(|| goal::construct(kind))
        .or_else(|| collaboration::construct(kind))
        .expect("native catalog constructor")
}
