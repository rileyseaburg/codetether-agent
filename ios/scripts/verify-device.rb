#!/usr/bin/env ruby
# No provisioning input: prove the physical app can reuse its Keychain token.
require 'json'
require 'time'
device = ARGV.fetch(0)
bundle = 'run.codetether.ios'
evidence = File.expand_path("~/CodeTether-iOS-evidence/relaunch-#{Time.now.utc.strftime('%Y%m%dT%H%M%SZ')}")
Dir.mkdir(evidence)
base = ['xcrun', 'devicectl', 'device']
domain = ['--device', device, '--domain-type', 'appDataContainer', '--domain-identifier', bundle]
files = File.join(evidence, 'documents-before.json')
abort 'Cannot inspect app files' unless system(*base, 'info', 'files', *domain,
  '--subdirectory', 'Documents', '--json-output', files)
listing = JSON.parse(File.read(files)).fetch('result').fetch('files')
abort 'Unexpected bootstrap: Keychain-only test refused' if listing.any? { |f| f['name'] == 'bootstrap.json' }
started = Time.now - 2
abort 'Launch failed' unless system(*base, 'process', 'launch', '--device', device,
  '--terminate-existing', bundle, '--json-output', File.join(evidence, 'launch.json'))
receipt_path = File.join(evidence, 'connection-receipt.json')
verified = false
12.times do
  sleep 5
  copied = system(*base, 'copy', 'from', *domain, '--source', 'Documents/connection-receipt.json',
    '--destination', receipt_path, out: File::NULL, err: File::NULL)
  next unless copied
  receipt = JSON.parse(File.read(receipt_path))
  next unless Time.iso8601(receipt.fetch('checkedAt')) >= started
  next unless receipt.fetch('endpoint') == 'https://server.codetether.run'
  puts JSON.pretty_generate(receipt)
  verified = true
  break
end
abort 'No fresh authenticated receipt after Keychain-only relaunch' unless verified
puts "Keychain-only physical-device proof: #{evidence}"
# Receipt contains endpoint/version/count/time, never bearer or Apple keys.
# Device UUID in launch.json binds this check to the physical iPhone.
