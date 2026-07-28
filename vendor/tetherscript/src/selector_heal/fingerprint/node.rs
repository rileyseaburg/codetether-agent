//! Minimal DOM node used by the selector healer.

use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DomNode {
    pub tag: String,
    pub text: String,
    pub attrs: BTreeMap<String, String>,
    pub children: Vec<DomNode>,
}

impl DomNode {
    pub fn attr(&self, k: &str) -> Option<&str> {
        self.attrs.get(k).map(String::as_str)
    }

    pub fn name(&self) -> String {
        self.attr("aria-label")
            .or_else(|| self.attr("title"))
            .map(str::to_string)
            .unwrap_or_else(|| self.text.trim().to_string())
    }

    pub fn get<'a>(&'a self, path: &[usize]) -> Option<&'a DomNode> {
        let mut n = self;
        for &i in path {
            n = n.children.get(i)?;
        }
        Some(n)
    }

    pub fn walk(&self, out: &mut Vec<(Vec<usize>, DomNode)>, path: Vec<usize>) {
        out.push((path.clone(), self.clone()));
        for (i, c) in self.children.iter().enumerate() {
            let mut p = path.clone();
            p.push(i);
            c.walk(out, p);
        }
    }
}
