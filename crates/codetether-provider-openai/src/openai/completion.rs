//! Completion request construction, dispatch, and response conversion.

use super::*;
mod body;
mod content;
mod finish;
mod request;
mod response;
mod send;
use content::completion_content;
use finish::completion_finish;
use response::completion_response;
