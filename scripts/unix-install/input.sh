#!/bin/sh
# Interactive input through the controlling terminal, never the piped script stdin.
ct_prompt() {
    printf '%s' "$1" > /dev/tty
    IFS= read -r CT_INPUT < /dev/tty
}

ct_quote() {
    printf "'"
    printf '%s' "$1" | sed "s/'/'\\\\''/g"
    printf "'"
}

ct_address_valid() {
    case "$1" in https://?*|http://localhost:*|http://127.0.0.1:*) ;; *) return 1 ;; esac
    case "$1" in *'@'*|*'?'*|*'#'*|*[[:space:]]*) return 1 ;; esac
}