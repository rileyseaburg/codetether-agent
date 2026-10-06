#!/usr/bin/env ruby
# Only allowlisted installation assets may enter the mounted NAS folder.
require 'digest'
require 'fileutils'
require 'json'
require 'open3'
source = File.expand_path(ARGV.fetch(0))
version, build, expected = ARGV[1, 3]
abort 'Usage: publish-nas.rb ASSETS VERSION BUILD SHA256' unless
  version&.match?(/\A\d+\.\d+\.\d+\z/) && build&.match?(/\A[1-9]\d*\z/) &&
  expected&.match?(/\A[a-f0-9]{64}\z/)
abort 'IPA checksum mismatch' unless Digest::SHA256.file(File.join(source, 'CodeTether.ipa')).hexdigest == expected
nas = File.expand_path('~/NAS')
mounts, status = Open3.capture2('mount')
abort 'NAS share is not mounted' unless status.success? &&
  mounts.lines.any? { |line| line.include?(" on #{nas} (smbfs,") && line.include?('192.168.50.133/NAS') }
files = %w[CodeTether.ipa manifest.plist index.html release.json SHA256SUMS]
abort 'Unexpected publication assets' unless Dir.children(source).sort == files.sort
destination = File.join(nas, "CodeTether-iOS-#{version}-#{build}")
abort 'NAS destination is a symlink' if File.symlink?(destination)
FileUtils.mkdir_p(destination)
abort 'Existing NAS folder has unexpected files' unless (Dir.children(destination) - files).empty?
hashes = files.to_h do |file|
  input, output = File.join(source, file), File.join(destination, file)
  hash = Digest::SHA256.file(input).hexdigest
  abort "Refusing to replace different NAS asset: #{file}" if File.exist?(output) &&
    Digest::SHA256.file(output).hexdigest != hash
  FileUtils.cp(input, output) unless File.exist?(output)
  abort "Copy checksum mismatch: #{file}" unless Digest::SHA256.file(output).hexdigest == hash
  [file, hash]
end
puts JSON.pretty_generate(validation_level: 'static/local', folder: destination,
  share: 'smb://192.168.50.133/NAS', file_sha256: hashes)