//! Model discovery and error response wire types.
mod copilot_error_response;
pub(crate) use copilot_error_response::CopilotErrorResponse;
mod copilot_error_detail;
pub(crate) use copilot_error_detail::CopilotErrorDetail;
mod copilot_models_response;
pub(crate) use copilot_models_response::CopilotModelsResponse;
mod copilot_model_info;
pub(crate) use copilot_model_info::CopilotModelInfo;
mod copilot_model_policy;
pub(crate) use copilot_model_policy::CopilotModelPolicy;
mod copilot_model_capabilities;
pub(crate) use copilot_model_capabilities::CopilotModelCapabilities;
mod copilot_model_limits;
pub(crate) use copilot_model_limits::CopilotModelLimits;
mod copilot_model_supports;
pub(crate) use copilot_model_supports::CopilotModelSupports;
