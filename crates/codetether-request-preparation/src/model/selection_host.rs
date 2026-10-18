/// Registry listing needed to resolve an initial model selection.
/// # Implementors
/// Extend the step-model host with the native registry's configured names.
pub trait SelectionHost<R, S>: crate::step_model::Host<R, S> {
    /// List configured provider names in the native registry's order.
    fn providers(&self) -> Vec<&str>;
}
