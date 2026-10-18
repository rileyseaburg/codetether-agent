//! Application-local completion methods for the shared composer.
/// Internal composer-state method declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __app_composer_completion {
    ($state:ty) => {
        impl $state {
            pub fn refresh_slash_suggestions(&mut self) {
                self.composer
                    .completion
                    .refresh_slash_suggestions(&self.composer.input);
            }

            pub fn slash_suggestions_visible(&self) -> bool {
                $crate::interactive::suggestions_visible(
                    &self.view_mode,
                    self.processing,
                    &self.composer.input,
                )
            }
            pub fn slash_suggestions_navigable(&self) -> bool {
                $crate::interactive::suggestions_navigable(
                    &self.view_mode,
                    self.processing,
                    &self.composer.input,
                    &self.composer.completion,
                )
            }

            /// Usage hint for the currently-typed slash command, if any.
            ///
            /// Looks up the first whitespace-separated token of `input` in
            /// [`super::slash_hints`]. Used by the chat-view suggestions panel to
            /// render `<args>` documentation once the prefix-match list is empty.
            pub fn current_slash_hint(&self) -> Option<&'static str> {
                $crate::state_binding::current_slash_hint(&self.composer.input)
            }

            pub fn select_prev_slash_suggestion(&mut self) {
                self.composer.completion.select_prev_slash_suggestion();
            }

            pub fn select_next_slash_suggestion(&mut self) {
                self.composer.completion.select_next_slash_suggestion();
            }

            pub fn selected_slash_suggestion(&self) -> Option<&str> {
                self.composer.completion.selected_slash_suggestion()
            }

            pub fn apply_selected_slash_suggestion(&mut self) -> bool {
                self.composer().apply_selected_slash_suggestion()
            }
        }
    };
}
