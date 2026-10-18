//! state for the planning tools.

#[derive(Debug, Clone)]
pub(crate) struct Plan {
    pub(crate) goal: String,
    pub(crate) steps: Vec<PlanStep>,
    pub(crate) current_step: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct PlanStep {
    pub(crate) description: String,
    pub(crate) completed: bool,
    pub(crate) notes: Option<String>,
}
