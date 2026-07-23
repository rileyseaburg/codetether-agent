//! Live first-party collaborator context for model requests.

use std::fmt::Write;

const LIMIT: usize = 12;

pub(crate) fn append(mut prompt: String) -> String {
    prompt.push_str(
        "\n\n## Live first-party LAN collaboration\n\n\
         CodeTether owns peer discovery and authentication. Before inspecting another \
         repository or service over the network, use `agent` with `list`, then `message` \
         the relevant peer. Do not run dns-sd, browse generic mDNS service types, scan \
         ports, or curl endpoints to locate a CodeTether collaborator.\n\n",
    );
    let local_name = super::local_identity::current_name();
    append_identity(&mut prompt, local_name.as_deref());
    let mut peers = super::peer_route::list();
    peers.retain(|(name, _)| local_name.as_deref() != Some(name));
    peers.sort_by(|left, right| left.0.cmp(&right.0));
    if peers.is_empty() {
        prompt.push_str("- No live peers yet; `agent list` can observe arrivals.\n");
        return prompt;
    }
    let omitted = peers.len().saturating_sub(LIMIT);
    for (name, route) in peers.into_iter().take(LIMIT) {
        let skills = route.skills.join(", ");
        let _ = writeln!(
            prompt,
            "- @{name}: {} [skills: {skills}]",
            route.description
        );
    }
    if omitted > 0 {
        let _ = writeln!(prompt, "- {omitted} more live peers; use `agent list`.");
    }
    prompt
}

fn append_identity(prompt: &mut String, name: Option<&str>) {
    if let Some(name) = name {
        let _ = writeln!(prompt, "- Your LAN peer name is @{name} (this process).\n");
    } else {
        prompt.push_str("- This process has no active LAN peer endpoint.\n\n");
    }
}

#[cfg(test)]
#[path = "peer_prompt_tests.rs"]
mod tests;
