# v3.2.1

## What's New

- **Background provider loading** — Providers now initialize asynchronously, improving startup time
- **Kubernetes node identification** — Workers include `K8S_NODE_NAME` in registration for better cluster visibility
- **Expanded worker capabilities** — Enhanced A2A worker feature set

## Bug Fixes

- Fixed scroll behavior in the TUI message panel

## Changes

- **TUI performance** — Message line rendering is now cached for smoother scrolling and reduced redraw overhead

---

**Stats:** 3 commits, 3 files changed, 265 insertions(+), 96 deletions(-)
