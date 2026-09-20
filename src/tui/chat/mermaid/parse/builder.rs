//! Accumulate nodes and edges from flowchart statements.

use crate::tui::chat::mermaid::model::{Edge, Node};
use crate::tui::chat::mermaid::parse::{chain, node::parse_node};

/// Mutable collection state shared while parsing statements.
#[derive(Default)]
pub struct Builder {
    /// Nodes in first-seen order.
    pub nodes: Vec<Node>,
    /// Edges in declaration order.
    pub edges: Vec<Edge>,
}

impl Builder {
    /// Record a node, keeping the first non-default label seen for an id.
    pub fn push_node(&mut self, node: Node) {
        match self.nodes.iter_mut().find(|n| n.id == node.id) {
            Some(existing) if existing.label == existing.id => {
                existing.label = node.label;
                existing.shape = node.shape;
            }
            Some(_) => {}
            None => self.nodes.push(node),
        }
    }

    /// Parse one flowchart statement, adding its nodes and each chained edge.
    pub fn push_statement(&mut self, line: &str) {
        let segments = chain::segments(line);
        if segments.is_empty() {
            if let Some(node) = parse_node(line) {
                self.push_node(node);
            }
            return;
        }
        for seg in segments {
            let (Some(from), Some(to)) = (parse_node(seg.from), parse_node(seg.to)) else {
                continue;
            };
            let edge = Edge {
                from: from.id.clone(),
                to: to.id.clone(),
                label: seg.label,
                dotted: seg.arrow.contains('.'),
            };
            self.push_node(from);
            self.push_node(to);
            self.edges.push(edge);
        }
    }
}
