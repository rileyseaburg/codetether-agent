//! In-process Candle inference runtime.
//!
//! Compiled only with the `candle` feature. Loading lives in `candle_new`,
//! model dispatch in `candle_model`, and generation in `candle_prefill` and
//! `candle_decode`.

#[path = "candle_arch.rs"]
mod candle_arch;
#[path = "candle_decode.rs"]
mod candle_decode;
#[path = "candle_decoded.rs"]
mod candle_decoded;
#[path = "candle_device.rs"]
mod candle_device;
#[path = "candle_encode.rs"]
mod candle_encode;
#[path = "candle_eos.rs"]
mod candle_eos;
#[path = "candle_gguf.rs"]
mod candle_gguf;
#[path = "candle_load.rs"]
mod candle_load;
#[path = "candle_model.rs"]
mod candle_model;
#[path = "candle_new.rs"]
mod candle_new;
#[path = "candle_output.rs"]
mod candle_output;
#[path = "candle_penalty.rs"]
mod candle_penalty;
#[path = "candle_prefill.rs"]
mod candle_prefill;
#[path = "candle_prefix.rs"]
mod candle_prefix;
#[path = "candle_prompt.rs"]
mod candle_prompt;
#[path = "candle_resolve.rs"]
mod candle_resolve;
#[path = "candle_runtime.rs"]
mod candle_runtime;
#[path = "candle_sampler.rs"]
mod candle_sampler;
#[path = "candle_sampling.rs"]
mod candle_sampling;
#[path = "candle_think.rs"]
mod candle_think;

pub(crate) use candle_runtime::CandleThinker;

use super::{CandleDevicePreference, ThinkerConfig, ThinkerOutput};
