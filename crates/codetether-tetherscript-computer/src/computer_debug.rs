//! Preserve authority diagnostics without exposing the host marker.
use crate::{ComputerAuthority, Host};

impl<H: Host> std::fmt::Debug for ComputerAuthority<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComputerAuthority")
            .field("origins", &self.origins)
            .field("scopes", &self.scopes)
            .finish()
    }
}
