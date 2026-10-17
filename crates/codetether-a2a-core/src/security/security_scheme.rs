use super::*;

// ─── Security Types (OAS 3.1 / proto parity) ────────────────────────────

/// Security scheme definition (union), matches proto `SecurityScheme`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
/// SecurityScheme data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::SecurityScheme::MutualTls { description: None };
/// match value {
///     codetether_a2a_core::SecurityScheme::ApiKey { .. } => {},
///     codetether_a2a_core::SecurityScheme::Http { .. } => {},
///     codetether_a2a_core::SecurityScheme::OAuth2 { .. } => {},
///     codetether_a2a_core::SecurityScheme::OpenIdConnect { .. } => {},
///     codetether_a2a_core::SecurityScheme::MutualTls { .. } => {},
/// }
/// ```
pub enum SecurityScheme {
    #[serde(rename = "apiKey")]
    ApiKey {
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        name: String,
        #[serde(rename = "in")]
        location: String, // "query" | "header" | "cookie"
    },
    #[serde(rename = "http")]
    Http {
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        scheme: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        bearer_format: Option<String>,
    },
    #[serde(rename = "oauth2")]
    OAuth2 {
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        flows: Box<OAuthFlows>,
    },
    #[serde(rename = "openIdConnect")]
    OpenIdConnect {
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        open_id_connect_url: String,
    },
    #[serde(rename = "mutualTLS")]
    MutualTls {
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
}
