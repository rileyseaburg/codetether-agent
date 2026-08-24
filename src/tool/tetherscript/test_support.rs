//! Shared environment guards for plugin process tests.

macro_rules! require_sandbox {
    () => {
        if let Some(reason) = $crate::tool::sandbox::unavailable_reason() {
            panic!("mandatory sandbox unavailable: {reason}");
        }
    };
}

pub(crate) use require_sandbox;
