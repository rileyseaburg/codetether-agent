use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// AgentSkill data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::AgentSkill { id: String::new(), name: String::new(), description: String::new(), tags: vec![], examples: vec![], input_modes: vec![], output_modes: vec![] };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct AgentSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub examples: Vec<String>,
    #[serde(default)]
    pub input_modes: Vec<String>,
    #[serde(default)]
    pub output_modes: Vec<String>,
}
