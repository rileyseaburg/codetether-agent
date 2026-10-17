//! tool call dedup::tests transform rules.

use super::*;

use codetether_provider_core::Role;
mod call;
use call::call;
mod different_names_not_merged;
mod recent_calls_preserved;
mod small_args_preserved;
