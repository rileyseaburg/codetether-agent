//! Bind application factories to shared scalar tool construction.
use super::*;
use codetether_tool_construction::{NativeTool, application::Factories};
struct Constructors;
impl Factories for Constructors {
    fn files(&self, kind: NativeTool) -> Option<Arc<dyn Tool>> {
        files::create(kind)
    }
    fn context(&self, kind: NativeTool) -> Option<Arc<dyn Tool>> {
        context::create(kind)
    }
    fn execution(&self, kind: NativeTool) -> Option<Arc<dyn Tool>> {
        execution::create(kind)
    }
    fn integrations(&self, kind: NativeTool) -> Option<Arc<dyn Tool>> {
        integrations::create(kind)
    }
    fn goals(&self, kind: ToolKind) -> Option<Arc<dyn Tool>> {
        goal::construct(kind)
    }
    fn collaboration(&self, kind: ToolKind) -> Option<Arc<dyn Tool>> {
        collaboration::construct(kind)
    }
}
pub(super) fn create(kind: ToolKind) -> Arc<dyn Tool> {
    codetether_tool_construction::application::create(kind, &Constructors)
}
