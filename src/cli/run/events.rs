//! Live thread and JSONL event sink for `codetether run`.

#[path = "events_drain.rs"]
mod drain;
#[path = "events_finish.rs"]
mod finish;

use crate::session::SessionEvent;
use crate::session::thread_events::{ThreadEventContext, ThreadEventMapper};
use crate::session::thread_store::ThreadStore;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use tokio::task::JoinHandle;

type SharedMapper = Arc<Mutex<ThreadEventMapper>>;

pub(in crate::cli::run) struct LiveEvents {
    tx: Option<mpsc::Sender<SessionEvent>>,
    handle: JoinHandle<Result<()>>,
    mapper: SharedMapper,
    store: ThreadStore,
    jsonl: bool,
}

impl LiveEvents {
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

    pub fn sender(&self) -> mpsc::Sender<SessionEvent> {
        self.tx.as_ref().expect("live sender present").clone()
    }
}
