//! Canonical target lookup across durable manifest trees.

use super::manifest::Manifest;
use std::collections::HashMap;

#[path = "tree_target/path.rs"]
mod path;

pub(super) fn find(manifests: &[Manifest], current: &str, target: &str) -> Option<Manifest> {
    let entries = manifests
        .iter()
        .map(|manifest| (manifest.child_session_id.as_str(), manifest))
        .collect::<HashMap<_, _>>();
    let root = path::root(current, &entries);
    if let Some(manifest) = entries.get(target) {
        return path::canonical(target, &root, &entries).map(|_| (*manifest).clone());
    }
    let current_path = path::canonical(current, &root, &entries)?;
    let expected = if target.starts_with('/') {
        target.to_string()
    } else {
        format!("{current_path}/{target}")
    };
    entries.iter().find_map(|(id, manifest)| {
        (path::canonical(id, &root, &entries).as_deref() == Some(&expected))
            .then(|| (*manifest).clone())
    })
}

#[cfg(test)]
#[path = "tree_target_tests.rs"]
mod tests;
