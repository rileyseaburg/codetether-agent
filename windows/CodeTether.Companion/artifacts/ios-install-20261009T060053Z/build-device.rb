#!/usr/bin/env ruby
# Build the simulator-verified source for an in-place iPhone update.
# Vault credentials arrive on stdin; no owner bearer is read or provisioned.
require 'json'
require 'tmpdir'
root = File.expand_path('~/CodeTether-ScreenMultiTurn-20261009T055645Z')
evidence = File.expand_path('~/CodeTether-iOSInstall-20261009T060053Z')
credentials = JSON.parse($stdin.read)
apple = credentials.fetch('apple')
key = apple.keys.find { |name| name.match?(/^AuthKey_[A-Z0-9]+\.p8$/) }
abort 'Missing Apple signing key' unless key
key_id = key.delete_prefix('AuthKey_').delete_suffix('.p8')
log = File.join(evidence, 'build-device.log')
abort 'Refusing to overwrite build evidence' if File.exist?(log)
Dir.chdir(root) do
  abort 'Source snapshot mismatch' unless system('shasum', '-a', '256', '-c',
    File.join(evidence, 'source.sha256'), out: File.join(evidence, 'source-check.log'))
end
abort 'Keychain unlock failed' unless system('security', 'unlock-keychain',
  '-p', credentials.fetch('password'), File.expand_path('~/Library/Keychains/login.keychain-db'),
  out: File::NULL)
status = nil
Dir.mktmpdir('codetether-signing-') do |directory|
  path = File.join(directory, key)
  File.write(path, apple.fetch(key), mode: 'w', perm: 0600)
  Dir.chdir(root) do
    system('xcodebuild', 'build', '-project', 'CodeTether.xcodeproj',
      '-scheme', 'CodeTether', '-destination', 'generic/platform=iOS',
      '-derivedDataPath', File.join(evidence, 'build-device'),
      '-allowProvisioningUpdates', '-authenticationKeyPath', path,
      '-authenticationKeyID', key_id, '-authenticationKeyIssuerID', apple.fetch('issuer_id'),
      'CURRENT_PROJECT_VERSION=23', out: log, err: [:child, :out])
    status = $?.exitstatus
  end
end
File.write(File.join(evidence, 'build-device.exit.txt'), "#{status}\n")
abort "Device build failed (#{status}); inspect build-device.log" unless status == 0
app = File.join(evidence, 'build-device/Build/Products/Debug-iphoneos/CodeTether.app')
abort 'Signature verification failed' unless system('codesign', '--verify', '--deep', '--strict', app,
  out: File.join(evidence, 'signature.log'), err: [:child, :out])
puts "Device build and signature check succeeded: #{app}"
