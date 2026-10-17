// Standard JSON-RPC error codes
#[allow(dead_code)]
pub const PARSE_ERROR: i32 = -32700;

#[allow(dead_code)]
pub const INVALID_REQUEST: i32 = -32600;

pub const METHOD_NOT_FOUND: i32 = -32601;

pub const INVALID_PARAMS: i32 = -32602;

pub const INTERNAL_ERROR: i32 = -32603;

// A2A-specific error codes
pub const TASK_NOT_FOUND: i32 = -32001;

pub const TASK_NOT_CANCELABLE: i32 = -32002;

#[allow(dead_code)]
pub const PUSH_NOT_SUPPORTED: i32 = -32003;

pub const UNSUPPORTED_OPERATION: i32 = -32004;

#[allow(dead_code)]
pub const CONTENT_TYPE_NOT_SUPPORTED: i32 = -32005;
