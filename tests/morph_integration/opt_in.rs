//! The Morph backend must remain explicitly opt-in.

use super::environment::{self, EnvGuard};
use codetether_agent::tool::morph_backend;

#[tokio::test]
async fn morph_backend_is_opt_in() {
    let _lock = environment::lock().lock().await;
    let _guard = EnvGuard::new("CODETETHER_MORPH_TOOL_BACKEND");
    unsafe {
        std::env::remove_var("CODETETHER_MORPH_TOOL_BACKEND");
    }
    assert!(!morph_backend::should_use_morph_backend());
}
