use super::dependency_graph::DependencyGraph;
use std::collections::HashSet;

pub(super) fn find_cycle<'a>(graph: &'a DependencyGraph<'a>) -> Option<Vec<&'a str>> {
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();
    for &node in &graph.ids {
        if !visited.contains(node) {
            let mut path = Vec::new();
            if let Some(cycle) = visit(node, graph, &mut visited, &mut rec_stack, &mut path) {
                return Some(cycle);
            }
        }
    }
    None
}

fn visit<'a>(
    node: &'a str,
    graph: &'a DependencyGraph<'a>,
    visited: &mut HashSet<&'a str>,
    rec_stack: &mut HashSet<&'a str>,
    path: &mut Vec<&'a str>,
) -> Option<Vec<&'a str>> {
    visited.insert(node);
    rec_stack.insert(node);
    path.push(node);
    for &neighbor in graph.edges.get(node).into_iter().flatten() {
        if !visited.contains(neighbor) {
            if let Some(cycle) = visit(neighbor, graph, visited, rec_stack, path) {
                return Some(cycle);
            }
        } else if rec_stack.contains(neighbor) {
            return path
                .iter()
                .position(|&n| n == neighbor)
                .map(|start| path[start..].to_vec());
        }
    }
    path.pop();
    rec_stack.remove(node);
    None
}
