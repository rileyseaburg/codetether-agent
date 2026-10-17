//! cycle for PRD construction and validation.
use super::*;

pub(super) fn detect_cycle(prd: &Prd) -> Option<Vec<String>> {
    let story_map: std::collections::HashMap<&str, &UserStory> = prd
        .user_stories
        .iter()
        .map(|s| (s.id.as_str(), s))
        .collect();

    let mut visited = HashSet::new();
    let mut visiting = HashSet::new();
    let mut path = Vec::new();

    for story in &prd.user_stories {
        if let Some(cycle) = cycle_dfs::dfs(
            story.id.as_str(),
            &story_map,
            &mut visiting,
            &mut visited,
            &mut path,
        ) {
            return Some(cycle);
        }
    }

    None
}
