//! Editable state for a validated batch and its initial selection.
use crate::{Answers, Request};
/// Editable batch; selections persist when moving between questions.
///
/// # Examples
/// ```
/// use codetether_question::{Draft, Request};
/// let request = Request::parse(serde_json::json!({"question":"Why?"})).unwrap();
/// let draft = Draft::new(request);
/// assert_eq!(draft.total(), 1);
/// ```
#[derive(Clone, Debug)]
pub struct Draft {
    pub(super) request: Request,
    pub(super) index: usize,
    pub(super) cursors: Vec<usize>,
    pub(super) selected: Vec<Vec<bool>>,
    pub(super) custom: Vec<String>,
    pub(super) answers: Answers,
}
impl Draft {
    /// Start at the first choice of a validated request.
    pub fn new(request: Request) -> Self {
        let count = request.questions().len();
        let selected = request
            .questions()
            .iter()
            .map(|q| vec![false; q.options.len()])
            .collect();
        Self {
            request,
            index: 0,
            cursors: vec![0; count],
            selected,
            custom: vec![String::new(); count],
            answers: vec![vec![]; count],
        }
    }
}
