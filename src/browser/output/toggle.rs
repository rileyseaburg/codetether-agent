use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ToggleOutput {
    pub ok: bool,
    pub selected: Option<String>,
    pub active_class: bool,
}
