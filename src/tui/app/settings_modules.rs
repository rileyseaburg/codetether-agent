// Settings submodule wiring and re-exports, split out so `settings.rs`
// stays within the per-file line budget.

#[path = "settings_access_mode.rs"]
pub mod access_mode;
#[path = "settings_bedrock.rs"]
pub mod bedrock;
#[path = "settings_bedrock_effort.rs"]
pub mod bedrock_effort;
#[path = "settings_codex_effort.rs"]
pub mod codex_effort;
#[path = "settings_dispatch.rs"]
pub mod dispatch;
#[path = "settings_network.rs"]
pub mod network;
#[path = "settings_openrouter_effort.rs"]
pub mod openrouter_effort;

pub use bedrock::bedrock_service_tier_label;
pub use bedrock::cycle_bedrock_service_tier;
pub use bedrock_effort::{bedrock_thinking_effort_label, cycle_bedrock_thinking_effort};
pub use codex_effort::codex_thinking_effort_label;
pub use dispatch::toggle_selected_setting;
pub use network::{network_access_status_message, set_network_access, toggle_network_access};
pub use openrouter_effort::{cycle_openrouter_thinking_effort, openrouter_thinking_effort_label};