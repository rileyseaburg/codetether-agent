//! Policy-aware Git undo and preview operations.
//!
//! [Undo] inspects history and previews affected files before requesting
//! authorization through [Policy]. Reset execution and output formatting have
//! separate modules. [UndoInput] retains the serialized invocation contract.
mod execute;
mod history;
mod input;
mod outcome;
mod policy;
mod preview;
mod reset;
mod schema;
mod tool;
pub use input::UndoInput;
pub use policy::Policy;
/// Undo implementation bound to the application's authorization policy.
///
/// # Examples
/// ```rust
/// fn tool<P:codetether_undo_tool::Policy>()->codetether_undo_tool::Undo<P>{
///     codetether_undo_tool::Undo::new()
/// }
/// ```
pub struct Undo<P>(std::marker::PhantomData<P>);
impl<P: Policy> Undo<P> {
    /// Construct a tool using the policy type's authorization callback.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A tool ready for registration.
    /// # Examples
    /// ```rust
    /// # fn example<P:codetether_undo_tool::Policy>() {
    /// let _tool=codetether_undo_tool::Undo::<P>::new();
    /// # }
    /// ```
    pub const fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<P: Policy> Default for Undo<P> {
    fn default() -> Self {
        Self::new()
    }
}
