//! DOM identity fingerprint.

use super::hash::{hash_str, norm, stable_attrs};
use super::node::DomNode;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DomFingerprint {
    pub tag: String,
    pub text_hash: u64,
    pub attr_hash: u64,
    pub structural_hash: u64,
}

impl DomFingerprint {
    pub fn from_dom(root: &DomNode, path: &[usize]) -> Option<Self> {
        let n = root.get(path)?;
        Some(Self {
            tag: n.tag.clone(),
            text_hash: hash_str(&norm(&n.text)),
            attr_hash: hash_str(&stable_attrs(n)),
            structural_hash: hash_str(
                &path
                    .iter()
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join("/"),
            ),
        })
    }

    pub fn similarity(&self, other: &Self) -> f32 {
        let mut s = 0.0;
        if self.tag == other.tag {
            s += 0.30;
        }
        if self.text_hash == other.text_hash {
            s += 0.30;
        }
        if self.attr_hash == other.attr_hash {
            s += 0.25;
        }
        if self.structural_hash == other.structural_hash {
            s += 0.15;
        }
        s
    }
}
