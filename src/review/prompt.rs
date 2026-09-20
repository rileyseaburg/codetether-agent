//! Everything the reviewer is told about the change under review.

mod system;

pub use system::SYSTEM;

/// Inputs gathered from the approval request and the session.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReviewSubject {
    pub tool: String,
    pub action: String,
    pub resource: String,
    pub justification: Option<String>,
    pub preview: Option<String>,
    /// Rendered goal governance block, when the session has a goal.
    pub goal: Option<String>,
}

/// Build the user turn for the reviewer.
pub fn user(subject: &ReviewSubject) -> String {
    let mut text = String::new();
    match &subject.goal {
        Some(goal) => text.push_str(goal),
        None => text.push_str("## Goal Governance\nNo goal is set for this session."),
    }
    text.push_str(&format!(
        "\n\n## Proposed change\ntool: {}\naction: {}\nresource: {}\n",
        subject.tool, subject.action, subject.resource
    ));
    if let Some(reason) = non_blank(subject.justification.as_deref()) {
        text.push_str(&format!("author's justification: {reason}\n"));
    }
    match non_blank(subject.preview.as_deref()) {
        Some(preview) => text.push_str(&format!("\n```diff\n{preview}\n```\n")),
        None => text.push_str("\n(no preview was supplied; open the resource to inspect it)\n"),
    }
    text.push_str("\nReview it and finish with the JSON verdict.");
    text
}

fn non_blank(value: Option<&str>) -> Option<&str> {
    value.filter(|text| !text.trim().is_empty())
}
