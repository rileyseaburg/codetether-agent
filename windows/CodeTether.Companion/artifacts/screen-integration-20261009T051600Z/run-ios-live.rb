#!/usr/bin/env ruby
# Vault signing JSON on stdin; never bootstrap/replace the physical app bearer.
require 'json'
require 'tmpdir'
credentials = JSON.parse($stdin.read)
apple = credentials.fetch('apple')
key_field = apple.keys.find { |name| name.match?(/^AuthKey_[A-Z0-9]+\.p8$/) }
abort 'No signing API key' unless key_field
root = File.expand_path('~/CodeTether-ScreenTest-20261009T051600Z')
keychain = File.expand_path('~/Library/Keychains/login.keychain-db')
if credentials['password']
  abort 'Cannot unlock signing Keychain' unless system('security', 'unlock-keychain',
    '-p', credentials['password'], keychain, out: File::NULL)
end
result = false
Dir.mktmpdir('codetether-signing-') do |directory|
  path = File.join(directory, key_field)
  File.write(path, apple.fetch(key_field), mode: 'w', perm: 0600)
  Dir.chdir(root) do
    args = ['xcodebuild', 'test', '-project', 'CodeTether.xcodeproj', '-scheme', 'CodeTetherLive',
      '-destination', 'platform=iOS,id=00008110-001879C23E0A401E',
      '-derivedDataPath', 'build-device', '-resultBundlePath', 'results/screen-live-01.xcresult',
      '-only-testing:CodeTetherUITests/ScreenLiveUITests', '-parallel-testing-enabled', 'NO',
      '-allowProvisioningUpdates', '-authenticationKeyPath', path,
      '-authenticationKeyID', key_field.delete_prefix('AuthKey_').delete_suffix('.p8'),
      '-authenticationKeyIssuerID', apple.fetch('issuer_id')]
    result = system(*args, out: 'results/screen-live-01.log', err: [:child, :out])
    File.write('results/screen-live-01.exit.txt', "#{$?.exitstatus}\n")
  end
end
puts "Physical iPhone Screen test #{result ? 'succeeded' : 'failed'}; see results/screen-live-01.log"
exit(result ? 0 : 1)
# Only dedicated simulator tests may replace Keychain items. This physical run
# selects one UI class, never the unit-test target or broader Chat/Voice tests.
# Temporary signing key is removed; evidence and app Keychain are preserved.