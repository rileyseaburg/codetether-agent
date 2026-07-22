# GPT-5.6 Sol TetherScript probe

`openai_codex_5_6_sol.tether` sends a streaming Responses request to the
private ChatGPT Codex backend. It is intended for protocol debugging with an
account you are authorized to use, not as a production provider integration.

## Hooks

- `request_body(prompt)` builds request JSON without network access.
- `call(access_token, account_id, prompt)` sends the authenticated request and
  returns the raw server-sent event stream.

The call sends `Authorization`, `chatgpt-account-id`, `Content-Type`, and
`version` headers. TetherScript manages `Host`, `User-Agent`, `Accept`,
`Connection`, and `Content-Length` itself.

## Invocation

Run it directly without putting credentials in command-line arguments:

```bash
export OPENAI_CODEX_ACCESS_TOKEN='<access-token>'
export OPENAI_CODEX_ACCOUNT_ID='<account-id>'
tetherscript run --interp \
  examples/tetherscript/openai_codex_5_6_sol.tether -- 'Explain this repository'
unset OPENAI_CODEX_ACCESS_TOKEN OPENAI_CODEX_ACCOUNT_ID
```

Or invoke its `call` hook through the CodeTether tool:

```json
{
  "path": "examples/tetherscript/openai_codex_5_6_sol.tether",
  "hook": "call",
  "args": ["<access-token>", "<account-id>", "Explain this repository"]
}
```

Arguments are ordered as OAuth access token, ChatGPT workspace/account ID,
and user prompt. Never commit real values or include them in screenshots.
Tool-call arguments may be retained by audit or tracing systems, so use this
probe only where that exposure is acceptable.

For ordinary CodeTether use, authenticate with `codetether auth codex` and
select `openai-codex/gpt-5.6-sol`; that path keeps credentials in the configured
secret backend and handles token refresh.

## Behavior and limitations

The request uses `gpt-5.6-sol`, medium reasoning, `store: false`, encrypted
reasoning inclusion, and streaming output. HTTP failures become plugin errors
containing the response status and body. The TetherScript HTTP client has a
15-second socket timeout and buffers the response, so long pauses can fail.

The `chatgpt.com/backend-api/codex` endpoint is not the public OpenAI API and
may change without notice. Public API integrations should instead call
`https://api.openai.com/v1/responses` with an OpenAI API key and omit the
ChatGPT account and private backend version headers.

## References

- [OpenAI GPT-5.6 guidance](https://developers.openai.com/api/docs/guides/model-guidance?model=gpt-5.6)
- [OpenAI Responses API](https://developers.openai.com/api/reference/resources/responses/methods/create)
- [OpenAI Codex trace capture](../tracing/openai_codex.md)
