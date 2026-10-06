#!/usr/bin/env ruby
# Credential-free source and signed-product identity evidence.
require 'json'
require 'digest'
require 'time'
root = File.expand_path('~/CodeTether-iOS')
evidence = File.expand_path('~/CodeTether-iOS-evidence')
app = File.join(root, 'build-device/Build/Products/Debug-iphoneos/CodeTether.app')
abort 'Invalid code signature' unless system('codesign', '--verify', '--deep', '--strict', app)
system('codesign', '-d', '--verbose=4', app,
  out: File.join(evidence, 'codesign.txt'), err: [:child, :out])
manifest = { checkedAt: Time.now.utc.iso8601, bundleID: 'run.codetether.ios',
  sourceFiles: {}, productFiles: {} }
Dir.chdir(root) do
  Dir.glob('{Sources,Tests,UITests}/**/*.swift').sort.each do |path|
    manifest[:sourceFiles][path] = Digest::SHA256.file(path).hexdigest
  end
end
Dir.chdir(app) do
  Dir.glob('**/*').sort.select { |path| File.file?(path) }.each do |path|
    manifest[:productFiles][path] = Digest::SHA256.file(path).hexdigest
  end
end
File.write(File.join(evidence, 'build-manifest.json'), JSON.pretty_generate(manifest))
puts "Signature verified; #{manifest[:sourceFiles].length} source hashes and #{manifest[:productFiles].length} product hashes recorded"
