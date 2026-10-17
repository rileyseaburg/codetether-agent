//! Data prepared before applying Linux Landlock restrictions.

use std::ffi::CString;

#[derive(Debug, Clone)]
pub(crate) struct Rules {
    pub paths: Vec<PathRule>,
    pub handled_access: u64,
}

#[derive(Debug, Clone)]
pub(crate) struct PathRule {
    pub path: CString,
    pub access: u64,
}

#[derive(Debug)]
pub(crate) struct Prepared {
    pub rules: Option<Rules>,
    pub fallback: Option<String>,
}