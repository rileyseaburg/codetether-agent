//! JSON schema advertised for persistent command execution.

use serde_json::{Value, json};

pub(super) fn schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "cmd": {"type": "string", "description": "Shell command to execute."},
            "workdir": {"type": "string", "description": "Working directory; defaults to the workspace."},
            "tty": {"type": "boolean", "description": "Allocate a PTY and keep interactive stdin open for write_stdin."},
            "yield_time_ms": {"type": "integer", "description": "Wait before yielding output; effective range 250-30000 ms."},
            "max_output_tokens": {"type": "integer", "description": "Output token budget; defaults to 10000 and is policy-capped."},
            "shell": {"type": "string", "description": "Shell binary; defaults to the platform shell."},
            "login": {"type": "boolean", "description": "Use login-shell semantics."},
            "justification": {"type": "string", "description": "Reason when approval is required."},
            "prefix_rule": {"type": "array", "items": {"type": "string"}},
            "sandbox_permissions": {"type": "string", "enum": ["use_default", "require_escalated"]}
        },
        "required": ["cmd"],
        "additionalProperties": true
    })
}
