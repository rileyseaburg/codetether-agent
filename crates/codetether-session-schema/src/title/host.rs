//! Shared session title host operations.
/// Internal title binding for an application-local session.
#[doc(hidden)]
#[macro_export]
macro_rules! __session_title_host {
    ($name:ident, $registry:ty) => {
        impl $crate::__private::titles::TitleSession for $name {
            fn title(&self) -> ::std::option::Option<&str> {
                self.title.as_deref()
            }
            fn messages(&self) -> &[$crate::__private::Message] {
                &self.messages
            }
            fn store_title(&mut self, title: ::std::option::Option<::std::string::String>) {
                self.title = title;
            }
            fn touch(&mut self) {
                self.updated_at = $crate::__private::Utc::now();
            }
        }
    };
}
