//! Frontend view binding for human-answer workflows.
/// Implementation detail of [crate::declare_frontend_answer_hosts].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_answer_view {
    () => {
        mod view {
            use super::Native;
            impl $crate::InputView for Native<'_> {
                fn input(&self) -> &str {
                    &self.app.state.composer.input
                }
                fn rewrite(&mut self, text: String) {
                    self.app.state.composer.cursor = text.chars().count();
                    self.app.state.composer.input = text;
                }
                fn status(&mut self, text: String) {
                    self.app.state.status = text;
                }
                fn history(&mut self, text: String) {
                    self.app.state.composer.push_history(text);
                }
                fn clear(&mut self) {
                    self.app.state.composer.clear_input();
                }
                fn answer_no(&mut self) {
                    self.app.state.answer_review_yes = false;
                }
                fn redraw(&mut self) {
                    self.app.state.needs_redraw = true;
                }
            }
        }
    };
}
