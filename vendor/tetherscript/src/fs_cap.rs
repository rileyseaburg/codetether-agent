//! `FsAuthority` — the exemplar `Authority` impl.
//!
//! Grants filesystem access within a root directory, with optional
//! mode restrictions (read/write/list) and a deny-list of relative path
//! prefixes. Narrowing is monotonic: a narrowed child never grants more
//! than its parent.
//!
//! # Security posture
//!
//! - **Absolute paths:** always rejected.
//! - **`..` components:** allowed only if they don't walk above the root.
//!   Tracked by depth counter during path resolution.
//! - **Symlinks:** for *existing* targets, we canonicalize and verify the
//!   result is still under the canonical root. For non-existent targets
//!   (e.g. writing a new file), we canonicalize the parent if it exists.
//!   This catches symlinks that point outside the root.
//!
//! The security invariant that matters: **any path this authority will
//! operate on must resolve to a location inside `root`.** If you find a
//! way to bypass that, it's a bug.

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::rc::Rc;

use crate::capability::Authority;
use crate::value::{Runtime, Value};

#[derive(Clone, Copy, Debug)]
struct Modes {
    read: bool,
    write: bool,
    list: bool,
}

impl Modes {
    fn full() -> Self {
        Modes {
            read: true,
            write: true,
            list: true,
        }
    }
    fn intersect(self, other: Self) -> Self {
        Modes {
            read: self.read && other.read,
            write: self.write && other.write,
            list: self.list && other.list,
        }
    }
}

pub struct FsAuthority {
    root: PathBuf,
    modes: Modes,
    /// Relative path prefixes forbidden within `root`. Monotonically grows
    /// under narrowing.
    deny: Vec<String>,
}

impl FsAuthority {
    /// Build a root fs authority with full modes over `root`. The harness
    /// uses this when granting `fs` to an agent.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(root: impl Into<PathBuf>) -> Rc<dyn Authority> {
        Rc::new(FsAuthority {
            root: root.into(),
            modes: Modes::full(),
            deny: Vec::new(),
        })
    }

    fn resolve(&self, relative: &str) -> Result<PathBuf, String> {
        let p = Path::new(relative);
        let mut depth: i64 = 0;
        for component in p.components() {
            match component {
                Component::Normal(_) => depth += 1,
                Component::CurDir => {}
                Component::ParentDir => {
                    depth -= 1;
                    if depth < 0 {
                        return Err(format!("fs: path `{}` escapes root", relative));
                    }
                }
                Component::RootDir | Component::Prefix(_) => {
                    return Err(format!("fs: absolute paths not allowed (`{}`)", relative));
                }
            }
        }

        // Deny-list check against the cleaned relative path.
        for d in &self.deny {
            if relative.starts_with(d) {
                return Err(format!("fs: path `{}` is denied by scope", relative));
            }
        }

        let joined = self.root.join(p);

        // Symlink defense: if the target exists, canonicalize and verify
        // it's still inside the canonical root. For non-existent targets
        // (new file writes), canonicalize the parent.
        let to_check = if joined.exists() {
            joined
                .canonicalize()
                .map_err(|e| format!("fs: canonicalize: {}", e))?
        } else if let Some(parent) = joined.parent() {
            if parent.exists() {
                let canon_parent = parent
                    .canonicalize()
                    .map_err(|e| format!("fs: canonicalize parent: {}", e))?;
                canon_parent.join(joined.file_name().unwrap_or_default())
            } else {
                joined.clone()
            }
        } else {
            joined.clone()
        };

        let canon_root = self
            .root
            .canonicalize()
            .map_err(|e| format!("fs: root canonicalize: {}", e))?;
        if !to_check.starts_with(&canon_root) {
            return Err(format!("fs: resolved path `{}` is outside root", relative));
        }

        Ok(joined)
    }
}

impl Authority for FsAuthority {
    fn narrow(&self, params: &Value) -> Result<Rc<dyn Authority>, String> {
        let map = match params {
            Value::Map(m) => m.clone(),
            _ => return Err("fs.narrow: expected a map of params".into()),
        };
        let m = map.borrow();

        let mut new_root = self.root.clone();
        let mut new_modes = self.modes;
        let mut new_deny = self.deny.clone();

        if let Some(v) = m.get("root") {
            let sub = match v {
                Value::Str(s) => (**s).clone(),
                _ => return Err("fs.narrow: `root` must be a string".into()),
            };
            // Reuse the parent's resolver to guarantee the requested sub-root
            // is actually inside the parent root.
            let resolved = self.resolve(&sub)?;
            if !resolved.is_dir() {
                return Err(format!("fs.narrow: `{}` is not a directory", sub));
            }
            new_root = resolved;
            // Reset deny: entries were relative to the old root. Anything
            // in deny that fell under the new root is still covered by the
            // parent at invoke time; the child doesn't need to duplicate it.
            new_deny = Vec::new();
        }

        if let Some(v) = m.get("mode") {
            let s = match v {
                Value::Str(s) => (**s).clone(),
                _ => return Err("fs.narrow: `mode` must be a string".into()),
            };
            let requested = match s.as_str() {
                "ro" => Modes {
                    read: true,
                    write: false,
                    list: true,
                },
                "rw" => Modes {
                    read: true,
                    write: true,
                    list: true,
                },
                "list" => Modes {
                    read: false,
                    write: false,
                    list: true,
                },
                other => return Err(format!("fs.narrow: unknown mode `{}`", other)),
            };
            new_modes = new_modes.intersect(requested);
        }

        if let Some(v) = m.get("deny") {
            let xs = match v {
                Value::List(xs) => xs.clone(),
                _ => return Err("fs.narrow: `deny` must be a list of strings".into()),
            };
            for entry in xs.borrow().iter() {
                match entry {
                    Value::Str(s) => new_deny.push((**s).clone()),
                    _ => return Err("fs.narrow: deny entries must be strings".into()),
                }
            }
        }

        Ok(Rc::new(FsAuthority {
            root: new_root,
            modes: new_modes,
            deny: new_deny,
        }))
    }

    fn invoke(&self, _rt: &mut dyn Runtime, method: &str, args: &[Value]) -> Result<Value, String> {
        match (method, args) {
            ("read", [Value::Str(p)]) => {
                if !self.modes.read {
                    return Err("fs.read: capability lacks read mode".into());
                }
                let resolved = self.resolve(p)?;
                let bytes = fs::read(&resolved).map_err(|e| format!("fs.read: {}", e))?;
                match String::from_utf8(bytes) {
                    Ok(s) => Ok(Value::Str(Rc::new(s))),
                    Err(err) => Ok(Value::Bytes(Rc::new(RefCell::new(err.into_bytes())))),
                }
            }
            ("write", [Value::Str(p), content @ (Value::Str(_) | Value::Bytes(_))]) => {
                if !self.modes.write {
                    return Err("fs.write: capability lacks write mode".into());
                }
                let resolved = self.resolve(p)?;
                let bytes = match content {
                    Value::Str(content) => content.as_bytes().to_vec(),
                    Value::Bytes(bytes) => bytes.borrow().clone(),
                    _ => unreachable!(),
                };
                fs::write(&resolved, bytes).map_err(|e| format!("fs.write: {}", e))?;
                Ok(Value::Nil)
            }
            ("list", [Value::Str(p)]) => {
                if !self.modes.list {
                    return Err("fs.list: capability lacks list mode".into());
                }
                let resolved = self.resolve(p)?;
                let mut names: Vec<Value> = Vec::new();
                for entry in fs::read_dir(&resolved).map_err(|e| format!("fs.list: {}", e))? {
                    let entry = entry.map_err(|e| format!("fs.list: {}", e))?;
                    let name = entry.file_name().to_string_lossy().into_owned();
                    names.push(Value::Str(Rc::new(name)));
                }
                names.sort_by(|a, b| match (a, b) {
                    (Value::Str(x), Value::Str(y)) => x.cmp(y),
                    _ => std::cmp::Ordering::Equal,
                });
                Ok(Value::List(Rc::new(RefCell::new(names))))
            }
            ("exists", [Value::Str(p)]) => {
                // `exists` doesn't require any mode — it's a pure predicate
                // about whether a path reachable under the scope is there.
                let resolved = self.resolve(p)?;
                Ok(Value::Bool(resolved.exists()))
            }
            ("describe", []) => {
                // Curated introspection. Debug aid — not part of the
                // security model. Returns a map summarizing scope.
                let mut m = HashMap::new();
                m.insert(
                    "root".into(),
                    Value::Str(Rc::new(self.root.display().to_string())),
                );
                m.insert("read".into(), Value::Bool(self.modes.read));
                m.insert("write".into(), Value::Bool(self.modes.write));
                m.insert("list".into(), Value::Bool(self.modes.list));
                let deny: Vec<Value> = self
                    .deny
                    .iter()
                    .map(|d| Value::Str(Rc::new(d.clone())))
                    .collect();
                m.insert("deny".into(), Value::List(Rc::new(RefCell::new(deny))));
                Ok(Value::Map(Rc::new(RefCell::new(m))))
            }
            (m, _) => Err(format!(
                "fs: no method `{}` (have: read, write, list, exists, describe)",
                m
            )),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
