#[path = "skip/comment.rs"]
mod comment_mod;
#[path = "skip/string.rs"]
mod string_mod;
#[path = "skip/template.rs"]
mod template_mod;

pub(super) use comment_mod::comment;
pub(super) use string_mod::string;
pub(super) use template_mod::template;
