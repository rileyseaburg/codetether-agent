use std::collections::BTreeSet;

pub(super) enum Outcome {
    Merged(BTreeSet<String>),
    Conflicted(Vec<String>),
}
