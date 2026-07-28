//! Computed-style lookup by DOM path.

use std::collections::BTreeMap;

use crate::browser::{computed_styles, Document, Node, StyledNode};

use super::{defaults, ComputedStyle};

pub(crate) fn at_path(document: &Document, css: &str, path: &[usize]) -> Option<ComputedStyle> {
    let roots = computed_styles(document, css);
    let styled = node_at_path(&roots, path)?;
    let Node::Element(element) = &styled.node else {
        return None;
    };
    let mut properties = BTreeMap::new();
    for (name, value) in &styled.styles {
        properties.insert(name.clone(), value.clone());
    }
    defaults::apply(&element.tag, &mut properties);
    Some(ComputedStyle {
        path: path.to_vec(),
        tag: element.tag.clone(),
        properties,
    })
}

fn node_at_path<'a>(nodes: &'a [StyledNode], path: &[usize]) -> Option<&'a StyledNode> {
    let (first, rest) = path.split_first()?;
    let mut current = nodes.get(*first)?;
    for index in rest {
        current = current.children.get(*index)?;
    }
    Some(current)
}
