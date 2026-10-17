//! Data prepared before applying Linux Landlock restrictions.

use std::ffi::CString;

#[derive(Debug, Clone)]
/// Owned path rules to install when the child is spawned.
pub struct Rules {
    pub paths: Vec<PathRule>,
    pub handled_access: u64,
}

#[derive(Debug, Clone)]
/// A NUL-terminated path and its allowed filesystem access mask.
pub struct PathRule {
    pub path: CString,
    pub access: u64,
}

#[derive(Debug)]
/// Rule preparation outcome and optional confinement fallback reason.
pub struct Prepared {
    pub rules: Option<Rules>,
    pub fallback: Option<String>,
}
