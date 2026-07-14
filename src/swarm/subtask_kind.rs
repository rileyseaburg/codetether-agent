//! Intent classification for swarm worktree and tool policy decisions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TaskKind {
    ReadOnly,
    Verification,
    Mutating,
}

#[path = "subtask_kind_words.rs"]
mod words;

pub(super) fn classify(name: &str, instruction: &str, specialty: Option<&str>) -> TaskKind {
    let intent = format!("{instruction} {name} {}", specialty.unwrap_or_default());
    let tokens = tokens(&intent);
    if has_directive(&tokens, words::MUTATING) {
        TaskKind::Mutating
    } else if is_verification(&tokens) {
        TaskKind::Verification
    } else if has_directive(&tokens, words::READING) {
        TaskKind::ReadOnly
    } else {
        TaskKind::Mutating
    }
}

fn tokens(text: &str) -> Vec<String> {
    text.to_ascii_lowercase()
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_owned)
        .collect()
}

fn has_directive(tokens: &[String], actions: &[&str]) -> bool {
    tokens.iter().enumerate().any(|(index, token)| {
        actions.contains(&token.as_str())
            && (index == 0 || words::CONNECTORS.contains(&tokens[index - 1].as_str()))
    })
}

fn is_verification(tokens: &[String]) -> bool {
    has_directive(tokens, words::VERIFYING)
        || (tokens
            .first()
            .is_some_and(|token| words::RUN.contains(&token.as_str()))
            && tokens
                .iter()
                .any(|token| words::VERIFYING.contains(&token.as_str())))
}

#[cfg(test)]
#[path = "subtask_kind_tests.rs"]
mod tests;
