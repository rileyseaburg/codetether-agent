//! Computer grant records and typed host authority.
use crate::Host;
use std::{collections::HashSet, rc::Rc};

use tetherscript::{
    capability::Authority,
    value::{Runtime, Value},
};

/// Computer capability grant parameters passed from tool input.
///
/// # Examples
/// ```rust
/// let grant=codetether_tetherscript_computer::ComputerGrant {
///     enabled: true, origins: vec![], scopes: vec!["computer.inspect".into()],
/// };
/// assert!(grant.enabled);
/// ```
pub struct ComputerGrant {
    pub enabled: bool,
    pub origins: Vec<String>,
    pub scopes: Vec<String>,
}

/// Harness-side authority for the v1 `computer` capability.
///
/// # Examples
/// ```rust
/// # fn example<H:codetether_tetherscript_computer::Host>() {
/// let _authority=codetether_tetherscript_computer::ComputerAuthority::<H>::new(vec![],vec!["computer.inspect".into()]);
/// # }
/// ```
#[derive(Clone)]
pub struct ComputerAuthority<H: Host> {
    pub(super) host: std::marker::PhantomData<H>,
    pub(super) origins: Vec<String>,
    pub(super) scopes: HashSet<String>,
}

impl<H: Host> ComputerAuthority<H> {
    /// Construct an authority with normalized origins and the selected scopes.
    ///
    /// # Arguments
    /// * `origins` - Allowed origins; trailing slashes are removed.
    /// * `scopes` - Allowed computer action categories.
    /// # Returns
    /// A reference-counted native authority for the script runtime.
    /// # Examples
    /// ```rust
    /// # fn example<H:codetether_tetherscript_computer::Host>() {
    /// let _authority=codetether_tetherscript_computer::ComputerAuthority::<H>::new(vec![],vec![]);
    /// # }
    /// ```
    pub fn new(origins: Vec<String>, scopes: Vec<String>) -> Rc<dyn Authority> {
        Rc::new(Self {
            host: std::marker::PhantomData,
            origins: origins.into_iter().map(normalize_origin).collect(),
            scopes: scopes.into_iter().collect(),
        })
    }

    /// Return every supported computer capability scope.
    pub fn all_scopes() -> Vec<String> {
        super::computer_scopes::all()
    }
}

fn normalize_origin(origin: String) -> String {
    origin.trim_end_matches('/').to_string()
}

impl<H: Host> Authority for ComputerAuthority<H> {
    fn narrow(&self, params: &Value) -> Result<Rc<dyn Authority>, String> {
        super::computer_narrow::narrow::<H>(self, params)
    }

    fn invoke(&self, _rt: &mut dyn Runtime, method: &str, args: &[Value]) -> Result<Value, String> {
        super::computer_invoke::invoke::<H>(self, method, args)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
