#!/bin/bash
# Build/sign only a copied test runner. Never install the owner application.
set -euo pipefail
cd "$HOME/CodeTether-DeviceEval-20261009T052914Z"
mkdir -p results
source="$HOME/CodeTether-ScreenTest-20261009T051600Z/build-device/Build/Products/Debug-iphoneos/CodeTetherUITests-Runner.app"
runner="$PWD/CodeTetherUITests-Runner.app"
ditto "$source" "$runner"
platform="$(xcode-select -p)/Platforms/iPhoneOS.platform"
sdk="$(xcrun --sdk iphoneos --show-sdk-path)"
bundle="$runner/PlugIns/CodeTetherUITests.xctest"
xcrun --sdk iphoneos swiftc -target arm64-apple-ios17.0 -sdk "$sdk" \
  -parse-as-library -emit-library -module-name CodeTetherUITests \
  -F "$platform/Developer/Library/Frameworks" -I "$platform/Developer/usr/lib" \
  -L "$platform/Developer/usr/lib" \
  -Xlinker -rpath -Xlinker @executable_path/Frameworks \
  -Xlinker -rpath -Xlinker @loader_path/Frameworks \
  ManualBridge.swift ManualActions.swift ManualState.swift -o "$bundle/CodeTetherUITests" \
  > results/build.log 2>&1
identity='Apple Development: Created via API (79P599J7R5)'
codesign --force --sign "$identity" --preserve-metadata=entitlements,requirements,flags "$bundle" \
  > results/sign.log 2>&1
codesign --force --sign "$identity" --preserve-metadata=entitlements,requirements,flags "$runner" \
  >> results/sign.log 2>&1
codesign --verify --deep --strict "$runner" >> results/sign.log 2>&1
printf 'runner_build_and_signature_exit=0\n'
# The existing runner's provisioning profile is retained. No signing secrets
# or app bearer credentials are copied into logs or passed to the test.