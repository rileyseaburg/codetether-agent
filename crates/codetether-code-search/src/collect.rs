//! collect for workspace code search.
use super::*;
impl CodeSearchTool {
    pub(crate) fn collect(
        &self,
        search_root: &PathBuf,
        file_glob: Option<glob::Pattern>,
        regex: &regex::Regex,
        p: &Params,
        all_matches: &mut Vec<Match>,
    ) {
        for entry in WalkDir::new(search_root)
            .into_iter()
            .filter_entry(|entry| !self.should_skip(entry.path()))
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() || self.should_skip(path) || !self.is_text_file(path) {
                continue;
            }

            if let Some(ref glob) = file_glob
                && !glob.matches_path(path)
            {
                continue;
            }

            if let Ok(matches) =
                self.search_file(path, &regex, p.context_lines.min(MAX_CONTEXT_LINES))
            {
                all_matches.extend(matches);
                if all_matches.len() >= MAX_RESULTS {
                    break;
                }
            }
        }
    }
}
