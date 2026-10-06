#!/usr/bin/env ruby
# Exports a manually signed registered-device IPA; no Apple secrets are bundled.
require 'json'
require 'fileutils'
require 'openssl'
require 'digest'
root = File.expand_path('~/CodeTether-iOS')
signing = File.expand_path('~/CodeTether-iOS-signing')
password = $stdin.read.strip
abort 'Vault-managed Keychain password required on stdin' if password.empty?
abort 'Keychain unlock failed' unless system('security', 'unlock-keychain', '-p', password,
  File.expand_path('~/Library/Keychains/login.keychain-db'), out: File::NULL)
profile = JSON.parse(File.read(File.join(signing, 'adhoc-profile.json'))).fetch('data').fetch('attributes')
uuid = profile.fetch('uuid')
certificate = OpenSSL::X509::Certificate.new(File.binread(File.join(signing, 'distribution.cer')))
identity = Digest::SHA1.hexdigest(certificate.to_der).upcase
out = File.expand_path("~/CodeTether-iOS-distribution/#{Time.now.utc.strftime('%Y%m%dT%H%M%SZ')}")
FileUtils.mkdir_p(out, mode: 0700)
archive = File.join(out, 'CodeTether.xcarchive')
export = File.join(out, 'export')
options = File.join(out, 'ExportOptions.plist')
File.write(options, <<~PLIST)
  <?xml version="1.0" encoding="UTF-8"?>
  <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
  <plist version="1.0"><dict>
  <key>method</key><string>release-testing</string>
  <key>signingStyle</key><string>manual</string>
  <key>teamID</key><string>J9YRM3U37D</string>
  <key>signingCertificate</key><string>#{identity}</string>
  <key>provisioningProfiles</key><dict><key>run.codetether.ios</key><string>#{uuid}</string></dict>
  <key>stripSwiftSymbols</key><true/>
  <key>manageAppVersionAndBuildNumber</key><false/>
  <key>thinning</key><string>&lt;none&gt;</string>
  </dict></plist>
PLIST
Dir.chdir(root) do
  args = ['xcodebuild', 'archive', '-project', 'CodeTether.xcodeproj', '-scheme', 'CodeTether',
    '-configuration', 'Release', '-destination', 'generic/platform=iOS', '-derivedDataPath', 'build-device',
    '-archivePath', archive, 'CODE_SIGN_STYLE=Manual', "CODE_SIGN_IDENTITY=#{identity}",
    "PROVISIONING_PROFILE_SPECIFIER=#{uuid}"]
  abort "Archive failed: #{out}" unless system(*args, out: File.join(out, 'archive.log'), err: [:child, :out])
  abort "Export failed: #{out}" unless system('xcodebuild', '-exportArchive', '-archivePath', archive,
    '-exportOptionsPlist', options, '-exportPath', export, out: File.join(out, 'export.log'), err: [:child, :out])
end
ipa = File.join(export, 'CodeTether.ipa')
abort 'IPA missing' unless File.file?(ipa)
puts JSON.generate(directory: out, ipa: ipa, sha256: Digest::SHA256.file(ipa).hexdigest)