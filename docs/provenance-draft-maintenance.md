# Maintaining the agent provenance draft

Edit [the Markdown draft](draft-seaburg-codetether-agent-provenance-00.md)
for future revisions. The [original 18-page PDF](../draft-seaburg-codetether-agent-provenance-00.pdf)
is the unchanged source snapshot, not an automatically regenerated artifact.

## Conversion scope

- Preserved the title, author, draft identifier, metadata, abstract, all main
  sections, three appendices, acknowledgments, and reference identifiers.
- Restored heading levels and section numbering, definition lists, nested lists,
  fenced policy pseudocode, the component diagram, and the example JWT.
- Removed PDF page breaks and joined wrapped prose and split identifiers.
- Expanded the literal `{::boilerplate bcp14-tagged}` directive into the standard
  BCP 14 requirements-language paragraph; converted `{{reference}}` placeholders
  into Markdown reference links. This is ordinary Markdown, not an RFC publishing
  toolchain source file.
- Kept the original normative/informative reference groupings. RFC 7800 and
  RFC 8785 are cited in the body but absent from the PDF's reference metadata;
  they have a separate reference group pending an editorial decision.
- Repaired diagram alignment and formatted JSON without changing its values.
  Protocol requirements and operational assertions remain those of the draft,
  not claims independently verified during conversion.

## Existing issues to resolve in a substantive revision

- Sections 2, 4.1, and 5.1 describe monotonic session taint, while Section 5.2
  permits ephemeral turn-scoped taint. Clarify how these requirements compose.
- Section 6.1 compares `max_depth` to current depth, while Section 6.2 decrements
  it at each hop. Clarify whether it is an absolute limit or a remaining budget.
- Section 6.1 introduces `parent_session_id`, but Section 4.1 does not define it.
- Section 8.3 requires verification of `ctp_tool_manifest_hash`, but Section 8.2
  omits that field from the attestation quote format.
- Appendix B introduces `ctp_revocation`, but Section 11.1 omits it from the
  requested JWT claim registrations.
- Appendix C retains the original abbreviated identifiers, hash placeholders,
  and numeric `iat`/`exp` values alongside 2026 timestamps. It is an illustrative
  JSON example, not a usable or cryptographically verified token.

## Editing conventions

Keep claim names, requirement keywords, and reference identifiers exact.
Update numbered headings and section citations together. Preserve code fences
around the diagram, policy predicate, and JSON; keep the JSON parseable.
Review protocol changes separately from transcription corrections. PDF/RFC export is not configured.

## Transcription checks (static/local)

From the repository root, with Poppler's `pdftotext` and Node.js installed:

```sh
pdftotext -layout draft-seaburg-codetether-agent-provenance-00.pdf docs/provenance-audit/source.layout.txt
node docs/provenance-audit/run.cjs
```

The [audit report](provenance-audit/validation.json) records a comparison of the
body's alphanumeric text (excluding the reformatted diagram and expanded BCP 14
boilerplate), exact parsed JSON values, reference definitions, section/appendix
counts, and code fences. The PDF extraction is retained beside the report.
This checks transcription fidelity, not protocol correctness, external link
availability, or rendered appearance. Substantive revisions will intentionally
diverge from this snapshot; do not treat this as a permanent specification test.

The optional `provenance-audit/run.tether` records the attempted TetherScript
execution path. With TetherScript 0.1.0-alpha.32, its embedded JS engine failed
with `JSON.parse: unexpected byte 0x65 at byte 0`; use the Node.js command above
for the comparison. This runtime limitation does not affect the Markdown.