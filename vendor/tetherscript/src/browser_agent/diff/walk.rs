use crate::browser::Node;
use crate::browser_agent::diff::types::DomDiffEntry;

pub(crate) fn diff_children(
    base: &str,
    before: &[Node],
    after: &[Node],
    out: &mut Vec<DomDiffEntry>,
) {
    let len = before.len().max(after.len());
    for index in 0..len {
        super::compare::slot(base, index, before.get(index), after.get(index), out);
    }
}
