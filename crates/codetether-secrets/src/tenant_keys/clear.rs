//! Overwrite selected owned credential strings before their owner is dropped.
use super::*;
impl TenantProviderKeyPayload {
    /// Overwrite and clear the API key, organization, and header-value strings.
    ///
    /// This matches the task owner cleanup; copies held elsewhere are unaffected.
    pub fn clear_key_strings(&mut self) {
        for secrets in self.providers.values_mut() {
            if let Some(key) = secrets.api_key.as_mut() {
                unsafe {
                    key.as_bytes_mut().fill(0);
                }
                key.clear();
            }
            if let Some(org) = secrets.organization.as_mut() {
                unsafe {
                    org.as_bytes_mut().fill(0);
                }
                org.clear();
            }
            if let Some(headers) = secrets.headers.as_mut() {
                for value in headers.values_mut() {
                    unsafe {
                        value.as_bytes_mut().fill(0);
                    }
                    value.clear();
                }
                headers.clear();
            }
        }
    }
}
