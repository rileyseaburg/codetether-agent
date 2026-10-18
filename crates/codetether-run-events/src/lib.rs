//! Live thread and JSONL event sink for non-interactive runs.
//!
//! [LiveEvents] owns the channel, mapping task and durable store for one turn.
//! Obtain its [sender](LiveEvents::sender), drop every sender after prompting,
//! then consume the sink with a success or failure completion.

mod drain;
mod finish;

use anyhow::Result;
use codetether_session_events::SessionEvent;
use codetether_thread_events::{ThreadEventContext, ThreadEventMapper};
use codetether_thread_store::ThreadStore;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use tokio::task::JoinHandle;

type SharedMapper = Arc<Mutex<ThreadEventMapper>>;

/// Capture one run's session events and persist its thread history.
///
/// `S` is the snapshot payload; mapping does not depend on native sessions.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let sink = codetether_run_events::LiveEvents::<()>::start("session-1", "hello", false).await?;
/// let sender = sink.sender();
/// sender.send(codetether_session_events::SessionEvent::TextChunk("hello".into())).await?;
/// drop(sender);
/// sink.finish_success("hello").await?;
/// # Ok(()) }
/// ```
pub struct LiveEvents<S> {
    tx: Option<mpsc::Sender<SessionEvent<S>>>,
    handle: JoinHandle<Result<()>>,
    mapper: SharedMapper,
    store: ThreadStore,
    jsonl: bool,
}

impl<S: Send + 'static> LiveEvents<S> {
    /// Append the turn-start record, then spawn its event drain task.
    /// # Arguments
    /// * `session_id` - Session owning the turn.
    /// * `message` - Initial user message.
    /// * `jsonl` - Also write mapped events to standard output.
    /// # Returns
    /// A sink with a bounded channel of 256 events.
    /// # Errors
    /// Returns configuration, storage or initial JSONL output errors.
    pub async fn start(session_id: &str, message: &str, jsonl: bool) -> Result<Self> {
        let store = ThreadStore::from_config()?;
        let mapper = Arc::new(Mutex::new(ThreadEventMapper::new(
            ThreadEventContext::for_session(session_id),
        )));
        let event = mapper.lock().await.turn_started(message);
        drain::emit(&store, &event, jsonl).await?;
        let (tx, rx) = mpsc::channel(256);
        let handle = tokio::spawn(drain::run(rx, Arc::clone(&mapper), store.clone(), jsonl));
        Ok(Self {
            tx: Some(tx),
            handle,
            mapper,
            store,
            jsonl,
        })
    }

    /// Clone the event sender; drop every clone before finishing the sink.
    /// # Panics
    /// Panics if its internal sender is absent; finishing consumes the sink.
    pub fn sender(&self) -> mpsc::Sender<SessionEvent<S>> {
        self.tx.as_ref().expect("live sender present").clone()
    }
}
