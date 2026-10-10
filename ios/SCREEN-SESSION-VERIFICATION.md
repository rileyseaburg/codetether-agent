# Screen multi-turn session repair

## Implementation — static/local

This repair addresses iOS session-lifecycle defects, not a demonstrated model
inability to answer follow-up requests. It does not recreate a Screen session,
change its one-hour expiry, replay owner commands, or alter the relay protocol.

- `ScreenClient` copies its configuration before constructing the REST client.
  Previously REST changed the shared resource timeout from 3900 to 180 seconds,
  so subsequent SSE sessions inherited the three-minute limit.
- `ScreenStreamRetry` counts consecutive failures rather than connections over
  the whole session. New event sequences or a connection lasting 30 seconds
  reset the budget. Repeated immediate snapshot/disconnect cycles still stop
  after bounded 1/2/4/8-second backoff. Authentication and expiry are not retried.
- `ScreenModel+StreamLoop` keeps generation/cancellation checks and delegates
  retry accounting. Repeated activation does not replace a healthy connection.
- `ScreenModel+Availability` and `ScreenStatusView` allow manual Reconnect after
  automatic retries are exhausted, while retaining session ownership and drafts.
- `ScreenModel+Question` does not restore the waiting-for-frame notice if the
  analysis already advanced before the HTTP acceptance arrived. Availability
  prevents overlapping question/reply submissions; neither is automatically resent.

## Verification — mocked local

On Xcode 16.4, the dedicated iOS 18.6 simulator ran 28 selected tests with zero
failures, twice (attempts 01 and 02; both xcodebuild exits 0). Attempt 02 includes
all final source changes. These use in-memory clients and URLProtocol fixtures,
not the live model, relay, paired iPhone, or Windows keyboard delivery.

Coverage includes 20 question/reply turns on one session, late HTTP acceptance,
duplicate submit prevention, configuration isolation, advancing and idle stream
recovery, retry exhaustion and manual reconnect without resending, preservation
of drafts, lifecycle cancellation, HTTP failures, and SSE parsing.

Repository-root evidence directory:
`windows/CodeTether.Companion/artifacts/screen-multiturn-20261009T055645Z/`

It contains `run-simulator.sh`, source snapshots, `screen-01`/`screen-02` logs,
exit files and summary JSON, and `screen-results.xcresult.tar.gz`. Original
result bundles remain on the Mac in
`~/CodeTether-ScreenMultiTurn-20261009T055645Z/results/`.

## Delivery — real platform upload (direct iPhone install)

After the user explicitly requested installation, Xcode 16.4 built and signed
version 1.5.1 (23). The in-place install on the paired iPhone 13 Pro Max succeeded;
`devicectl` independently reported `run.codetether.ios`, build 23. The installed
app was launched without replacing its saved credentials. This is a direct
device installation, not an App Store upload. Build/signature evidence is
static/local; install and launch evidence is real platform upload.

Evidence: `windows/CodeTether.Companion/artifacts/ios-install-20261009T060053Z/`
from the repository root. See its README, build log, install receipt, installed
app metadata, launch receipt, and signed app archive.

The post-launch authentication check failed to obtain a fresh connection receipt
(exit 1); its retained receipt predates launch. Saved-credential reuse is therefore
unconfirmed, not a passing live connection test. The cause is unresolved.

Live multi-turn and Windows insertion verification for this build are not-run.
No capture or typing request was sent and neither Windows nor the relay was
restarted. The phone's Screen pairing/drafts are memory-only and a fresh pairing
is needed after its restart. The separate single-composer UI is not delivered by
these session-lifecycle fixes.
