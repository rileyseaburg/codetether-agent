//! Initialize transcript accumulation with the original empty defaults.
use crate::{budget::BoundedMessages, state::State};
use codetether_provider_core::Usage;
impl Default for State {
    fn default() -> Self {
        Self {
            session_meta: None,
            updated_at: None,
            messages: BoundedMessages::new(),
            usage: Usage::default(),
            model: None,
            first_user_text: None,
        }
    }
}
