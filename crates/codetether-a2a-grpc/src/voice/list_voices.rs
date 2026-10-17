//! Voice transport list voices.
use super::*;
pub(super) async fn list_voices(
    _service: &VoiceServiceImpl,
    _request: Request<proto::ListVoicesRequest>,
) -> Result<Response<proto::ListVoicesResponse>, Status> {
    // Built-in voice profiles — matches the Python side's voice list
    let voices = profiles::profiles();

    Ok(Response::new(proto::ListVoicesResponse { voices }))
}
