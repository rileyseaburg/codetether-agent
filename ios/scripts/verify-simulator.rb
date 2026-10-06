#!/usr/bin/env ruby
# Live-network simulator check, distinct from fixture-backed XCTest tests.
require 'json'
require 'time'
device = ARGV.fetch(0)
abort 'Use a dedicated simulator and set RESET_DEDICATED_SIMULATOR=1' unless ENV['RESET_DEDICATED_SIMULATOR'] == '1'
begin
bundle = 'run.codetether.ios'
root = File.expand_path('~/CodeTether-iOS')
evidence = File.expand_path('~/CodeTether-iOS-evidence')
app = File.join(root, 'build/Build/Products/Debug-iphonesimulator/CodeTether.app')
abort 'Install failed' unless system('xcrun', 'simctl', 'install', device, app)
container = IO.popen(['xcrun', 'simctl', 'get_app_container', device, bundle, 'data'], &:read).strip
abort 'No simulator container' if container.empty?
path = File.join(container, 'Documents/bootstrap.json')
payload = JSON.parse($stdin.read)
File.write(path, JSON.generate(payload), mode: 'w', perm: 0600)
started = Time.now - 2
abort 'Launch failed' unless system('xcrun', 'simctl', 'launch', '--terminate-running-process', device, bundle)
receipt_path = File.join(container, 'Documents/connection-receipt.json')
verified = false
12.times do
  sleep 5
  next unless File.exist?(receipt_path)
  receipt = JSON.parse(File.read(receipt_path))
  next unless Time.iso8601(receipt.fetch('checkedAt')) >= started
  abort 'Bootstrap file was not removed' if File.exist?(path)
  File.write(File.join(evidence, 'simulator-live-receipt.json'), JSON.pretty_generate(receipt))
  puts JSON.pretty_generate(receipt)
  verified = true
  break
end
abort 'No fresh live authenticated receipt' unless verified
system('xcrun', 'simctl', 'io', device, 'screenshot', File.join(evidence, 'simulator-live.png'))
ensure
  system('xcrun', 'simctl', 'uninstall', device, 'run.codetether.ios')
  abort 'Dedicated simulator Keychain cleanup failed' unless system('xcrun', 'simctl', 'keychain', device, 'reset')
end
# Uninstall alone does not remove Keychain entries on iOS.
