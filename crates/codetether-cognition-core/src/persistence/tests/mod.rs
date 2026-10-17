//! Persisted schema and evidence-reference regression cases.
use super::references::referenced_event_ids;
use super::state::SCHEMA_VERSION;
use super::*;
use crate::GlobalWorkspace;
use chrono::Utc;
use std::collections::HashMap;
mod referenced_events_collected;
mod round_trip_serialize;
