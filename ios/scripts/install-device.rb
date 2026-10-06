#!/usr/bin/env ruby
# Run on the Mac with Vault's {"token":...} JSON on stdin.
require 'json'
require 'tmpdir'
require 'time'
device = ARGV.fetch(0)
bundle = 'run.codetether.ios'
root = File.expand_path('~/CodeTether-iOS')
evidence = File.expand_path("~/CodeTether-iOS-evidence/device-#{Time.now.utc.strftime('%Y%m%dT%H%M%SZ')}")
Dir.mkdir(evidence)
app = File.join(root, 'build-device/Build/Products/Debug-iphoneos/CodeTether.app')
base = ['xcrun', 'devicectl', 'device']
abort 'Installation failed' unless system(*base, 'install', 'app', '--device', device, app,
  '--json-output', File.join(evidence, 'device-install.json'))
payload = JSON.parse($stdin.read)
abort 'Missing token' if payload.fetch('token').empty?
domain = ['--device', device, '--domain-type', 'appDataContainer', '--domain-identifier', bundle]
Dir.mktmpdir('codetether-bootstrap-') do |directory|
  path = File.join(directory, 'bootstrap.json')
  File.write(path, JSON.generate(payload), mode: 'w', perm: 0600)
  abort 'Token transfer failed' unless system(*base, 'copy', 'to', *domain,
    '--source', path, '--destination', 'Documents/bootstrap.json')
  started = Time.now - 2
  abort 'App launch failed; unlock and open CodeTether to consume the token' unless system(
    *base, 'process', 'launch', '--device', device, '--terminate-existing', bundle,
    '--json-output', File.join(evidence, 'device-launch.json'))
  receipt_path = File.join(evidence, 'device-connection-receipt.json')
  verified = false
  12.times do
    sleep 5
    copied = system(*base, 'copy', 'from', *domain,
      '--source', 'Documents/connection-receipt.json', '--destination', receipt_path,
      out: File::NULL, err: File::NULL)
    next unless copied
    receipt = JSON.parse(File.read(receipt_path))
    next unless Time.iso8601(receipt.fetch('checkedAt')) >= started
    next unless receipt.fetch('endpoint') == 'https://server.codetether.run'
    puts JSON.pretty_generate(receipt)
    verified = true
    break
  end
  abort 'App installed, but no fresh authenticated connection receipt' unless verified
end
system(*base, 'info', 'files', *domain, '--subdirectory', 'Documents',
  '--json-output', File.join(evidence, 'device-documents.json'))
puts 'Physical iPhone authenticated refresh observed'
# The app consumes bootstrap.json into device-only Keychain storage on launch.
# Evidence includes no bearer or Apple private keys.
