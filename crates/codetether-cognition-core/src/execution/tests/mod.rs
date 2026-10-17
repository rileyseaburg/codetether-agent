//! Existing lease and validation regression tests.
use super::requests::validate_tool_args;
use super::*;
mod capability_lease_creation_and_validity;
mod validate_rejects_unlisted_tools;
mod validate_tool_args_checks_required_fields;
mod validate_tool_args_checks_types;
