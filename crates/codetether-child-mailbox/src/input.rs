//! Borrowed submission metadata retained through durable append.
use codetether_collaboration_input::MessageImage;
/// Images and parent context supplied by the owning application.
///
/// # Examples
/// ```
/// let input=codetether_child_mailbox::EnqueueInput{images:&[],parent_id:None,prior_context:None};
/// assert!(input.images.is_empty());
/// ```
pub struct EnqueueInput<'a> {
    /// Attachments to clone after loading the queue.
    pub images: &'a [MessageImage],
    /// Parent identity owning this submission.
    pub parent_id: Option<&'a str>,
    /// Parent context inheritance permission.
    pub prior_context: Option<bool>,
}
