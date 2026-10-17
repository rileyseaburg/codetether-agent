use super::*;
impl CollapseController {
    pub fn new(policy: CollapsePolicy) -> Self {
        Self {
            policy,
            first_seen_at: HashMap::new(),
            consecutive_compile_failures: HashMap::new(),
            consecutive_resource_failures: HashMap::new(),
            invariant_breaks: HashMap::new(),
            promoted_subtask_id: None,
        }
    }
}
