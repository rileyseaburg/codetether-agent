# Context reliability completion audit

Status: complete. All seven requirements have implementation and verification
evidence. The final regression run passed 424 unit/integration tests and 381 doc
examples; the combined native scenario passed separately (425 tests total).
One optional integration test and 24 pre-existing doc examples remain ignored.

This audit retains all seven requirements of the original goal. The working tree
also contains unrelated concurrent changes; this report covers the context,
checkpoint, progress, summary, journal and request paths below.

| Requirement | Implemented behavior | Verification evidence |
| --- | --- | --- |
| 1. Durable constraints and corrections | Full source text, role, absolute index and hash survive window eviction. Exact partial replacements retain unrelated requirements. Historical pin edits use bounded indexed reads and atomic page/metadata writes. Forks rebase retained source links. | `context-reliability-constraints`, `partial-corrections`, `fork-corrections`, `indexed-pins` artifacts; native combined scenario. |
| 2. Restart checkpoints | Canonical snapshot folding distinguishes attempts, unknown results, explicit success and failure. Complete user evidence, uncertainty and next actions survive reloads. Metadata-first publication and clear tombstones prevent stale sidecar resurrection. | `context-reliability-checkpoints`, `progress-evidence` artifacts; run-resume regressions and native combined checkpoint save/reload/clear. |
| 3. Evidence-based continuation | Persistent bounded observations use actual tool results. Calls or prose alone are not completed work. Failed identical edits receive targeted guards; new evidence or human direction permits investigation. Reads and legitimate waits do not force task termination. | `context-reliability-progress-evidence`, `request-layout` artifacts; repeat-guard tests and combined model-switch/restart scenario. |
| 4. Stable request identity | Foreground and worker builders share stable instruction/history assembly. Runtime accounting is refreshed after preparation and placed outside stable system policy. Repeated composition replaces snapshots. | `context-reliability-request-layout` artifacts; actual consecutive request construction and Codex, OpenAI chat, Anthropic, Bedrock and Google encoder tests. |
| 5. Summary provenance and invalidation | Complete source-message digests, summarizer policy, producer and requested model identify summary provenance. Invalid or unprovenanced summaries cannot enter projections. Indexed writes preserve unaffected prefix summaries through persistence. | `context-reliability-summaries` artifacts; tracked-vector regressions and combined source-edit/reload validation. |
| 6. Efficient journal governance | Cached folds replay appended bytes, retain strict corruption errors and preserve holds/accounting. Bounded cached file handles prevent deleted/recreated inode identity reuse. Replacement races return coherent snapshots. | Initial journal byte-count tests, goal runtime gates, reproduced inode failure and fixed replay/race logs in `context-reliability-combined`. |
| 7. Reproducible long-horizon evaluation | An offline native scenario combines three compaction/restart phases, successive corrections, failed and pending tools, human redirection, model switching, checkpoint recovery, summary invalidation, provider-outage fallback and consecutive request encoding. | `long_horizon_reliability` integration test and `context-reliability-combined/scenario.json`; final regression log. |

Artifact directories above are under `artifacts/`; source-hash manifests and raw
logs retain initial failures separately from successful replacements. Earlier
increment details and source locations are in `context-reliability-progress.md`.

The scenario uses a provider stub that rejected four completion attempts without
sending network requests. It measures
state fidelity and request structure, not live model answer quality. Model names
are routing fixtures. Provider cache-hit rate, latency improvement and monetary
savings remain unmeasured; fixture token counters are synthetic journal inputs.
No live-provider traffic, deployment or publication is part of this evaluation.
Full build/type coverage and optional ignored fixtures remain with their normal
CI or explicitly configured execution paths.

The evaluation exposed and retained evidence for two production fixes: cached
journal identities could be confused by immediate inode reuse, and indexed
message edits dirtied the entire loaded window. Both have regression coverage.
An intermediate test also used a noncanonical temporary locator and was stopped
after it reached the workspace's legacy store; no schema upgrade committed.
The final fixture asserts its isolated ID-matched locator before context lookup.

Final logs, scenario results, source fingerprints and the requirement mapping
are in `artifacts/context-reliability-final/` and
`artifacts/context-reliability-combined/`.
