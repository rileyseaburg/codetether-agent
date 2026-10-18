//! Application-local replace methods for the shared composer.
/// Internal composer-state method declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __app_composer_replace {
    ($state:ty) => {
        impl $state {
            pub fn replace_input_chars(
                &mut self,
                range: ::std::ops::Range<usize>,
                replacement: &str,
            ) {
                self.composer().replace_input_chars(range, replacement);
            }
        }
    };
}
