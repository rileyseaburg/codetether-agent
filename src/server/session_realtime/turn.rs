//! Concurrent event, result, and steering loop for one prompt turn.

use super::socket::{SocketSink, SocketStream};
use super::{turn_finish, turn_start, turn_step};

/// Run one prompt while keeping its WebSocket input live for steering.
pub(super) async fn run(
    mut sink: SocketSink,
    mut stream: SocketStream,
    session_id: String,
    message: String,
) {
    let Some((mut mapper, mut run)) = turn_start::start(&mut sink, &session_id, message).await
    else {
        return;
    };
    let mut events_open = true;
    loop {
        match turn_step::next(
            &mut sink,
            &mut stream,
            &session_id,
            &mut mapper,
            &mut run,
            &mut events_open,
        )
        .await
        {
            turn_step::TurnStep::Continue => {}
            turn_step::TurnStep::Abort => {
                run.task.abort();
                return;
            }
            turn_step::TurnStep::Finished(outcome) => {
                let _ = turn_finish::finish(&mut sink, &mut mapper, &mut run.events, outcome).await;
                return;
            }
        }
    }
}
