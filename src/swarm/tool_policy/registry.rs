use crate::tool::{ToolRegistry, readonly::is_read_only};

pub fn restrict_registry(registry: &mut ToolRegistry, read_only: bool) {
    if !read_only {
        registry.unregister("question");
        return;
    }
    let ids: Vec<String> = registry.list().into_iter().map(str::to_string).collect();
    for id in ids {
        if !is_read_only(&id) {
            registry.unregister(&id);
        }
    }
}
