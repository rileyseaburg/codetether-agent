pub(crate) fn reject(html: &str) -> Result<(), String> {
    let mut offset = 0;
    while let Some(hit) = html[offset..].find("<script") {
        let open_start = offset + hit;
        let Some(open_end_hit) = html[open_start..].find('>') else {
            return Ok(());
        };
        let open_end = open_start + open_end_hit;
        let open_tag = html[open_start..=open_end].to_ascii_lowercase();
        let body_start = open_end + 1;
        if has_src(&open_tag) {
            offset = body_start;
            continue;
        }
        let Some(close_hit) = html[body_start..].find("</script") else {
            return Ok(());
        };
        let close_start = body_start + close_hit;
        crate::js::reject_unsupported_syntax(&html[body_start..close_start])?;
        offset = close_start + "</script".len();
    }
    Ok(())
}

fn has_src(open_tag: &str) -> bool {
    open_tag.contains(" src=")
        || open_tag.contains("\tsrc=")
        || open_tag.contains("\nsrc=")
        || open_tag.contains("\rsrc=")
}
