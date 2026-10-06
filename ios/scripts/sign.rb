#!/usr/bin/env ruby
# Receives Vault JSON over SSH stdin; no credentials in source or argv logs.
require 'json'
require 'tmpdir'
require 'fileutils'
credentials = JSON.parse($stdin.read)
apple = credentials.fetch('apple')
key_field = apple.keys.find { |name| name.match?(/^AuthKey_[A-Z0-9]+\.p8$/) }
abort 'No Apple signing API key in Vault response' unless key_field
key_id = key_field.delete_prefix('AuthKey_').delete_suffix('.p8')
root = File.expand_path('~/CodeTether-iOS')
mode = ARGV.fetch(0, 'build')
evidence = File.expand_path('~/CodeTether-iOS-evidence')
keychain = File.expand_path('~/Library/Keychains/login.keychain-db')
if credentials['password']
  abort 'Keychain unlock failed' unless system('security', 'unlock-keychain',
    '-p', credentials['password'], keychain, out: File::NULL)
end
Dir.mktmpdir('codetether-signing-') do |directory|
  path = File.join(directory, key_field)
  File.write(path, apple.fetch(key_field), mode: 'w', perm: 0600)
  Dir.chdir(root) do
    live = mode == 'live-test'
    destination = live ? 'platform=iOS,id=00008110-001879C23E0A401E' : 'generic/platform=iOS'
    args = ['xcodebuild', live ? 'test' : 'build', '-project', 'CodeTether.xcodeproj',
      '-scheme', live ? 'CodeTetherLive' : 'CodeTether',
      '-destination', destination, '-derivedDataPath', 'build-device',
      '-allowProvisioningUpdates', '-authenticationKeyPath', path,
      '-authenticationKeyID', key_id, '-authenticationKeyIssuerID', apple.fetch('issuer_id')]
    log = File.join(evidence, "device-#{mode}-#{Time.now.utc.strftime('%Y%m%dT%H%M%SZ')}.log")
    args += ['-resultBundlePath', log.sub('.log', '.xcresult')] if live
    ok = system(*args, out: log, err: [:child, :out])
    abort "Xcode #{mode} failed; see #{log}" unless ok
  end
end
puts "Xcode #{mode} succeeded"
