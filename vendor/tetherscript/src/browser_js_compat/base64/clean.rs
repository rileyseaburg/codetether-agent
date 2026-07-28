use super::alphabet;

pub(super) fn input(input: &str) -> Result<Vec<char>, String> {
    let mut chars: Vec<char> = input
        .chars()
        .filter(|ch| !alphabet::is_ascii_ws(*ch))
        .collect();
    match chars.len() % 4 {
        1 => return Err("atob: invalid base64 length".into()),
        0 => strip_padding(&mut chars)?,
        _ if chars.contains(&'=') => {
            return Err("atob: invalid base64 padding".into());
        }
        _ => {}
    }
    validate_alphabet(&chars)?;
    Ok(chars)
}

fn strip_padding(chars: &mut Vec<char>) -> Result<(), String> {
    let padding = chars.iter().rev().take_while(|ch| **ch == '=').count();
    if padding > 2 {
        return Err("atob: invalid base64 padding".into());
    }
    chars.truncate(chars.len() - padding);
    if chars.contains(&'=') {
        return Err("atob: padding is only allowed at the end".into());
    }
    Ok(())
}

fn validate_alphabet(chars: &[char]) -> Result<(), String> {
    for ch in chars {
        if alphabet::value(*ch).is_none() {
            return Err(format!("atob: invalid base64 character {ch:?}"));
        }
    }
    Ok(())
}
