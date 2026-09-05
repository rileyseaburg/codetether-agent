#!/bin/sh
# Share the Docker-context implementation with the Forgejo setup entry point.
exec sh "$(dirname "$0")/../../docker/release/apt-https.sh" "$@"
