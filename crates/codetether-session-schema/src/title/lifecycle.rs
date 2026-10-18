//! Shared session title lifecycle operations.
/// Internal title binding for an application-local session.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_title_lifecycle {
    ($name:ident, $registry:ty) => {
        impl $name {
            /// Generate a title from the first user message if one is not already
            /// set.
            pub async fn generate_title(&mut self) -> $crate::__private::anyhow::Result<()> {
                $crate::__private::titles::generate(self).await
            }
            /// Regenerate the title from the first user message, even if already
            /// set.
            pub async fn regenerate_title(&mut self) -> $crate::__private::anyhow::Result<()> {
                $crate::__private::titles::regenerate(self).await
            }
            /// Set a custom title for the session.
            ///
            /// Whitespace is collapsed before persistence; empty or injected bootstrap
            /// context clears the title so it can be regenerated safely.
            ///
            /// # Arguments
            ///
            /// * `title` - Candidate user-facing title.
            pub fn set_title(&mut self, title: impl Into<::std::string::String>) {
                $crate::__private::titles::set(self, title);
            }
            /// Clear the title, allowing it to be regenerated on the next call to
            /// [`Session::generate_title`].
            pub fn clear_title(&mut self) {
                $crate::__private::titles::clear(self);
            }
            /// React to a context change (directory change, model change, etc.).
            /// Bumps `updated_at` and optionally regenerates the title.
            pub async fn on_context_change(
                &mut self,
                regenerate_title: bool,
            ) -> $crate::__private::anyhow::Result<()> {
                $crate::__private::titles::context_change(self, regenerate_title).await
            }
        }
    };
}
