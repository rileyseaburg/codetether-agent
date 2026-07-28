//! Log entry construction helpers.

use super::{NetworkLogEntry, RouteAction, RouteRequest};

impl NetworkLogEntry {
    pub(crate) fn new(sequence: u64, request: RouteRequest, action: RouteAction) -> Self {
        Self {
            sequence,
            method: request.method,
            url: request.url,
            headers: request.headers,
            body: request.body,
            action,
            security: request.security,
        }
    }
}
