//! Protocol compatibility window for the mux client.

use crate::mux::protocol::VERSION;

/// Only multi-session servers speak a compatible dialect: a connection binds
/// to a session at authentication, which older servers cannot honor.
pub(super) fn supported(version: u16) -> bool {
    version == VERSION
}

#[cfg(test)]
mod tests {
    #[test]
    fn only_the_multi_session_protocol_is_accepted() {
        assert!(!super::supported(7));
        assert!(super::supported(8));
        assert!(!super::supported(9));
    }
}
