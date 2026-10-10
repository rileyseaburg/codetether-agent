#!/usr/bin/env ruby
require 'json'
credentials = JSON.parse($stdin.read).fetch('data').fetch('data')
keychain = File.expand_path('~/Library/Keychains/login.keychain-db')
abort 'Cannot unlock provisioned signing Keychain' unless system('security', 'unlock-keychain',
  '-p', credentials.fetch('password'), keychain, out: File::NULL, err: File::NULL)
credentials.clear
root = File.expand_path('~/CodeTether-DeviceEval-20261009T052914Z')
runner = File.join(root, 'CodeTetherUITests-Runner.app')
bundle = File.join(runner, 'PlugIns/CodeTetherUITests.xctest')
File.open(File.join(root, 'results/sign-02.log'), 'w') do |log|
  [bundle, runner].each do |path|
    abort 'Runner signing failed' unless system('codesign', '--force', '--sign',
      'Apple Development: Created via API (79P599J7R5)',
      '--preserve-metadata=entitlements,requirements,flags', path, out: log, err: log)
  end
  abort 'Signature check failed' unless system('codesign', '--verify', '--deep', '--strict', runner, out: log, err: log)
end