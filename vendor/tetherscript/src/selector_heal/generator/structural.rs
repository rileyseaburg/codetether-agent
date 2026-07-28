//! Structural selector generation.

use super::super::DomNode;

pub fn selector(root: &DomNode, path: &[usize]) -> String {
    let mut cur = vec![];
    let mut parts = vec![];
    for &i in path {
        let Some(n) = root.get(&cur) else { break };
        let tag = n.children.get(i).map(|c| c.tag.as_str()).unwrap_or("*");
        parts.push(format!("{}:nth-child({})", tag, i + 1));
        cur.push(i);
    }
    parts.join(" > ")
}
