//! Redacted history-sink projection for metadata debug output.

use super::super::SessionMetadata;

pub(super) fn history_sink(metadata: &SessionMetadata) -> Option<(&str, &str, &str)> {
    metadata.history_sink.as_ref().map(|config| {
        (
            config.endpoint.as_str(),
            config.bucket.as_str(),
            config.prefix.as_str(),
        )
    })
}
