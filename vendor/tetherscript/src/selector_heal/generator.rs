//! Robust selector generation.

mod helpers;
mod matching;
mod structural;

use super::{DomNode, ElementPath};
use helpers::{esc, fragile_id, unique_class};

#[derive(Clone, Debug, Default)]
pub struct SelectorGenerator;

impl SelectorGenerator {
    pub fn generate(&self, root: &DomNode, path: &[usize]) -> Vec<String> {
        let Some(n) = root.get(path) else {
            return vec![];
        };
        let mut v = Vec::new();
        for a in ["data-testid", "data-test", "aria-label", "name"] {
            if let Some(x) = n.attr(a) {
                v.push(format!("[{a}=\"{}\"]", esc(x)));
            }
        }
        if let Some(role) = n.attr("role") {
            let name = n.name();
            if !name.is_empty() {
                v.push(format!(
                    "[role=\"{}\"][aria-label=\"{}\"]",
                    esc(role),
                    esc(&name)
                ));
            }
            v.push(format!("[role=\"{}\"]", esc(role)));
        }
        if let Some(id) = n.attr("id").filter(|x| !fragile_id(x)) {
            v.push(format!("#{}", id));
        }
        if let Some(c) = unique_class(n) {
            v.push(format!("{}.{}", n.tag, c));
        }
        if !n.text.trim().is_empty() {
            v.push(format!("{}:text(\"{}\")", n.tag, esc(n.text.trim())));
        }
        v.push(structural::selector(root, path));
        v.into_iter().filter(|s| !s.is_empty()).collect()
    }

    pub fn shortest_unique(&self, root: &DomNode, path: &[usize]) -> Option<String> {
        let mut xs = self.generate(root, path);
        xs.sort_by_key(String::len);
        xs.into_iter().find(|s| self.find(root, s).len() == 1)
    }

    pub fn find(&self, root: &DomNode, selector: &str) -> Vec<ElementPath> {
        let mut all = Vec::new();
        root.walk(&mut all, vec![]);
        all.into_iter()
            .filter_map(|(p, n)| matching::matches_sel(&n, selector).then_some(p))
            .collect()
    }
}
