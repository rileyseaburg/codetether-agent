//! Action grammar for unmistakable prior-context opt-ins.

const DIRECT: &[&str] = &[
    "use", "call", "recall", "access", "search", "browse", "load", "restore", "allow", "enable",
    "permit", "turn",
];
const AMBIGUOUS: &[&str] = &[
    "check",
    "inspect",
    "open",
    "read",
    "look at",
    "look through",
];

pub(super) fn matches(text: &str) -> bool {
    let text = text.trim_start_matches("now ");
    starts(DIRECT, text)
        || (starts(AMBIGUOUS, text)
            && ["this turn", "this request", "this time", "just once"]
                .iter()
                .any(|term| text.contains(term)))
}

fn starts(actions: &[&str], text: &str) -> bool {
    actions.iter().any(|action| {
        text.strip_prefix(action)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with(' '))
    })
}
