//! Frontend embedded-editor key handling and state bindings.
mod host;
/// Declare editor-key handling over the existing frontend buffer and modes.
/// # Examples
/// See the frontend editor-key module for its selected callbacks and types.
#[macro_export]
macro_rules! declare_frontend_editor_keys {
    ($app:path, $mode:path, $key:path; $approval:path, $finder:path $(,)?) => {
        use $app as App;
        use $key as KeyEvent;
        use ::std::path::Path;
        $crate::__frontend_editor_keys_host!($mode; $approval, $finder);
        pub(crate) fn handle_editor_key(app: &mut App, cwd: &Path, key: KeyEvent) -> bool {
            $crate::interactive::handle_editor_key(&mut host::Native(app), cwd, key)
        }
    };
}
