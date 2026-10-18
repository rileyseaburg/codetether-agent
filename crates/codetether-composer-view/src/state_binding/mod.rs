//! Local AppState methods grouped by composer responsibility.
//! [crate::impl_app_composer!] binds existing fields to shared editing behavior.
pub mod completion;
pub mod cursor;
pub mod edit;
pub mod history;
pub mod replace;
pub use codetether_slash_command::current_slash_hint;
/// Implement composer adapters on the application's existing state type.
///
/// Invoke within a child module of the state module to retain the composer
/// accessor's parent-scoped visibility. Public cursor/edit/history methods keep
/// their existing signatures.
/// # Usage
/// ```text
/// codetether_composer_view::impl_app_composer!(super::AppState);
/// ```
#[macro_export]
macro_rules! impl_app_composer {
    ($state:ty) => {
        $crate::__app_composer_cursor!($state);
        $crate::__app_composer_edit!($state);
        $crate::__app_composer_replace!($state);
        $crate::__app_composer_history!($state);
        $crate::__app_composer_completion!($state);
    };
}
