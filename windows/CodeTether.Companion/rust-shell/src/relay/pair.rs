use super::{client, device::Device, error::Error, validation};
use codetether_companion_protocol::{PairReceipt, PairRequest};
use zeroize::{Zeroize, Zeroizing};

pub(super) fn exchange(code: &str) -> Result<Device, Error> {
    let agent = client::agent();
    let mut request = PairRequest {
        code: code.to_owned(),
    };
    let sent = client::send(agent.post(&format!("{}/pair", client::ORIGIN)), &request);
    request.code.zeroize();
    let response = sent?;
    let mut receipt: PairReceipt = client::decode(response)?;
    let checked = validation::receipt(&receipt);
    let token = Zeroizing::new(std::mem::take(&mut receipt.device_token));
    let expires_at = checked?;
    Ok(Device {
        agent: super::http::client()?,
        session_id: receipt.id,
        token,
        interval_seconds: receipt.interval_seconds,
        expires_at,
        replies: std::sync::Mutex::new(std::collections::HashSet::new()),
    })
}
