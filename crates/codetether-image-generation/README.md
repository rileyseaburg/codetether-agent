# Image generation in CodeTether Agent

The `image_gen` tool (alias `imagegen`) supports the existing OpenAI backend and
a native Rust Gemini Web backend. No Node process is used by the Rust backend.

## Gemini Web

Ask the agent to generate an image with Gemini Web, or supply these tool arguments:

```json
{"provider":"gemini-web","prompt":"A watercolor cabin beside an alpine lake"}
```

The result includes the image preview, `saved_path`, `generated_image_paths`,
and, when available, `refinement_path`. Improve that exact image with:

```json
{"provider":"gemini-web","prompt":"Make the lighting warmer, keeping the composition","refine_from":"/path/from/previous/refinement_path"}
```

`refine_from` accepts a previous image path, output folder, or `conversation.json`.
The standalone Node launcher's conversation files are compatible. Old outputs
without conversation state cannot be continued. Every version gets a fresh
directory; no original image is overwritten. Gemini Web does not support the
arbitrary-upload selectors `referenced_image_paths` or `num_last_images_to_include`.

## Credentials and storage

Configure Vault provider `gemini-web` with a Netscape cookie export in `cookies`.
For local development, explicitly set `GEMINI_WEB_COOKIE_FILE` to a private export
file. UTF-8 and UTF-16LE exports are accepted. Hardened deployments with
`CODETETHER_DISABLE_ENV_FALLBACK=1` only use Vault.

The transport renews session cookies and caches them privately under the
CodeTether data directory. The original export and Vault are not modified.
With an explicit local cookie file, a matching private renewal cache from the
Node launcher (`$XDG_CACHE_HOME/codetether/gemini-web`, defaulting to
`~/.cache/codetether/gemini-web`) can seed a new Rust session. Migration is
read-only and disabled in Vault-only mode; subsequent renewal is native Rust.
Generated images and continuation files are private local artifacts. Do not
publish cookie caches, exports, or conversation state.

## Compatibility and testing

Calls without `provider` retain the OpenAI backend and existing editing behavior.
Gemini uses Google's undocumented web protocol, not the supported paid image API.
Focused tests: `cargo test -p codetether-image-generation`.
The ignored `gemini_web_live` integration test performs real generation and
refinement only when explicitly run with signed-in credentials.

See [operational limits and live evidence](GEMINI-LIMITS.md).