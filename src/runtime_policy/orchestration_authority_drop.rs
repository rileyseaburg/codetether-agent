//! Permit revocation when an approved orchestration finishes.

use super::{Permit, map};

impl Drop for Permit {
    fn drop(&mut self) {
        let mut active = map()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let remove = match active.get_mut(&self.workspace) {
            Some((token, count)) if token == &self.token && *count > 1 => {
                *count -= 1;
                false
            }
            Some((token, _)) => token == &self.token,
            None => false,
        };
        if remove {
            active.remove(&self.workspace);
        }
    }
}
