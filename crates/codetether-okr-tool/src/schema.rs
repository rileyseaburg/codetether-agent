//! schema for OKR tool dispatch.
use super::*;

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "enum": [
                    "create_okr", "get_okr", "update_okr", "delete_okr",
                    "list_okrs", "query_okrs",
                    "create_run", "get_run", "update_run", "delete_run",
                    "list_runs", "query_runs",
                    "stats"
                ],
                "description": "Action to perform"
            },
            "id": {"type": "string", "description": "UUID of the OKR or run"},
            "title": {"type": "string", "description": "OKR title (for create/update)"},
            "description": {"type": "string", "description": "OKR description"},
            "status": {
                "type": "string",
                "description": "Status filter or new status. OKR: draft/active/completed/cancelled/on_hold. Run: draft/pending_approval/approved/running/paused/waiting_approval/completed/failed/denied/cancelled"
            },
            "owner": {"type": "string", "description": "Owner filter or assignment"},
            "tenant_id": {"type": "string", "description": "Tenant ID filter or assignment"},
            "key_results": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "title": {"type": "string"},
                        "target_value": {"type": "number"},
                        "unit": {"type": "string", "default": "%"}
                    },
                    "required": ["title", "target_value"]
                },
                "description": "Key results to add (for create_okr)"
            },
            "okr_id": {"type": "string", "description": "Parent OKR UUID (for run operations)"},
            "name": {"type": "string", "description": "Run name (for create_run)"},
            "correlation_id": {"type": "string", "description": "Correlation ID for run queries"},
            "session_id": {"type": "string", "description": "Session ID for run queries"},
            "checkpoint_id": {"type": "string", "description": "Relay checkpoint ID for run queries"}
        },
        "required": ["action"]
    })
}
