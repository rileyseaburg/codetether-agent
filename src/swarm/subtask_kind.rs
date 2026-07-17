//! Intent classification for swarm worktree and tool policy decisions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TaskKind {
    ReadOnly,
    Verification,
    Mutating,
}

#[path = "subtask_kind_decision.rs"]
mod decision;
#[path = "subtask_kind_words.rs"]
mod words;

pub(super) fn classify(name: &str, instruction: &str, specialty: Option<&str>) -> TaskKind {
    let intent = format!("{instruction} {name} {}", specialty.unwrap_or_default());
    let label = tokens(name);
    let tokens = tokens(&intent);
    decision::classify(&tokens, &label)
}

fn tokens(text: &str) -> Vec<String> {
    text.to_ascii_lowercase()
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
#[path = "subtask_kind_tests.rs"]
mod tests;
