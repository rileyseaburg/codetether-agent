use super::super::sandbox_bwrap_push::{push_pair, push_triple};
use super::SandboxPolicy;

const PROTECTED: &[&str] = &[".git", ".codetether", ".codex", ".agents"];

pub(super) fn mounts(out: &mut Vec<String>, policy: &SandboxPolicy) {
    for root in &policy.allowed_paths {
        for name in PROTECTED {
            let path = root.join(name);
            let value = path.display().to_string();
            if path.exists() {
                push_triple(out, "--ro-bind", &value, &value);
            } else {
                push_pair(out, "--dir", &value);
            }
        }
    }
}
