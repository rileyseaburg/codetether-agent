# Gemini Web image generation (private protocol)

This Node client uses the private web protocol, not the paid Google image API.
It is separate from the Rust provider. Backend model identity is not attested.

## Run
The simpler prompt-only launcher is documented in [QUICKSTART.md](QUICKSTART.md).
For automatic cookie renewal and follow-up edits, see [SESSIONS-AND-REFINEMENT.md](SESSIONS-AND-REFINEMENT.md).
The diagnostic CLI remains available below.

```sh
node scripts/gemini-web-debug/generate-image.mjs \
  "Generate a cinematic image of a red sailboat on a mountain lake at sunrise" \
  ./new-image-output
```

Requires Node, HTTP/2-capable curl, and configured Vault CLI access. Cookies are
read from `secret/codetether/providers/gemini-web`, field `cookies`, as a Netscape
export with domain/path/secure/expiry information. Two-column cookie lists are
rejected for this feature. Never paste credentials into CLI arguments or code.
For an explicit file override, prefix the command with
`GEMINI_WEB_COOKIE_FILE=/secure/path/cookies.txt`. UTF-8 and Windows UTF-16LE
Netscape exports are supported. Vault remains the default when this is unset.
`image_session_not_authenticated` means the supplied cookies did not yield a signed-in app token.

The output directory must not exist; its parent must exist. Omit it for a new
timestamped directory in the working directory. Images and JSON evidence use
0600 permissions, with a 0700 directory. No raw RPC bodies, signed image URLs,
or credentials are saved. Exit 0 requires retrieved, structurally checked image
bytes; exit 2 records a safe failure. Partial output is retained, not overwritten.

## Protocol

- `rpc-client.mjs` exposes `generateImages(prompt)` alongside unchanged text
  `generate(model, prompt, expected)`. Image URLs returned by the API are private;
  feed them to `downloadImage`, never log them.
- `image-payload.mjs` builds a fresh 99-slot request and image-mode header using
  observed selector `e051ce1aa80aa576`. No opaque HAR session fields are replayed.
- `image-protocol.mjs` reads structured attachments from `wrb.fr` frames, not prose.
- `image-download.mjs` follows up to six HTTPS hops, including HTTP-200 plain-text
  URL bodies. Hosts and paths are allowlisted before each request. Account cookies
  are domain/path-scoped and sent only to the observed account-authenticated hop.
- Transfers have 20-second connect / 180-second total timeouts and a 32 MiB limit.
  There is no automatic generation retry. JPEG/PNG signatures and dimensions are
  checked before saving; this is not a full pixel decoder.

## Evidence and limits

Mocked local tests: `node --test scripts/gemini-web-debug/*.test.mjs`.
Static/local client against live Google HTTP: a fresh yellow-canoe image was
generated and downloaded by this CLI, exit 0, 1024x572 JPEG, 121028 bytes.
Evidence: `artifacts/gemini-fresh-cookie-image-20261024T045237Z/summary.json`.
Earlier cookie material was not accepted as signed in; a fresh user-provided
export resolved that. Private protocol fields may change, and cookies can expire.