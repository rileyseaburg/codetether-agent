//! Wire envelopes for OPA queries and responses.
use crate::{PolicyResource, PolicyUser};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(crate) struct OpaInput {
    pub(crate) input: OpaInputBody,
}

#[derive(Serialize)]
pub(crate) struct OpaInputBody {
    pub(crate) user: PolicyUser,
    pub(crate) action: String,
    pub(crate) resource: PolicyResource,
}

#[derive(Deserialize)]
pub(crate) struct OpaResponse {
    pub(crate) result: Option<bool>,
}
pub(crate) fn input(
    user: &PolicyUser,
    action: &str,
    resource: Option<&PolicyResource>,
) -> OpaInput {
    OpaInput {
        input: OpaInputBody {
            user: user.clone(),
            action: action.to_string(),
            resource: resource.cloned().unwrap_or_default(),
        },
    }
}
