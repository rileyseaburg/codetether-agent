use super::{TaskKind, words};

pub(super) fn classify(tokens: &[String], label: &[String]) -> TaskKind {
    if has_directive(tokens, words::MUTATING) || has_label(label, words::MUTATING) {
        TaskKind::Mutating
    } else if is_verification(tokens) || has_label(label, words::VERIFYING) {
        TaskKind::Verification
    } else if has_directive(tokens, words::READING) || has_label(label, words::READING) {
        TaskKind::ReadOnly
    } else {
        TaskKind::Mutating
    }
}

fn has_label(tokens: &[String], actions: &[&str]) -> bool {
    tokens.iter().any(|token| actions.contains(&token.as_str()))
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
