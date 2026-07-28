use crate::browser::{Element, Node};
use crate::browser_agent::diff::types::{DomDiffEntry, DomDiffKind};

pub(crate) fn inserted(path: String, node: &Node) -> DomDiffEntry {
    node_entry(DomDiffKind::Inserted, path, None, Some(node))
}

pub(crate) fn removed(path: String, node: &Node) -> DomDiffEntry {
    node_entry(DomDiffKind::Removed, path, Some(node), None)
}

pub(crate) fn text(path: String, before: &str, after: &str) -> DomDiffEntry {
    DomDiffEntry {
        kind: DomDiffKind::TextChanged,
        path,
        before: Some(super::attrs::text(before)),
        after: Some(super::attrs::text(after)),
    }
}

pub(crate) fn attrs(path: String, before: &Element, after: &Element) -> DomDiffEntry {
    DomDiffEntry {
        kind: DomDiffKind::AttributesChanged,
        path,
        before: Some(super::attrs::render(&before.attrs)),
        after: Some(super::attrs::render(&after.attrs)),
    }
}

fn node_entry(
    kind: DomDiffKind,
    path: String,
    before: Option<&Node>,
    after: Option<&Node>,
) -> DomDiffEntry {
    DomDiffEntry {
        kind,
        path,
        before: before.map(super::attrs::node),
        after: after.map(super::attrs::node),
    }
}
