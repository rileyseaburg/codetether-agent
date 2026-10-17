//! Wire question and choice types, plus the validated request container.
use serde::{Deserialize, Serialize};

/// Ordered answers: one list of labels/custom text per question.
pub type Answers = Vec<Vec<String>>;

/// A labeled choice with explanatory text.
///
/// # Examples
/// ```
/// use codetether_question::Choice;
/// let c = Choice { label: "Fast".into(), description: "Low latency".into() };
/// assert_eq!(c.label, "Fast");
/// ```
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Choice {
    /// Text returned when selected.
    pub label: String,
    /// Explanation displayed beside the choice.
    #[serde(default)]
    pub description: String,
}

/// One question; a custom answer is always available in addition to choices.
///
/// # Examples
/// ```
/// use codetether_question::Question;
/// let q: Question = serde_json::from_str(r#"{"question":"Why?"}"#).unwrap();
/// assert!(!q.multiple);
/// ```
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Question {
    /// Full prompt shown to the user.
    pub question: String,
    /// Short title label.
    #[serde(default)]
    pub header: String,
    /// Available choices, in recommendation order.
    #[serde(default)]
    pub options: Vec<Choice>,
    /// Whether multiple labels may be selected.
    #[serde(default)]
    pub multiple: bool,
}

/// Validated nonempty batch. Construct with [`Request::parse`].
///
/// # Examples
/// ```
/// use codetether_question::Request;
/// let r = Request::parse(serde_json::json!({"question":"Continue?"})).unwrap();
/// assert_eq!(r.questions().len(), 1);
/// ```
#[derive(Clone, Debug, Serialize)]
pub struct Request {
    pub(super) questions: Vec<Question>,
}
