#!/usr/bin/env ruby
require 'json'
attempt = ARGV.fetch(0, '04')
test = ARGV.fetch(1, 'ReplyObservation/testExistingReplyError')
abort 'Invalid observer selection' unless attempt.match?(/\A\d{2}\z/) && test.match?(/\A\w+\/\w+\z/)
credentials = JSON.parse($stdin.read).fetch('data').fetch('data')
keychain = File.expand_path('~/Library/Keychains/login.keychain-db')
abort 'Cannot unlock provisioned signing Keychain' unless system('security', 'unlock-keychain',
  '-p', credentials.fetch('password'), keychain, out: File::NULL, err: File::NULL)
credentials.clear
root = File.expand_path('~/CodeTether-DeviceEval-20261009T052914Z')
runner = File.join(root, "ReplyObserver-#{attempt}.app")
bundle = File.join(runner, 'PlugIns/CodeTetherUITests.xctest')
File.open(File.join(root, "results/observer-sign-#{attempt}-retry.log"), File::WRONLY | File::CREAT | File::EXCL) do |log|
  [bundle, runner].each do |path|
    abort 'Observer signing failed' unless system('codesign', '--force', '--sign',
      'Apple Development: Created via API (79P599J7R5)',
      '--preserve-metadata=entitlements,requirements,flags', path, out: log, err: log)
  end
  abort 'Signature check failed' unless system('codesign', '--verify', '--deep', '--strict', runner, out: log, err: log)
end
configuration = File.read(File.join(root, 'manual.xctestrun')).sub(
  'ManualBridge/testManualConnection', test)
File.open(File.join(root, "reply-observation-#{attempt}.xctestrun"), File::WRONLY | File::CREAT | File::EXCL) do |file|
  file.write(configuration)
end
puts 'observer_signature_exit=0'