//! Session title lifecycle, projection and provider selection.
//! [crate::impl_session_titles!] binds shared title operations to the local schema.
mod ai;
mod display;
mod host;
mod lifecycle;
mod selection;

/// Add title generation and display methods to an application-local session.
///
/// The registry type must provide the existing `list` and `get` provider APIs.
/// Selection uses the shared provider preference and default-model policies.
/// # Usage
/// After declaring the session and its lifecycle methods, invoke:
/// ```text
/// codetether_session_schema::impl_session_titles!(Session, ProviderRegistry);
/// ```
#[macro_export]
macro_rules! impl_session_titles {
    ($name:ident, $registry:ty) => {
        $crate::__session_title_host!($name, $registry);
        $crate::__session_title_lifecycle!($name, $registry);
        $crate::__session_title_ai!($name, $registry);
        $crate::__session_title_selection!($name, $registry);
        $crate::__session_title_display!($name, $registry);
    };
}
