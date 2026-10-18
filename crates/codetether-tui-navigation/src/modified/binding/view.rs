//! Frontend view effects for modifier shortcuts.
/// Implementation detail of [crate::declare_frontend_modified_keys].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_modified_view {
    () => {
        mod view {
            use super::{Action, App};
            pub(super) fn apply(app: &mut App, action: &Action) -> bool {
                match action {
                    Action::SymbolSearch => {
                        app.state.symbol_search.open();
                    }
                    Action::ToggleLayout => {
                        app.state.chat_layout_mode = app.state.chat_layout_mode.toggle();
                    }
                    Action::ClearMessages => {
                        app.state.messages.clear();
                    }
                    Action::ClearStreaming => {
                        app.state.clear_streaming_text();
                    }
                    Action::SaveScroll => {
                        app.state.save_scroll_state();
                    }
                    Action::View(view) => {
                        app.state.view_mode = view.clone();
                    }
                    _ => return false,
                }
                true
            }
        }
    };
}
