#!/bin/bash
# Compile only the observation runner; keep every previous binary/log intact.
set -euo pipefail
cd "$HOME/CodeTether-DeviceEval-20261009T052914Z"
runner="$PWD/ReplyObserver-04.app"
[[ ! -e "$runner" && ! -e results/observer-build-04.log ]] || exit 2
ditto CodeTetherUITests-Runner.app "$runner"
platform="$(xcode-select -p)/Platforms/iPhoneOS.platform"
sdk="$(xcrun --sdk iphoneos --show-sdk-path)"
bundle="$runner/PlugIns/CodeTetherUITests.xctest"
xcrun --sdk iphoneos swiftc -target arm64-apple-ios17.0 -sdk "$sdk" \
  -parse-as-library -emit-library -module-name CodeTetherUITests \
  -F "$platform/Developer/Library/Frameworks" -I "$platform/Developer/usr/lib" \
  -L "$platform/Developer/usr/lib" \
  -Xlinker -rpath -Xlinker @executable_path/Frameworks \
  -Xlinker -rpath -Xlinker @loader_path/Frameworks \
  ReplyObservation.swift ReplyErrors.swift -o "$bundle/CodeTetherUITests" \
  > results/observer-build-04.log 2>&1
identity='Apple Development: Created via API (79P599J7R5)'
codesign --force --sign "$identity" --preserve-metadata=entitlements,requirements,flags "$bundle" \
  > results/observer-sign-04.log 2>&1
codesign --force --sign "$identity" --preserve-metadata=entitlements,requirements,flags "$runner" \
  >> results/observer-sign-04.log 2>&1
codesign --verify --deep --strict "$runner" >> results/observer-sign-04.log 2>&1
sed 's#ManualBridge/testManualConnection#ReplyObservation/testExistingReplyError#' \
  manual.xctestrun > reply-observation.xctestrun
printf 'observer_build_and_signature_exit=0\n'