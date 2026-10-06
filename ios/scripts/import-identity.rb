#!/usr/bin/env ruby
# Mac only; password from Vault via stdin, key remains on the build host.
require 'openssl'
require 'securerandom'
require 'tmpdir'
password = $stdin.read.strip
root = File.expand_path('~/CodeTether-iOS-signing')
kind = ARGV.fetch(0, 'development')
abort 'Invalid signing identity kind' unless %w[development distribution].include?(kind)
keychain = File.expand_path('~/Library/Keychains/login.keychain-db')
key = OpenSSL::PKey::RSA.new(File.read(File.join(root, "#{kind}.key")))
cert = OpenSSL::X509::Certificate.new(File.binread(File.join(root, "#{kind}.cer")))
passphrase = SecureRandom.hex(32)
label = "CodeTether iOS #{kind}"
pkcs12 = OpenSSL::PKCS12.create(passphrase, label, key, cert)
Dir.mktmpdir('codetether-identity-') do |directory|
  path = File.join(directory, 'development.p12')
  File.binwrite(path, pkcs12.to_der)
  File.chmod(0600, path)
  abort 'Keychain unlock failed' unless system('security', 'unlock-keychain', '-p', password, keychain)
  abort 'Identity import failed' unless system('security', 'import', path, '-k', keychain,
    '-P', passphrase, '-T', '/usr/bin/codesign', '-T', '/usr/bin/security')
  abort 'Partition setup failed' unless system('security', 'set-key-partition-list',
    '-S', 'apple-tool:,apple:,codesign:', '-s', '-l', label, '-k', password,
    keychain, out: File::NULL)
end
puts "#{kind.capitalize} signing identity imported (existing certificates preserved)"