use super::args::EditArgs;
use anyhow::Result;

pub async fn apply(content: &str, args: &EditArgs<'_>) -> Result<Option<String>> {
    let enabled = crate::tool::morph_backend::should_use_morph_backend();
    if !enabled || args.instruction.is_none() && args.update.is_none() {
        return Ok(None);
    }
    let instruction =
        args.instruction
            .map(str::to_string)
            .or_else(|| {
                args.old_string.zip(args.new_string).map(|(old, new)| {
            format!("Replace the target snippet once.\nOld snippet:\n{old}\n\nNew snippet:\n{new}")
        })
            })
            .unwrap_or_else(|| "Apply the requested update precisely.".to_string());
    let update = args
        .update
        .map(str::to_string)
        .or_else(|| {
            args.old_string.zip(args.new_string).map(|(old, new)| {
                format!("// Replace this snippet:\n{old}\n// With this snippet:\n{new}")
            })
        })
        .unwrap_or_else(|| "// ...existing code...".to_string());
    match crate::tool::morph_backend::apply_edit_with_morph(content, &instruction, &update).await {
        Ok(updated) => Ok(Some(updated)),
        Err(error) if args.has_pair() => {
            tracing::warn!(path = %args.path, error = %error, "Morph edit failed; using string matcher");
            Ok(None)
        }
        Err(error) => Err(error),
    }
}
