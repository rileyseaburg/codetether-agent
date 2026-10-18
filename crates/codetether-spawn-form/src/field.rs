//! Focus options for the spawn form.
//! Form state and terminal presentation require no application runtime.

//! State container for the interactive spawn form.

/// Which form field currently has input focus.
///
/// # Examples
/// ```
/// use codetether_spawn_form::SpawnField;
/// assert!(matches!(SpawnField::default(), SpawnField::Name));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpawnField {
    /// Agent name (required).
    #[default]
    Name,
    /// Free-text system instructions.
    Instructions,
}
