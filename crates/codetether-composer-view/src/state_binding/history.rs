//! Application-local history methods for the shared composer.
/// Internal composer-state method declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __app_composer_history {
    ($state:ty) => {
        impl $state {
            pub fn history_prev(&mut self) -> bool {
                self.composer().history_prev()
            }
            pub fn history_next(&mut self) -> bool {
                self.composer().history_next()
            }
        }

        impl $state {
            pub fn push_history(&mut self, entry: ::std::string::String) {
                self.composer.history.push_history(entry);
            }
        }
    };
}
