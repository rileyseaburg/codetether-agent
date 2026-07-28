use crate::browser::Node;
use crate::browser_agent::diff::types::DomDiffEntry;

pub(crate) fn slot(
    base: &str,
    index: usize,
    before: Option<&Node>,
    after: Option<&Node>,
    out: &mut Vec<DomDiffEntry>,
) {
    match (before, after) {
        (Some(left), Some(right)) => nodes(base, index, left, right, out),
        (Some(left), None) => out.push(super::entry::removed(child(base, index, left), left)),
        (None, Some(right)) => out.push(super::entry::inserted(child(base, index, right), right)),
        (None, None) => {}
    }
}

fn nodes(base: &str, index: usize, before: &Node, after: &Node, out: &mut Vec<DomDiffEntry>) {
    match (before, after) {
        (Node::Text(left), Node::Text(right)) if left != right => {
            out.push(super::entry::text(child(base, index, before), left, right));
        }
        (Node::Text(_), Node::Text(_)) => {}
        (Node::Element(left), Node::Element(right)) if left.tag == right.tag => {
            element(base, index, before, left, right, out);
        }
        _ => replace(base, index, before, after, out),
    }
}

fn element(
    base: &str,
    index: usize,
    node: &Node,
    before: &crate::browser::Element,
    after: &crate::browser::Element,
    out: &mut Vec<DomDiffEntry>,
) {
    let path = child(base, index, node);
    if before.attrs != after.attrs {
        out.push(super::entry::attrs(path.clone(), before, after));
    }
    super::walk::diff_children(&path, &before.children, &after.children, out);
}

fn replace(base: &str, index: usize, before: &Node, after: &Node, out: &mut Vec<DomDiffEntry>) {
    out.push(super::entry::removed(child(base, index, before), before));
    out.push(super::entry::inserted(child(base, index, after), after));
}

fn child(base: &str, index: usize, node: &Node) -> String {
    super::path::child(base, index, node)
}
