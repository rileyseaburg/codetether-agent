//! Role credentials plus their hard expiry, shared by the refresh flow.

use crate::provider::bedrock::AwsCredentials;
use chrono::{DateTime, Utc};

/// STS role credentials plus the instant the underlying session expires.
///
/// The expiration is the real lifetime ceiling for any minted Bedrock API
/// key: when the SSO/STS session dies, every token signed from it dies too.
pub(crate) struct Exported {
    pub creds: AwsCredentials,
    pub expiration: Option<DateTime<Utc>>,
}
