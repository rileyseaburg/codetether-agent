//! Borrowed application resources and input channels for one selection cycle.
use crate::{LoopIo, Timers};
use std::{path::Path, sync::Arc};

/// Resources shared by event selection and the ensuing dispatch.
///
/// Type parameters describe the reader, application, session slot, provider
/// registry, worker bridge, session runtime, event, notice and recorder.
/// The frontend can give its concrete specialization a local type alias.
///
/// # Examples
/// ```text
/// type SelectArgs<'a> = codetether_tui_reactor::Selection<
///     'a, Reader, App, Slot, Registry, Bridge, Runtime, Event, Notice, Recorder,
/// >;
/// ```
pub struct Selection<'a, R, A, S, P, B, H, E, N, D> {
    /// Terminal event stream.
    pub reader: &'a mut R,
    /// Mutable application state.
    pub app: &'a mut A,
    /// Active workspace directory.
    pub cwd: &'a Path,
    /// Active session slot.
    pub slot: &'a mut S,
    /// Optional provider registry shared with command handlers.
    pub registry: &'a Option<Arc<P>>,
    /// Optional worker bridge, mutable during event handling.
    pub worker_bridge: &'a mut Option<B>,
    /// Session execution handle.
    pub runtime: &'a H,
    /// Borrowed event, notice and shutdown channels.
    pub io: &'a mut LoopIo<'a, E, N>,
    /// UI, audit and watchdog timers.
    pub timers: &'a mut Timers,
    /// Shared bus event recorder.
    pub bus_recorder: &'a D,
}
