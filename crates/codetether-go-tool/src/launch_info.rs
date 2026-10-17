//! Borrowed values used to render a launch acknowledgement.

pub(crate) struct LaunchInfo<'a> {
    pub(crate) okr_id_str: &'a str,
    pub(crate) task: &'a str,
    pub(crate) resolved_model: &'a str,
    pub(crate) max_iterations: usize,
    pub(crate) working_dir: &'a str,
    pub(crate) prd_filename: &'a str,
    pub(crate) progress_filename: &'a str,
}
