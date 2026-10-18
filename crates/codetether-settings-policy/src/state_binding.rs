//! Application-local settings-policy state adapters.

/// Declare the existing settings-policy methods on a local AppState.
/// # Usage
/// Pass the state type and any native type/variant bindings required by the matcher.
#[macro_export]
macro_rules! impl_app_settings_navigation {
    ($state:ty) => {
        impl $state {
            pub(crate) const SETTINGS_COUNT: usize = $crate::panel::COUNT;

            pub fn settings_select_prev(&mut self) {
                codetether_settings_policy::panel::select_previous(
                    &mut self.selected_settings_index,
                );
            }

            pub fn settings_select_next(&mut self) {
                codetether_settings_policy::panel::select_next(&mut self.selected_settings_index);
            }
        }
    };
}
