//! Bus-backed VoiceService transport.
//!
//! [VoiceServiceImpl] exposes voice lifecycle operations and event streams.
//! The host supplies its shared bus and mounts the generated tonic service.
use crate::proto;
use crate::proto::voice_service_server::{VoiceService, VoiceServiceServer};
use codetether_bus_core::{AgentBus, BusEnvelope, BusMessage};
use std::{pin::Pin, sync::Arc};
use tokio_stream::Stream;
use tonic::{Request, Response, Status};
type StreamResult<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

/// VoiceService gRPC implementation backed by the AgentBus.
pub struct VoiceServiceImpl {
    bus: Arc<AgentBus>,
}
mod construct;
mod create_voice_session;
mod delete_voice_session;
mod events_end;
mod events_state;
mod events_transcript;
mod get_voice_session;
mod list_voices;
mod profiles;
mod stream;
mod stream_voice_events;
macro_rules! implement_service {
    ($($method:ident: $request:ty => $response:ty,)*) => {
        #[tonic::async_trait]
        impl VoiceService for VoiceServiceImpl {
            type StreamVoiceEventsStream = StreamResult<proto::VoiceEvent>;
            $(async fn $method(&self, request: Request<$request>) -> Result<Response<$response>, Status> {
                $method::$method(self, request).await
            })*
        }
    };
}
mod adapter;
