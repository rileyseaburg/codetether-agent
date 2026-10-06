#!/usr/bin/env ruby
# Verify the exported IPA, not merely the archive's development signature.
require 'digest'
require 'json'
require 'open3'
require 'tmpdir'
require 'time'
require_relative 'plist-decode'
def command(*args, input: '')
  output, error, status = Open3.capture3(*args, stdin_data: input)
  abort "#{args.first}: #{error} #{output}" unless status.success?
  output
end
ipa = File.expand_path(ARGV.fetch(0))
version, build, expected = ARGV.drop(1)
abort 'Usage: verify-adhoc.rb IPA VERSION BUILD SHA256' unless
  version&.match?(/\A\d+\.\d+\.\d+\z/) && build&.match?(/\A[1-9]\d*\z/) && expected&.match?(/\A[a-f0-9]{64}\z/)
abort 'IPA checksum mismatch' unless Digest::SHA256.file(ipa).hexdigest == expected
Dir.mktmpdir('codetether-ipa-verify-') do |directory|
  command('ditto', '-x', '-k', ipa, directory)
  apps = Dir.glob(File.join(directory, 'Payload', '*.app'))
  abort 'Expected exactly one app' unless apps.length == 1
  app = apps.first
  command('codesign', '--verify', '--deep', '--strict', app)
  info = plist(File.read(File.join(app, 'Info.plist')))
  profile = plist(command('security', 'cms', '-D', '-i', File.join(app, 'embedded.mobileprovision')))
  entitlements = plist(command('codesign', '-d', '--entitlements', ':-', app))
  device = '00008110-001879C23E0A401E'
  abort 'Wrong bundle or version' unless info['CFBundleIdentifier'] == 'run.codetether.ios' &&
    info['CFBundleShortVersionString'] == version && info['CFBundleVersion'] == build
  abort 'Riley iPhone missing from profile' unless profile.fetch('ProvisionedDevices').include?(device)
  abort 'Provisioning profile expired' unless Time.parse(profile.fetch('ExpirationDate')) > Time.now
  abort 'Debug entitlement present' unless entitlements['get-task-allow'] == false
  abort 'Wrong Apple team' unless entitlements['application-identifier'] == 'J9YRM3U37D.run.codetether.ios'
  evidence = JSON.pretty_generate(validation_level: 'static/local', sha256: expected,
    signature: 'strict verification succeeded', bundle_id: info['CFBundleIdentifier'],
    version: version, build: build, iphone_in_profile: true, profile_uuid: profile['UUID'],
    profile_expires: profile['ExpirationDate'], debug_entitlement: false,
    application_identifier: entitlements['application-identifier'],
    keychain_access_groups: entitlements.fetch('keychain-access-groups', []),
    camera_usage_description: info['NSCameraUsageDescription'])
  File.write(ARGV[4], evidence + "\n") if ARGV[4]
  puts evidence
end