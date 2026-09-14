#!/bin/sh
# Model selectors are data, not commands; read the current capability schema.
ct_valid_model() {
    printf '%s\n' "$1" | grep -Eq '^[[:alnum:]_-]+/[[:alnum:]_.:/+-]+$'
}
ct_discover_model() {
    "$1" models --json 2>/dev/null |
        sed -n 's/.*"selectable_id":[[:space:]]*"\([^"]*\)".*/\1/p' |
        head -1
}