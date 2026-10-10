#!/bin/bash
# Publish release assets via the Forgejo API directly (no tea).
# Replaces forgejo-release@de47e39 (tea 0.9.0), which went silent for 60min
# and died at the job deadline in runs 11515/11516/11517. curl + API is the
# path proven live: 91MB -> HTTP 201 in <1s over the LAN.
# Env: RELEASE_URL, TOKEN, REPOSITORY, TAG, SHA, NOTES_FILE, DIST_DIR, VERSION
set -euo pipefail
: "${RELEASE_URL:?}" "${TOKEN:?}" "${REPOSITORY:?}" "${TAG:?}" "${SHA:?}" "${NOTES_FILE:?}" "${DIST_DIR:?}" "${VERSION:?}"

# Publish-job LXCs are minimal; the old action installed jq/curl on demand.
# NEEDRESTART guards mirror release-setup.sh: an interactive apt/needrestart
# prompt hung run 85 for its whole timeout.
if ! command -v jq >/dev/null || ! command -v curl >/dev/null; then
    export DEBIAN_FRONTEND=noninteractive NEEDRESTART_MODE=a NEEDRESTART_SUSPEND=1
    apt-get -qq update && apt-get install -y -qq --no-install-recommends jq curl
fi

api() {
    local method=$1 path=$2
    shift 2
    curl --fail -sS -X "$method" -H "Authorization: token $TOKEN" "$@" \
        "$RELEASE_URL/api/v1/repos/$REPOSITORY/$path"
}

# Idempotent: remove any stale release/tag from a prior failed run, then pin
# the tag to the build SHA explicitly so the release cannot drift from it.
api DELETE "releases/tags/$TAG" >/dev/null 2>&1 || true
api DELETE "tags/$TAG" >/dev/null 2>&1 || true

api POST tags -d "{\"tag_name\":\"$TAG\",\"target_commitish\":\"$SHA\"}" >/dev/null

echo "Creating release $TAG (target $SHA)..."
api POST releases -d "{\"tag_name\":\"$TAG\",\"name\":\"$TAG\",\"target_commitish\":\"$SHA\",\"draft\":true,\"prerelease\":true,\"body\":$(jq -Rs . < "$NOTES_FILE")}" >release.json
REL_ID=$(jq -e '.id' release.json)
echo "Release id $REL_ID created."

shasum=$(api GET "tags/$TAG" | jq -r '.commit.sha // empty')
[ "$shasum" = "$SHA" ] || {
    echo "tag $TAG sha mismatch: got '${shasum:-none}', want $SHA" >&2
    exit 1
}

n=0
for f in "$DIST_DIR"/*; do
    name=$(basename "$f")
    [ "$name" = "release.json" ] && continue
    echo "Uploading $name ($(du -h "$f" | cut -f1))..."
    curl --fail -sS -X POST \
        -H "Authorization: token $TOKEN" \
        -F "attachment=@$f" \
        "$RELEASE_URL/api/v1/repos/$REPOSITORY/releases/$REL_ID/assets?name=$name"
    echo " -> $name uploaded"
    n=$((n + 1))
done
echo "$n assets uploaded."

echo "Publishing (draft -> false)..."
api PATCH "releases/$REL_ID" -d '{"draft": false}'
echo "Release $TAG published: $RELEASE_URL/$REPOSITORY/releases/tag/$TAG"
