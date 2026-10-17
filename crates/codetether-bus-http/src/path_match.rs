//! Choose the most specific or uniquely matching workspace.
use super::*;

pub(super) fn best_workspace_match<'a>(
    workspace_root: &str,
    workspaces: &'a [WorkspaceSummary],
) -> Option<&'a WorkspaceSummary> {
    let direct = workspaces
        .iter()
        .filter_map(|workspace| {
            let path = workspace.path.as_deref()?;
            if workspace_root == path || workspace_root.starts_with(&format!("{}/", path)) {
                Some((path.len(), workspace))
            } else {
                None
            }
        })
        .max_by_key(|(path_len, _)| *path_len)
        .map(|(_, workspace)| workspace);

    if direct.is_some() {
        return direct;
    }

    let mut scored: Vec<(usize, &WorkspaceSummary)> = workspaces
        .iter()
        .filter_map(|workspace| {
            let path = workspace.path.as_deref()?;
            let score = shared_path_suffix_score(workspace_root, path);
            (score > 0).then_some((score, workspace))
        })
        .collect();

    scored.sort_by(|left, right| right.0.cmp(&left.0));

    match scored.as_slice() {
        [] => None,
        [(score, workspace), ..] => {
            let is_unique_best = scored
                .get(1)
                .map(|(next_score, _)| next_score < score)
                .unwrap_or(true);
            if is_unique_best {
                Some(*workspace)
            } else {
                None
            }
        }
    }
}
