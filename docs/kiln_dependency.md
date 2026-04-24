# Kiln Dependency

CodeTether uses Kiln's scripting VM for the `kiln_plugin` tool.

The VM API used by this tool is pinned to a Git revision because the scripting
crate is not currently available from crates.io as a compatible release. Build
and CI environments therefore need `git` and network access to fetch:

```text
https://github.com/kiln-rs/kiln.git
```

The revision is locked in both `Cargo.toml` and `Cargo.lock` to keep builds
reproducible once the dependency has been fetched.
