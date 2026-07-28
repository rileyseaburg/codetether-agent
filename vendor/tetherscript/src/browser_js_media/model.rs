use super::error::MediaError;
use super::*;

#[derive(Clone)]
pub(super) struct MediaState {
    pub(super) src: String,
    pub(super) current_src: String,
    pub(super) duration: f64,
    pub(super) current_time: f64,
    pub(super) paused: bool,
    pub(super) ended: bool,
    pub(super) ready_state: u16,
    pub(super) muted: bool,
    pub(super) volume: f64,
    pub(super) playback_rate: f64,
    pub(super) error: Option<MediaError>,
}

impl Default for MediaState {
    fn default() -> Self {
        Self {
            src: String::new(),
            current_src: String::new(),
            duration: 0.0,
            current_time: 0.0,
            paused: true,
            ended: false,
            ready_state: 0,
            muted: false,
            volume: 1.0,
            playback_rate: 1.0,
            error: None,
        }
    }
}

impl MediaState {
    pub(super) fn from_attrs(attrs: &HashMap<String, String>) -> Self {
        Self {
            src: attrs.get("src").cloned().unwrap_or_default(),
            duration: attr_number(attrs, "duration")
                .or_else(|| attr_number(attrs, "data-duration"))
                .unwrap_or(0.0),
            muted: attrs.contains_key("muted"),
            volume: attr_number(attrs, "volume").unwrap_or(1.0).clamp(0.0, 1.0),
            playback_rate: attr_number(attrs, "playbackrate").unwrap_or(1.0),
            ..Self::default()
        }
    }
}

fn attr_number(attrs: &HashMap<String, String>, name: &str) -> Option<f64> {
    attrs.get(name).and_then(|value| value.parse::<f64>().ok())
}
