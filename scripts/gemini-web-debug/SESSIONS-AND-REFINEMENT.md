# Session renewal and image refinement

## Cookies should not require an export for every request

The launcher now calls Google's `accounts.google.com/RotateCookies` protocol to
renew short-lived session cookies. Future expiry dates in a Netscape file do not
mean its session tokens are still accepted by Google.

Renewed credentials are cached under `~/.cache/codetether/gemini-web/` (or the
equivalent directory under `XDG_CACHE_HOME`). This cache is sensitive: directories
are 0700 and files 0600. File names are hashes, not raw credential values. The
source export and Vault are not modified. Do not publish or share this cache.

A cache less than one minute old is reused; a cached authentication failure gets
one forced renewal. A revoked Google session still needs a fresh login/export.
There are no automatic retries of image generation, which might duplicate work.

## Ask for improvements to the same image

```sh
./gemini-image "A red ceramic teapot beside a sunlit window"
./gemini-image --refine latest "Change only the teapot to cobalt blue"
./gemini-image --refine latest "Add a little steam from the spout"
```

`latest` finds the newest successful refinable output under `./gemini-images/`.
For custom output locations or branching from an older version, use
`--refine ./path/to/version`. Every revision writes a new folder.
When several terminals or agents generate images, `latest` is shared. Use the
explicit folder to pin your source; the result records and prints its parent.

Each successful version stores private conversation/response/candidate IDs in
`conversation.json`. These are not cookies or signed download URLs. The client
checks that a refinement remains in the same conversation. Earlier outputs that
lack this file cannot be continued; generate a new base image once. Arbitrary
image upload/editing is not implemented by this conversation feature.

## Evidence

Static/local client tests against live Google HTTP recovered the previously
failing export, then changed a red teapot to blue in the same conversation.
Original: `gemini-images/20261024T051205832Z/summary.json`.
Revision: `gemini-images/20261024T051230153Z/summary.json`.