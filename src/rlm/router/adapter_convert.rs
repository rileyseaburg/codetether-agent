//! Request/response conversion between main crate and RLM types.

mod request;
mod response;

pub(super) use request::build_req;
pub(super) use response::to_llm_resp;
