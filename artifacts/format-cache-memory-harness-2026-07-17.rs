//! Isolated test harness for the production format-cache policy and store.

mod tui {
    pub mod chat {
        pub mod message {
            use std::time::SystemTime;

            pub struct ChatMessage {
                pub content: String,
                pub timestamp: SystemTime,
            }
        }
    }
}

mod format_cache {
    type Key = (u128, usize, usize, usize, u8);

    #[path = "../../src/tui/ui/chat_view/format_cache/policy.rs"]
    mod policy;
    #[path = "../../src/tui/ui/chat_view/format_cache/store.rs"]
    mod store;
}
