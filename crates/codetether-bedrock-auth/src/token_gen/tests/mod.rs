//! Preserved token-generation golden vectors.
use super::generate_at;
use crate::AwsCredentials;
use base64::Engine;
use chrono::{TimeZone, Utc};

pub(super) fn test_creds() -> AwsCredentials {
    AwsCredentials {
        access_key_id: "AKIAIOSFODNN7EXAMPLE".into(),
        secret_access_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".into(),
        session_token: None,
    }
}
mod matches_botocore_golden_vector;
mod session_token_is_included_in_signed_query;
mod session_token_matches_botocore_golden_vector;
