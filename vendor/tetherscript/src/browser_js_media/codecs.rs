pub(super) fn can_play_type(tag: &str, mime: &str) -> &'static str {
    let media_type = mime
        .split(';')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    match media_type.as_str() {
        "audio/mpeg" | "audio/mp3" | "audio/wav" | "audio/ogg" => audio(tag, "probably"),
        "video/mp4" | "video/webm" | "video/ogg" => video(tag, "probably"),
        other if other.starts_with("audio/") => audio(tag, "maybe"),
        other if other.starts_with("video/") => video(tag, "maybe"),
        _ => "",
    }
}

fn audio(tag: &str, answer: &'static str) -> &'static str {
    if tag == "audio" {
        answer
    } else {
        ""
    }
}

fn video(tag: &str, answer: &'static str) -> &'static str {
    if tag == "video" {
        answer
    } else {
        ""
    }
}
