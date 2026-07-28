//! Public browser authority implementation.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::time::Duration;

use super::trace::BrowserTrace;
use crate::capability::Authority;

#[path = "authority_trait.rs"]
mod authority_trait;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Tetherscript browser authority exposed as a capability value.
#[derive(Clone, Debug)]
pub struct BrowserAuthority {
    pub(crate) endpoint: String,
    pub(crate) allowed_origins: Vec<String>,
    pub(crate) allowed_scopes: HashSet<String>,
    pub(crate) path_prefix: Option<String>,
    pub(crate) storage_scope: Option<String>,
    pub(crate) human_approval: bool,
    pub(crate) timeout: Duration,
    pub(crate) trace: Rc<RefCell<BrowserTrace>>,
}

impl BrowserAuthority {
    /// Create a root browser authority for a tetherscript browser endpoint.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(endpoint: &str, origins: Vec<String>, scopes: Vec<String>) -> Rc<dyn Authority> {
        Rc::new(Self {
            endpoint: endpoint.trim_end_matches('/').into(),
            allowed_origins: origins
                .into_iter()
                .map(super::origin::normalize_origin)
                .collect(),
            allowed_scopes: scopes.into_iter().collect(),
            path_prefix: None,
            storage_scope: None,
            human_approval: false,
            timeout: DEFAULT_TIMEOUT,
            trace: Rc::new(RefCell::new(BrowserTrace::default())),
        })
    }

    /// Return every browser scope understood by the authority.
    pub fn all_scopes() -> Vec<String> {
        super::scope::all_scopes()
    }
}
