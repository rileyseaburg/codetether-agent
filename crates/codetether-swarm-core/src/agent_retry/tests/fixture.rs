use super::*;
pub(super) struct Attempt {
    pub(super) count: AtomicUsize,
    complete_on: Option<usize>,
    error: bool,
}
impl Attempt {
    pub(super) fn new(complete_on: Option<usize>, error: bool) -> Self {
        Self {
            count: AtomicUsize::new(0),
            complete_on,
            error,
        }
    }
}
#[async_trait]
impl AgentAttempt for Attempt {
    async fn execute(&self) -> AgentOutcome {
        let call = self.count.fetch_add(1, Ordering::SeqCst);
        if self.error {
            return Err(anyhow::anyhow!("provider failed"));
        }
        let exit = if self.complete_on == Some(call) {
            AgentLoopExit::Completed
        } else {
            AgentLoopExit::TimedOut
        };
        Ok((String::new(), 1, 0, exit))
    }
}
