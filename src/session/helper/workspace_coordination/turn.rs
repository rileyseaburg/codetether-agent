//! Prompt-lifetime renewal and deterministic release of mux leases.

#[path = "turn/release.rs"]
mod release;
#[path = "turn/renewal.rs"]
mod renewal;

pub(in crate::session::helper) struct LeaseTurn {
    owner: Option<String>,
    renewal: Option<tokio::task::JoinHandle<()>>,
}

impl LeaseTurn {
    pub(in crate::session::helper) fn open(owner: &str) -> Self {
        if !crate::mux::coordination::active() {
            return Self {
                owner: None,
                renewal: None,
            };
        }
        let owner = owner.to_string();
        let renewal = renewal::start(owner.clone());
        Self {
            owner: Some(owner),
            renewal: Some(renewal),
        }
    }

    pub(in crate::session::helper) async fn close(mut self) {
        self.abort_renewal();
        let Some(owner) = self.owner.clone() else {
            return;
        };
        release::run(&owner).await;
        self.owner = None;
    }

    fn abort_renewal(&mut self) {
        if let Some(renewal) = self.renewal.take() {
            renewal.abort();
        }
    }
}

impl Drop for LeaseTurn {
    fn drop(&mut self) {
        self.abort_renewal();
        let Some(owner) = self.owner.take() else {
            return;
        };
        release::spawn(owner);
    }
}

#[cfg(test)]
#[path = "turn/tests.rs"]
mod tests;
