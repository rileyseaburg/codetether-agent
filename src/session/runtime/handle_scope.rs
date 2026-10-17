//! Explicit propagation of parent controls through a spawned job's future.

use super::ControlHandle;
tokio::task_local! { static CURRENT: ControlHandle; }

impl ControlHandle {
    pub(crate) async fn scope<F: std::future::Future>(&self, future: F) -> F::Output {
        CURRENT.scope(self.clone(), future).await
    }

    pub(crate) fn current() -> Option<Self> {
        CURRENT.try_with(Clone::clone).ok()
    }

    pub(crate) fn spawn<F>(future: F) -> tokio::task::JoinHandle<F::Output>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let control = Self::current();
        tokio::spawn(async move {
            match control {
                Some(control) => control.scope(future).await,
                None => future.await,
            }
        })
    }
}
