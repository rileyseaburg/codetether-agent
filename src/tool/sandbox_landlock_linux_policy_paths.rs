//! Landlock access rules derived from explicit policy roots.

use super::{PathRule, rule, sys};
use crate::tool::sandbox::SandboxPolicy;

pub(super) fn rules(policy: &SandboxPolicy) -> Result<Vec<PathRule>, &'static str> {
    let mut rules = Vec::new();
    for path in &policy.allowed_paths {
        rules.push(rule(path, sys::READ_ACCESS | sys::WRITE_ACCESS)?);
    }
    for path in &policy.read_only_paths {
        rules.push(rule(path, sys::READ_ACCESS)?);
    }
    Ok(rules)
}