//! Capabilities: first-class authority values.
//!
//! TetherScript is an agent habitat, not a sandbox. Capabilities are the grants by
//! which an agent touches the world — filesystem, network, mail, etc. The
//! agent holds them as values, passes them to functions, and narrows them
//! to delegate subsets of authority to sub-computations.
//!
//! The TetherScript-visible surface is `Value::Capability(Rc<Capability>)`. Every
//! side-effecting native goes through `Authority::invoke`; there is no
//! ambient way to read a file or open a socket.
//!
//! # Revocation
//!
//! Each capability carries a `Vec<Rc<Cell<bool>>>` of revocation flags —
//! its own, plus every ancestor's. `invoke` fails if *any* flag is set.
//! `narrow` clones the parent's flags and appends a fresh one, so:
//!   - revoking the parent kills the parent and all children
//!   - revoking a child kills that child and any further narrowings of it,
//!     but leaves the parent alone
//!
//! # Attenuation
//!
//! `Authority::narrow(params)` returns a *new* `Authority` encoding the
//! requested subset of the current one, or an error if the request can't
//! be expressed as a narrowing. The narrowed authority never grants more
//! than the parent did — that invariant lives in each `Authority` impl
//! and is the load-bearing security property of this system.

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::value::{Runtime, Value};

thread_local! {
    static AUDIT_LOG: RefCell<Option<Vec<CapabilityAuditEvent>>> = const { RefCell::new(None) };
}

/// Structured record emitted for capability boundary operations while audit
/// capture is enabled by the host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityAuditEvent {
    pub kind: String,
    pub method: String,
    pub allowed: bool,
    pub message: Option<String>,
}

impl CapabilityAuditEvent {
    fn allowed(kind: &str, method: &str) -> Self {
        Self {
            kind: kind.to_string(),
            method: method.to_string(),
            allowed: true,
            message: None,
        }
    }

    fn denied(kind: &str, method: &str, message: impl Into<String>) -> Self {
        Self {
            kind: kind.to_string(),
            method: method.to_string(),
            allowed: false,
            message: Some(message.into()),
        }
    }
}

/// Run `f` while collecting capability audit events for this thread.
///
/// Embedders can wrap plugin load/call execution with this helper to persist or
/// inspect every capability invoke/narrow/revoke decision without changing
/// individual authority implementations.
pub fn with_audit_capture<F, R>(f: F) -> (Vec<CapabilityAuditEvent>, R)
where
    F: FnOnce() -> R,
{
    let prev = AUDIT_LOG.with(|log| log.replace(Some(Vec::new())));
    let result = f();
    let events = AUDIT_LOG.with(|log| log.replace(prev).unwrap_or_default());
    (events, result)
}

fn emit_audit(event: CapabilityAuditEvent) {
    AUDIT_LOG.with(|log| {
        if let Some(events) = log.borrow_mut().as_mut() {
            events.push(event);
        }
    });
}

/// The TetherScript-visible capability value.
pub struct Capability {
    /// Stable kind tag. "fs", "http", "mail". Used in error messages and
    /// debug output — not a security primitive.
    pub kind: String,

    /// The per-kind authority this capability carries. Attenuation and
    /// dispatch both live inside this trait object.
    pub authority: Rc<dyn Authority>,

    /// Revocation flags. This node's flag is the last entry; everything
    /// before is inherited from ancestors via narrowing. Any flag set →
    /// capability is dead and every invoke/narrow fails.
    pub revoked_flags: Vec<Rc<Cell<bool>>>,
}

impl Capability {
    /// Build a root capability (no ancestors). One fresh revocation flag.
    pub fn new_root(kind: impl Into<String>, authority: Rc<dyn Authority>) -> Rc<Self> {
        Rc::new(Capability {
            kind: kind.into(),
            authority,
            revoked_flags: vec![Rc::new(Cell::new(false))],
        })
    }

    /// True if any flag on the ancestor chain (including this node) is set.
    pub fn is_revoked(&self) -> bool {
        self.revoked_flags.iter().any(|f| f.get())
    }

    /// Flip this node's own flag. Leaves ancestor flags alone (revoking a
    /// child doesn't revoke the parent) but kills every further narrowing
    /// of this capability.
    pub fn revoke(&self) {
        if let Some(own) = self.revoked_flags.last() {
            own.set(true);
        }
        emit_audit(CapabilityAuditEvent::allowed(&self.kind, "revoke"));
    }

    /// Invoke a method on this capability, checking revocation first.
    pub fn invoke(
        &self,
        rt: &mut dyn Runtime,
        method: &str,
        args: &[Value],
    ) -> Result<Value, String> {
        if self.is_revoked() {
            let message = format!("{}: capability has been revoked", self.kind);
            emit_audit(CapabilityAuditEvent::denied(&self.kind, method, &message));
            return Err(message);
        }
        match self.authority.invoke(rt, method, args) {
            Ok(value) => {
                emit_audit(CapabilityAuditEvent::allowed(&self.kind, method));
                Ok(value)
            }
            Err(message) => {
                emit_audit(CapabilityAuditEvent::denied(&self.kind, method, &message));
                Err(message)
            }
        }
    }

    /// Produce a narrowed child capability. Inherits every ancestor flag +
    /// its own fresh flag. Fails if revoked or if the requested narrowing
    /// can't be expressed.
    pub fn narrow(&self, params: &Value) -> Result<Rc<Capability>, String> {
        if self.is_revoked() {
            let message = format!("{}: cannot narrow a revoked capability", self.kind);
            emit_audit(CapabilityAuditEvent::denied(&self.kind, "narrow", &message));
            return Err(message);
        }
        let narrowed = match self.authority.narrow(params) {
            Ok(authority) => {
                emit_audit(CapabilityAuditEvent::allowed(&self.kind, "narrow"));
                authority
            }
            Err(message) => {
                emit_audit(CapabilityAuditEvent::denied(&self.kind, "narrow", &message));
                return Err(message);
            }
        };
        let mut flags = self.revoked_flags.clone();
        flags.push(Rc::new(Cell::new(false)));
        Ok(Rc::new(Capability {
            kind: self.kind.clone(),
            authority: narrowed,
            revoked_flags: flags,
        }))
    }
}

/// The per-kind capability trait. Each capability kind (fs, http, mail)
/// ships an implementation.
pub trait Authority: Any {
    /// Attenuate: return a new authority granting a *subset* of `self`.
    /// `params` is a TetherScript `Value::Map` so narrowing is scriptable.
    /// Per-kind logic enforces the "narrowed ⊆ parent" invariant.
    fn narrow(&self, params: &Value) -> Result<Rc<dyn Authority>, String>;

    /// Perform a method call. `rt` lets capabilities invoke TetherScript callables
    /// (e.g. http.listen handing requests to a TetherScript handler). Permission
    /// checks live here — per-kind, because scopes differ per kind.
    fn invoke(&self, rt: &mut dyn Runtime, method: &str, args: &[Value]) -> Result<Value, String>;

    /// Downcast hook. Capability-aware code that needs a concrete authority
    /// type (e.g. passing an http capability to a fn that needs exactly http)
    /// uses this via the standard `Any` pattern.
    fn as_any(&self) -> &dyn Any;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestAuthority;

    impl Authority for TestAuthority {
        fn narrow(&self, params: &Value) -> Result<Rc<dyn Authority>, String> {
            match params {
                Value::Bool(false) => Err("narrow denied".into()),
                _ => Ok(Rc::new(TestAuthority)),
            }
        }

        fn invoke(
            &self,
            _rt: &mut dyn Runtime,
            method: &str,
            _args: &[Value],
        ) -> Result<Value, String> {
            match method {
                "ok" => Ok(Value::Nil),
                _ => Err("method denied".into()),
            }
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    struct NoopRuntime;

    impl Runtime for NoopRuntime {
        fn invoke(&mut self, _callee: &Value, _args: &[Value]) -> Result<Value, String> {
            Ok(Value::Nil)
        }
    }

    #[test]
    fn audit_capture_records_capability_decisions() {
        let cap = Capability::new_root("test", Rc::new(TestAuthority));
        let mut rt = NoopRuntime;

        let (events, ()) = with_audit_capture(|| {
            cap.invoke(&mut rt, "ok", &[]).unwrap();
            cap.invoke(&mut rt, "nope", &[]).unwrap_err();
            assert!(cap.narrow(&Value::Bool(false)).is_err());
            cap.revoke();
            cap.invoke(&mut rt, "ok", &[]).unwrap_err();
        });

        assert_eq!(events.len(), 5);
        assert_eq!(events[0].method, "ok");
        assert!(events[0].allowed);
        assert_eq!(events[1].method, "nope");
        assert!(!events[1].allowed);
        assert_eq!(events[2].method, "narrow");
        assert!(!events[2].allowed);
        assert_eq!(events[3].method, "revoke");
        assert!(events[3].allowed);
        assert_eq!(
            events[4].message.as_deref(),
            Some("test: capability has been revoked")
        );
    }
}
