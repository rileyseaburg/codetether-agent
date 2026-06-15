//! Regression test: `!command` runs through `$SHELL`, not just `/bin/sh`.
//!
//! On Debian/Ubuntu `/bin/sh` is dash, which rejects bash-only syntax
//! such as `[[ ... ]]`. Honoring `$SHELL` makes `!` commands behave like
//! the user's interactive shell.

use crate::tui::app::state::App;
use crate::tui::chat::message::MessageType;

#[tokio::test]
async fn bang_prefix_supports_bash_only_syntax() {
    // SAFETY: single-threaded test; sets SHELL for this process only.
    unsafe { std::env::set_var("SHELL", "/bin/bash") };
    let mut app = App::default();
    let cwd = std::path::Path::new(".");

    assert!(super::run(&mut app, cwd, "![[ 1 == 1 ]] && echo bashok").await);
    match &app.state.messages.last().expect("message").message_type {
        MessageType::ToolResult {
            output, success, ..
        } => {
            assert!(*success, "bash syntax should succeed: {output}");
            assert!(output.contains("bashok"));
        }
        other => panic!("expected ToolResult, got {other:?}"),
    }
}
