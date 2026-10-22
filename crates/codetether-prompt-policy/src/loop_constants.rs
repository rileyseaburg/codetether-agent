//! Loop-control constants shared by the agentic prompt loops.
//!
//! The session's `prompt*` methods run an agentic loop that repeatedly calls
//! the LLM and executes any tool calls it emits. The constants here tune the
//! guardrails that detect and break pathological loops (e.g. the model keeps
//! running the same tool, keeps promising to call a tool without ever doing
//! it, etc.).

/// Reminder sent when the build-mode agent responds with a plan-only answer
/// despite the user asking for a file change.
pub const BUILD_MODE_TOOL_FIRST_NUDGE: &str = "Build mode policy reminder: execute directly. \
Start by calling at least one appropriate tool now (or emit <tool_call> markup for non-native \
tool providers). Do not ask for permission and do not provide a plan-only response.";

/// Maximum number of "tool-first" retries before we surface an error to the
/// caller instead of nudging again.
pub const BUILD_MODE_TOOL_FIRST_MAX_RETRIES: u8 = 2;

/// Maximum number of retries when the model claims it is about to use a tool
/// but never actually emits a tool call.
pub const NATIVE_TOOL_PROMISE_RETRY_MAX_RETRIES: u8 = 1;

/// Threshold of consecutive codesearch "no matches" results before we nudge
/// the model away from brute-force variant retries.
pub const MAX_CONSECUTIVE_CODESEARCH_NO_MATCHES: u32 = 5;

/// Maximum retries for post-edit validation (build/lint/test) failures before
/// we give up and surface the report to the caller.
pub const POST_EDIT_VALIDATION_MAX_RETRIES: u8 = 3;

/// Repeated returned observations before offering nonterminal recovery guidance.
pub const MAX_STEPS_WITHOUT_PROGRESS: u32 = 15;

/// Nudge sent when the model keeps trying punctuation/casing variants of the
/// same identifier via codesearch.
pub const CODESEARCH_THRASH_NUDGE: &str = "Stop brute-force codesearch variant retries. \
You already got repeated \"No matches found\" results. Do not try punctuation/casing/underscore \
variants of the same token again. Either switch to a broader strategy (e.g., inspect likely files \
directly) or conclude the identifier is absent and continue with the best available evidence.";

/// Nudge sent when the model describes an upcoming tool call in prose instead
/// of actually emitting one.
pub const NATIVE_TOOL_PROMISE_NUDGE: &str = "You said you would use tools. Do not describe the tool \
call or promise a next step. Emit the actual tool call now. If native tool calling fails, emit a \
<tool_call> JSON block immediately instead of prose.";

/// Compatibility guidance for repeated evidence; read-only investigation is valid.
pub const NO_PROGRESS_NUDGE: &str = "Recent tools returned repeated evidence. Investigate the next uncertainty or adjust the approach. Legitimate waiting and read-only investigation may continue; this guidance is not a reason to end the task.";
