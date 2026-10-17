//! Tests for the local speech-to-text catalog.

use super::{DecoderSupport, STT_MODELS, find, runnable};

#[test]
fn parakeet_asr_is_catalogued() {
    let model = find("nemotron-speech-streaming-en-0.6b").expect("entry present");
    assert_eq!(model.arch, "parakeet");
    assert_eq!(model.sample_rate, 16_000);
    assert!(model.streaming);
}

#[test]
fn parakeet_has_no_candle_decoder_yet() {
    // candle-transformers 0.11 ships whisper and encodec, not parakeet.
    let model = find("nemotron-speech-streaming-en-0.6b").expect("entry present");
    assert_eq!(model.support, DecoderSupport::MissingDecoder);
    assert!(
        !runnable().contains(&"nemotron-speech-streaming-en-0.6b"),
        "must not be advertised as runnable without a decoder"
    );
}

#[test]
fn unknown_ids_are_rejected() {
    assert!(find("whisper-large-v3").is_none());
    assert!(find("").is_none());
}

#[test]
fn every_entry_declares_a_sane_sample_rate() {
    for model in STT_MODELS {
        assert!(
            model.sample_rate == 16_000 || model.sample_rate == 8_000,
            "{} declares unsupported rate {}",
            model.id,
            model.sample_rate
        );
    }
}
