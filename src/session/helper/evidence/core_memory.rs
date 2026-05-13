use std::path::Path;

const EMPTY: &str = "Core memory: no high-confidence project beliefs loaded.";

pub(crate) fn render(project_root: &Path) -> String {
    let beliefs = crate::memory::palace::load_project_beliefs(project_root);
    let recall = super::belief_recall::render(&beliefs);
    let context = crate::memory::palace::belief_context(&beliefs);
    if context.trim().is_empty() {
        return format!("{EMPTY}\n{recall}");
    }
    format!("Core memory:\n{}\n{recall}", context.trim())
}

pub(crate) fn storage_hint() -> &'static str {
    "Persist durable decisions with memory.save using scope=current project and tags core-memory,evidence,scope."
}

#[cfg(test)]
mod tests {
    use super::storage_hint;

    #[test]
    fn storage_hint_names_core_memory_tag() {
        assert!(storage_hint().contains("core-memory"));
    }
}
