use crate::cognition::beliefs::Belief;

pub(crate) fn render(beliefs: &[Belief]) -> String {
    let keywords = super::belief_keywords::top_keywords(beliefs, 8);
    if keywords.is_empty() {
        return "Belief-guided recall: no active belief keywords.".to_string();
    }
    format!(
        "Belief-guided recall: prioritize memories and session_recall queries related to: {}.",
        keywords.join(", ")
    )
}
