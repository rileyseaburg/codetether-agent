//! Parsed mermaid model shared by the parser and the renderers.

/// Shape of a flowchart node, derived from its bracket syntax.
///
/// # Examples
///
/// ```rust
/// use codetether_mermaid::model::NodeShape;
///
/// let shape = NodeShape::Round;
/// match shape {
///     NodeShape::Rect => {}
///     NodeShape::Round => {}
///     NodeShape::Diamond => {}
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeShape {
    /// `id[label]`
    Rect,
    /// `id(label)`
    Round,
    /// `id{label}`
    Diamond,
}

/// A flowchart node or a sequence-diagram participant.
/// # Examples
/// ```
/// use codetether_mermaid::model::{Node, NodeShape};
/// let node = Node { id: "A".into(), label: "Start".into(), shape: NodeShape::Rect };
/// assert_eq!(node.id, "A");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    /// Mermaid identifier used by edges.
    pub id: String,
    /// Display label; defaults to the identifier.
    pub label: String,
    /// Border style to draw.
    pub shape: NodeShape,
}

/// A directed connection between two [`Node`]s.
/// # Examples
/// ```
/// let edge = codetether_mermaid::model::Edge { from: "A".into(), to: "B".into(), label: None, dotted: false };
/// assert!(!edge.dotted);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    /// Source node id.
    pub from: String,
    /// Target node id.
    pub to: String,
    /// Optional edge label (`-->|text|` or `A->>B: text`).
    pub label: Option<String>,
    /// True when the arrow was dotted (`-.->`).
    pub dotted: bool,
}

/// Which mermaid diagram family a fenced block declared.
/// # Examples
/// ```
/// use codetether_mermaid::model::DiagramKind;
/// match DiagramKind::Sequence { DiagramKind::Sequence => {}, DiagramKind::FlowchartVertical | DiagramKind::FlowchartHorizontal => {} }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagramKind {
    /// `flowchart`/`graph` with top-down layout.
    FlowchartVertical,
    /// `flowchart`/`graph` with left-right layout.
    FlowchartHorizontal,
    /// `sequenceDiagram`.
    Sequence,
}

/// A parsed mermaid diagram ready for rendering.
/// # Examples
/// ```
/// use codetether_mermaid::model::{Diagram, DiagramKind};
/// let diagram = Diagram { kind: DiagramKind::Sequence, nodes: Vec::new(), edges: Vec::new() };
/// assert!(diagram.edges.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagram {
    /// Diagram family and orientation.
    pub kind: DiagramKind,
    /// Nodes in declaration order.
    pub nodes: Vec<Node>,
    /// Edges in declaration order.
    pub edges: Vec<Edge>,
}
