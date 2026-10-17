use super::*;

/// Agent Card - self-describing manifest (full proto parity)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// AgentCard data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::AgentCard { name: String::new(), description: String::new(), url: String::new(), version: String::new(), protocol_version: String::new(), preferred_transport: None, additional_interfaces: vec![], capabilities: codetether_a2a_core::AgentCapabilities { streaming: false, push_notifications: false, state_transition_history: false, extensions: vec![] }, skills: vec![], default_input_modes: vec![], default_output_modes: vec![], provider: None, icon_url: None, documentation_url: None, security_schemes: std::collections::HashMap::new(), security: vec![], supports_authenticated_extended_card: false, signatures: vec![] };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct AgentCard {
    pub name: String,
    pub description: String,
    pub url: String,
    pub version: String,
    #[serde(default = "default_protocol_version")]
    pub protocol_version: String,
    /// Transport of the preferred endpoint. Defaults to "JSONRPC".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_transport: Option<String>,
    /// Additional supported transports
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_interfaces: Vec<AgentInterface>,
    pub capabilities: AgentCapabilities,
    pub skills: Vec<AgentSkill>,
    #[serde(default)]
    pub default_input_modes: Vec<String>,
    #[serde(default)]
    pub default_output_modes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<AgentProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    /// Security scheme definitions
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub security_schemes: HashMap<String, SecurityScheme>,
    /// Security requirements (OR of ANDs)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<SecurityRequirement>,
    /// Whether the agent supports an extended card when authenticated
    #[serde(default)]
    pub supports_authenticated_extended_card: bool,
    /// JWS signatures for this agent card
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signatures: Vec<AgentCardSignature>,
}
