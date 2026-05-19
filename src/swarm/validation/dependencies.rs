use super::dependency_graph::DependencyGraph;
use super::dependency_issues::{cycle_issue, missing_dependency_issue};
use super::{SwarmValidator, ValidationIssue};
use crate::swarm::SubTask;

impl SwarmValidator {
    pub(super) fn validate_dependencies(
        &self,
        subtasks: &[SubTask],
        issues: &mut Vec<ValidationIssue>,
    ) {
        let graph = DependencyGraph::from_subtasks(subtasks);
        push_missing_dependency_issues(subtasks, &graph, issues);
        if let Some(cycle) = super::dependency_cycle::find_cycle(&graph) {
            issues.push(cycle_issue(&cycle));
        }
    }
}

fn push_missing_dependency_issues(
    subtasks: &[SubTask],
    graph: &DependencyGraph<'_>,
    issues: &mut Vec<ValidationIssue>,
) {
    for subtask in subtasks {
        for dependency in &subtask.dependencies {
            if !graph.ids.contains(dependency.as_str()) {
                issues.push(missing_dependency_issue(subtask, dependency));
            }
        }
    }
}
