#[derive(Clone)]
pub(super) struct TaskInput {
    pub id: Option<String>,
    pub name: String,
    pub instruction: String,
    pub specialty: Option<String>,
}
