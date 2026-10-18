//! Shared session title display operations.
/// Internal title binding for an application-local session.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_title_display {
    ($name:ident, $registry:ty) => {
        impl $name {
            /// Return a normalized title or a compact identifier for display.
            ///
            /// # Returns
            ///
            /// The stored title with whitespace collapsed, the first eight identifier
            /// characters when no safe title exists, or `new` when both are absent.
            ///
            /// # Examples
            ///
            /// ```rust,no_run
            /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
            /// use codetether_agent::session::Session;
            /// let mut session = Session::new().await.unwrap();
            /// session.set_title("  Review   storage ");
            /// assert_eq!(session.display_label(), "Review storage");
            /// # });
            /// ```
            pub fn display_label(&self) -> ::std::string::String {
                $crate::__private::titles::label(self.title.as_deref(), &self.id)
            }
        }
    };
}
