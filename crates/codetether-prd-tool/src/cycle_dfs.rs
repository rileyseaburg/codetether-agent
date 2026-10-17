//! cycle dfs for PRD generation and validation.
use super::*;
pub(super) fn dfs<'a>(
    id: &'a str,
    story_map: &std::collections::HashMap<&str, &'a UserStory>,
    visiting: &mut HashSet<&'a str>,
    visited: &mut HashSet<&'a str>,
    path: &mut Vec<&'a str>,
) -> Option<Vec<String>> {
    if visiting.contains(id) {
        let start_idx = path.iter().position(|&x| x == id).unwrap_or(0);
        let mut cycle: Vec<String> = path[start_idx..].iter().map(|s| s.to_string()).collect();
        cycle.push(id.to_string());
        return Some(cycle);
    }

    if visited.contains(id) {
        return None;
    }

    visiting.insert(id);
    path.push(id);

    if let Some(story) = story_map.get(id) {
        for dep in &story.depends_on {
            if let Some(cycle) = dfs(dep.as_str(), story_map, visiting, visited, path) {
                return Some(cycle);
            }
        }
    }

    path.pop();
    visiting.remove(id);
    visited.insert(id);
    None
}
