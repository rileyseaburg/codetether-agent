//! One concurrency step across input, events, and prompt completion.

use crate::session::thread_events::ThreadEventMapper;

use super::prompt_run::{PromptOutcome, PromptRun};
use super::socket::{SocketSink, SocketStream};
use super::{turn_event, turn_input, wire};

/// State transition selected from concurrent turn activity.
pub(super) enum TurnStep {
    Continue,
    Abort,
    Finished(PromptOutcome),
}

/// Wait for and classify the next active-turn signal.
pub(super) async fn next(
    sink: &mut SocketSink,
    stream: &mut SocketStream,
    session_id: &str,
    mapper: &mut ThreadEventMapper,
    run: &mut PromptRun,
    events_open: &mut bool,
) -> TurnStep {
    tokio::select! {
        input = wire::next(stream) => match input {
            Ok(Some(frame)) => match turn_input::handle(
                sink, session_id, frame,
            ).await {
                Ok(turn_input::InputAction::Continue) => TurnStep::Continue,
                Ok(turn_input::InputAction::Cancel) | Err(_) => {
                    TurnStep::Abort
                }
            },
            Ok(None) | Err(_) => TurnStep::Abort,
        },
        event = turn_event::next(
            sink, mapper, &mut run.events,
        ), if *events_open => match event {
            turn_event::EventAction::Continue => TurnStep::Continue,
            turn_event::EventAction::Closed => {
                *events_open = false;
                TurnStep::Continue
            }
            turn_event::EventAction::Failed => TurnStep::Abort,
        },
        outcome = &mut run.task => TurnStep::Finished(outcome),
    }
}
