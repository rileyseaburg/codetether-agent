use super::{device::Device, error::Error, frame_input, http, wire};
use base64::{Engine, engine::general_purpose::STANDARD};
use chrono::{DateTime, SecondsFormat, Utc};
use codetether_companion_desktop::CapturedFrame;
use codetether_companion_protocol::{Accepted, CaptureTrigger};
use tokio_util::sync::CancellationToken;
use zeroize::Zeroizing;

impl Device {
    /// Uploads one bounded frame; callers must recheck local consent generations.
    pub(crate) async fn upload(
        &self,
        frame: &CapturedFrame,
        captured: DateTime<Utc>,
        trigger: CaptureTrigger,
        request_id: Option<String>,
        cancel: &CancellationToken,
    ) -> Result<(), Error> {
        self.live(cancel)?;
        frame_input::validate(frame, captured, trigger, request_id.as_deref())?;
        let mut image = Zeroizing::new(String::with_capacity(699_052));
        STANDARD.encode_string(frame.jpeg(), &mut image);
        let captured_at = captured.to_rfc3339_opts(SecondsFormat::Millis, true);
        let body = super::upload_body::UploadBody {
            image: &image,
            captured_at: &captured_at,
            trigger,
            request_id: request_id.as_deref(),
        };
        let payload = wire::body(&body)?;
        drop(image);
        self.live(cancel)?;
        let request = self
            .authorize(self.agent.post(self.path("frames")))?
            .header("Content-Type", "application/json")
            .body(payload);
        let accepted: Accepted = http::receive(request, 202, cancel).await?;
        self.live(cancel)?;
        if !accepted.accepted {
            return Err(Error::InvalidResponse);
        }
        Ok(())
    }
}
