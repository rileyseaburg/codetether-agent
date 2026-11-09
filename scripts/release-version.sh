#!/usr/bin/env bash
set -euo pipefail

current=${1:?usage: release-version.sh CURRENT BUMP [OVERRIDE]}
bump=${2:?usage: release-version.sh CURRENT BUMP [OVERRIDE]}
override=${3:-}

if [[ -n "$override" ]]; then
  printf '%s\n' "${override#v}"
  exit 0
fi

without_build=${current%%+*}
core=${without_build%%-*}
if [[ ! $core =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)$ ]]; then
  printf 'Invalid current version: %s\n' "$current" >&2
  exit 1
fi

major=${BASH_REMATCH[1]}
minor=${BASH_REMATCH[2]}
patch=${BASH_REMATCH[3]}
case "$bump" in
  patch) [[ $without_build == *-* ]] || ((patch += 1)) ;;
  minor) ((minor += 1)); patch=0 ;;
  major) ((major += 1)); minor=0; patch=0 ;;
  *) printf 'Unsupported bump type: %s\n' "$bump" >&2; exit 1 ;;
esac
printf '%s.%s.%s\n' "$major" "$minor" "$patch"
