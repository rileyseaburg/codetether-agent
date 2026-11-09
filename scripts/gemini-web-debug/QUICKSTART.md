# Easy Gemini images

From the repository root:

```sh
./gemini-image "A watercolor cabin beside a turquoise mountain lake"
```

Or run it without arguments and type your description when asked:

```sh
./gemini-image
```

The launcher:
- Finds the cookie export and renews session cookies into a private local cache.
- Creates a fresh dated folder under `./gemini-images/` (ignored by Git).
- Prints the saved image path instead of diagnostic JSON.
- Keeps full credential-free diagnostics beside each image.

Cookie selection, in order:

1. `--cookies /path/to/cookies.txt`
2. `GEMINI_WEB_COOKIE_FILE`
3. `~/NAS/wireshark/cookie.txt`, when present (the existing local setup)
4. Configured Vault access

Optional overrides:

```sh
./gemini-image "A yellow canoe" --cookies /secure/cookies.txt --out ./new-output
```

To improve the latest image created with this version:
```sh
./gemini-image --refine latest "Make the lighting warmer and the background simpler"
```
Each revision gets its own folder; the original is untouched. Use `--refine ./old-folder`
to branch from a particular version. See [SESSIONS-AND-REFINEMENT.md](SESSIONS-AND-REFINEMENT.md).
The explicit output folder must not exist. Keep cookie exports and the local cache private.
For the lower-level API and protocol, see [IMAGE-GENERATION.md](IMAGE-GENERATION.md).

Static/local client test against live Google HTTP: the interactive launcher exited
0 and generated a fresh 1024x572 watercolor image. Evidence is in
`gemini-images/20261024T045823615Z/summary.json`. This convenience wrapper does
not turn the undocumented web protocol into a supported production API.