//! Execute relay rounds in the original predecessor and agent order.
use super::relay_types::{RelayProgress, RelayRoundContext};
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) async fn run_relay_rounds(
        ctx: RelayRoundContext<'_>,
    ) -> anyhow::Result<RelayProgress> {
        let mut progress = RelayProgress {
            baton: ctx.prompt.to_string(),
            previous_normalized: None,
            convergence_hits: 0,
            turns: 0,
        };
        'relay_loop: for round in 1..=ctx.max_rounds {
            for idx in 0..ctx.team.ordered_agents.len() {
                let to = ctx.team.ordered_agents[idx].clone();
                let from = Self::relay_sender(&ctx.team.ordered_agents, idx, round);
                progress.turns += 1;
                Self::announce_relay_turn(
                    ctx.relay,
                    &from,
                    &to,
                    &progress.baton,
                    ctx.ralph_tx,
                    ctx.story_id,
                    round,
                    ctx.max_rounds,
                )
                .await;
                let output = Self::run_relay_turn(
                    ctx.registry,
                    &mut ctx.team.sessions,
                    &progress.baton,
                    &to,
                    ctx.story_id,
                    ctx.ralph_tx,
                )
                .await?;
                if progress.observe(output, &to, ctx.prompt, ctx.story_id) {
                    break 'relay_loop;
                }
            }
        }
        Ok(progress)
    }
}
