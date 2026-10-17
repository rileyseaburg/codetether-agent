//! Authentication declarations and OAuth flow metadata.
//!
//! These serializable contracts are exported from the crate root for transport consumers.
use super::*;
mod security_scheme;
pub use security_scheme::SecurityScheme;
mod oauth_flows;
pub use oauth_flows::OAuthFlows;
mod oauth_flow_implicit;
pub use oauth_flow_implicit::OAuthFlowImplicit;
mod oauth_flow_authorization_code;
pub use oauth_flow_authorization_code::OAuthFlowAuthorizationCode;
mod oauth_flow_client_credentials;
pub use oauth_flow_client_credentials::OAuthFlowClientCredentials;
mod oauth_flow_device_code;
pub use oauth_flow_device_code::OAuthFlowDeviceCode;
mod security_requirement;
pub use security_requirement::SecurityRequirement;
mod authentication_info;
pub use authentication_info::AuthenticationInfo;
