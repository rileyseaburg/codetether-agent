#!/bin/sh
# Mock RustyRoad MCP: assert handshake/arguments, return deterministic results.
set -eu
IFS= read -r request
case "$request" in *'"method":"initialize"'*) ;; *) exit 2 ;; esac
printf '%s\n' '{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05","serverInfo":{"name":"rustyroad-mcp","version":"fixture"},"capabilities":{"tools":{}}}}'
IFS= read -r request
case "$request" in *'"method":"notifications/initialized"'*) ;; *) exit 3 ;; esac
IFS= read -r request
case "$request" in *'"method":"tools/list"'*) ;; *) exit 4 ;; esac
printf '%s\n' '{"jsonrpc":"2.0","id":2,"result":{"tools":[{"name":"rustyroad_config","inputSchema":{"type":"object"}},{"name":"rustyroad_error","inputSchema":{"type":"object"}}]}}'
IFS= read -r request
case "$request" in *'"method":"tools/call"'*) ;; *) exit 5 ;; esac
case "$request" in *'"env":"test"'*) ;; *) exit 6 ;; esac
case "$request" in *'approval_id'*|*'justification'*) exit 7 ;; esac
case "$request" in
    *'"name":"rustyroad_error"'*)
        printf '%s\n' '{"jsonrpc":"2.0","id":3,"result":{"content":[{"type":"text","text":"fixture backend error"}],"isError":true}}'
        ;;
    *)
        printf '%s\n' '{"jsonrpc":"2.0","id":3,"result":{"content":[{"type":"text","text":"fixture result"}]}}'
        ;;
esac
# Remain alive to exercise explicit child cleanup after a response.
IFS= read -r request || true