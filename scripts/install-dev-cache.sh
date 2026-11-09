#!/usr/bin/env bash
# Select the compiler wrapper after shell initialization; source from install-dev.sh.
# Disabling the wrapper keeps rustc inside the build service's resource limits.
if [ "${CODETETHER_INSTALL_SCCACHE:-1}" = 0 ]; then
  export RUSTC_WRAPPER=
elif command -v sccache >/dev/null 2>&1; then
  export RUSTC_WRAPPER="${RUSTC_WRAPPER:-sccache}"
fi