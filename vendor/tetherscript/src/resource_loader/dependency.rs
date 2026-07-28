//! Dependency graph for CSS imports and JavaScript modules.
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyKind {
    CssImport,
    JsModule,
}

#[derive(Debug, Default)]
pub struct DependencyGraph {
    edges: HashMap<String, Vec<(String, DependencyKind)>>,
    render_blocking: HashSet<String>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_dependency(
        &mut self,
        from: impl Into<String>,
        to: impl Into<String>,
        kind: DependencyKind,
    ) {
        self.edges
            .entry(from.into())
            .or_default()
            .push((to.into(), kind));
    }

    pub fn add_css_import(&mut self, from: &str, to: &str) {
        self.add_dependency(from, to, DependencyKind::CssImport);
        self.render_blocking.insert(to.into());
    }

    pub fn add_js_module(&mut self, from: &str, to: &str) {
        self.add_dependency(from, to, DependencyKind::JsModule);
    }

    pub fn mark_render_blocking(&mut self, url: &str) {
        self.render_blocking.insert(url.into());
    }

    pub fn is_render_blocking(&self, url: &str) -> bool {
        self.render_blocking.contains(url)
    }

    pub fn dependencies(&self, url: &str) -> Vec<String> {
        self.edges
            .get(url)
            .map(|v| v.iter().map(|(u, _)| u.clone()).collect())
            .unwrap_or_default()
    }

    pub fn has_cycle(&self) -> bool {
        let mut seen = HashSet::new();
        let mut stack = HashSet::new();
        self.edges
            .keys()
            .any(|n| self.visit(n, &mut seen, &mut stack))
    }

    fn visit(&self, n: &str, seen: &mut HashSet<String>, stack: &mut HashSet<String>) -> bool {
        if stack.contains(n) {
            return true;
        }
        if !seen.insert(n.into()) {
            return false;
        }
        stack.insert(n.into());
        for (m, _) in self.edges.get(n).into_iter().flatten() {
            if self.visit(m, seen, stack) {
                return true;
            }
        }
        stack.remove(n);
        false
    }
}
