//! execute for workspace code search.
use super::*;
impl CodeSearchTool {
    pub(crate) async fn run(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        let regex = regex::RegexBuilder::new(&p.pattern)
            .case_insensitive(!p.case_sensitive)
            .build()
            .context("Invalid regex pattern")?;

        let search_root = match &p.path {
            Some(subpath) => self.root.join(subpath),
            None => self.root.clone(),
        };

        let file_glob = p
            .file_pattern
            .as_ref()
            .and_then(|pat| glob::Pattern::new(pat).ok());

        let mut all_matches = Vec::new();

        self.collect(&search_root, file_glob, &regex, &p, &mut all_matches);
        render::render(&all_matches, &p)
    }
}
