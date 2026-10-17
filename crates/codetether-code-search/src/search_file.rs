//! search file for workspace code search.
use super::*;
impl CodeSearchTool {
    pub(crate) fn search_file(
        &self,
        path: &std::path::Path,
        pattern: &regex::Regex,
        context: usize,
    ) -> Result<Vec<Match>> {
        let content = std::fs::read_to_string(path)?;
        let lines: Vec<&str> = content.lines().collect();
        let mut matches = Vec::new();

        for (idx, line) in lines.iter().enumerate() {
            if pattern.is_match(line) {
                let start = idx.saturating_sub(context);
                let end = (idx + context + 1).min(lines.len());
                let context_lines: Vec<String> = lines[start..end]
                    .iter()
                    .enumerate()
                    .map(|(i, l)| {
                        let line_num = start + i + 1;
                        let marker = if start + i == idx { ">" } else { " " };
                        format!("{} {:4}: {}", marker, line_num, l)
                    })
                    .collect();

                matches.push(Match {
                    path: path
                        .strip_prefix(&self.root)
                        .unwrap_or(path)
                        .to_string_lossy()
                        .to_string(),
                    line: idx + 1,
                    matched_line: line.to_string(),
                    context: context_lines.join("\n"),
                });
            }
        }
        Ok(matches)
    }
}
