#!/usr/bin/env ruby
# frozen_string_literal: true

require "active_support/core_ext/string"
require "fileutils"
require "minitar"
require "zlib"

require_relative "../lib/y/version"

RUBIES = %w[2.6.10 2.7.6 3.0.4 3.1.2].freeze

# darwin20 = mac os 11.x
# darwin21 = mac os 12.x
# darwin22 = mac os 13.x
TARGETS = {
  "aarch64-apple-darwin": {
    cpu: "arm64",
    os: "darwin21"
  },
  "aarch64-unknown-linux-gnu": {
    cpu: "arm64",
    os: "linux"
  },
  "aarch64-unknown-linux-musl": {
    cpu: "arm64",
    os: "linux-musl",
    env: {
      "RUSTFLAGS" => "\"-C target-feature=-crt-static\""
    }
  },
  "x86_64-unknown-linux-gnu": {
    cpu: "x86_64",
    os: "linux"
  },
  "x86_64-unknown-linux-musl": {
    cpu: "x86_64",
    os: "linux-musl",
    env: {
      "RUSTFLAGS" => "\"-C target-feature=-crt-static\""
    }
  }
}.freeze

def build_lib(target, options, ruby_version) # rubocop:disable Metrics/MethodLength
  env_vars = if !options.nil? && options.key?(:env)
               target_env = target.to_s.underscore.upcase
               options[:env]
                 .map { |env, value| "CARGO_TARGET_#{target_env}_#{env}=#{value}" }
                 .join(" ")
             else
               ""
             end

  cmd = [
    "cargo clean",
    "#{env_vars} RUBY_VERSION=#{ruby_version} cross build --target=#{target} --release"
  ].join(" && ")

  `#{cmd}`
end

def copy_lib(target)
  path = "#{__dir__}/../target/#{target}/release"

  file = Dir["#{path}/*{dylib,so}"].first

  extension = File.extname(file)
  dest = "#{path}/y_rb#{extension}"

  FileUtils.cp(file, dest)

  dest
end

# Build

FileUtils.rm_rf "#{__dir__}/out"

TARGETS.each do |target, options|
  RUBIES.each do |ruby_version|
    puts "Build: target=#{target}, ruby=#{ruby_version}"

    version = Gem::Version.new(ruby_version)
    version_segments = version.canonical_segments
    ruby_version_string = "#{version_segments[0]}#{version_segments[1]}"

    build_lib(target, options, ruby_version)
    file = copy_lib(target)

    raise "Build failed" if file.nil?

    extension = File.extname(file)
    dir = "#{__dir__}/out/y_rb-#{Y::VERSION}-ruby#{ruby_version_string}-#{options[:os]}-#{options[:cpu]}"
    FileUtils.mkdir_p(dir)
    FileUtils.cp(file, "#{dir}/y_rb#{extension}")

    archive = "#{dir}/y_rb-#{Y::VERSION}-ruby#{ruby_version_string}-#{options[:os]}-#{options[:cpu]}.tar.gz"
    Zlib::GzipWriter.open(archive) do |tgz|
      Dir.chdir(dir) do
        Archive::Tar::Minitar.pack(".", tgz)
      end
    end
  end
end
