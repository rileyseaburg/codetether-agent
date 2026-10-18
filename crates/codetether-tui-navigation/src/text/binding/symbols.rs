//! Frontend symbols actions selected by shared text-input policy.
/// Implementation detail of [crate::declare_frontend_text_input].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_text_symbols {
    ($dismiss:path, $open:path, $refresh:path) => {
        mod symbols {
            use super::{Action, App};
            pub(super) fn apply(app: &mut App, action: &Action) -> bool {
                match action {
                    Action::DismissMention => {
                        $dismiss(app);
                    }
                    Action::OpenMention => {
                        $open(app);
                    }
                    Action::SymbolChar(c) => {
                        app.state.symbol_search.handle_char(*c);
                    }
                    Action::SymbolBackspace => {
                        app.state.symbol_search.handle_backspace();
                    }
                    Action::RefreshSymbols => {
                        $refresh(app);
                    }
                    _ => return false,
                }
                true
            }
        }
    };
}
