//! Commit one answer, deduplicate custom text, and advance the batch.
use super::Draft;
use crate::Answers;
impl Draft {
    pub(super) fn submit(&mut self, count: usize, cursor: usize) -> Option<Answers> {
        let q = self.question();
        let mut answer: Vec<String> = if q.multiple {
            q.options
                .iter()
                .zip(self.selected())
                .filter(|(_, on)| **on)
                .map(|(o, _)| o.label.clone())
                .collect()
        } else if cursor < count {
            vec![q.options[cursor].label.clone()]
        } else {
            vec![]
        };
        if (q.multiple || cursor == count)
            && !self.custom().trim().is_empty()
            && !answer.iter().any(|a| a == self.custom().trim())
        {
            answer.push(self.custom().trim().to_owned());
        }
        if answer.is_empty() {
            return None;
        }
        self.answers[self.index] = answer;
        if self.index + 1 == self.total() {
            return Some(self.answers.clone());
        }
        self.index += 1;
        None
    }
}
