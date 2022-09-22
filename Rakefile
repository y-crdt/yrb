# frozen_string_literal: true

require "bundler/gem_tasks"
require "rubygems/package_task"
require "rake/testtask"
require "rake/extensiontask"

cross_rubies = %w[3.1.0 3.0.0 2.7.0]
cross_platforms = %w[
  arm-linux
  aarch64-linux
  arm64-darwin
  x86_64-darwin
  x86_64-linux
]
ENV["RUBY_CC_VERSION"] = cross_rubies.join(":")

spec = Bundler.load_gemspec("y-rb.gemspec")

Gem::PackageTask.new(spec).define
task "package" => cross_platforms.map { |p| "gem:#{p}" }

Rake::ExtensionTask.new("yrb", spec) do |ext|
  ext.source_pattern = "*.{rs,toml}"
  ext.cross_compile = true
  ext.cross_platform = cross_platforms
  ext.config_script = ENV["ALTERNATE_CONFIG_SCRIPT"] || "extconf.rb"
  ext.cross_compiling do |spec|
    spec.files.reject! { |file| File.fnmatch?("*.tar.gz", file) }
    spec.dependencies.reject! { |dep| dep.name == "rb-sys" }
  end
end

task :setup do
  require "rake_compiler_dock"
  RakeCompilerDock.sh "bundle"
rescue => e
  warn e.message
end

cross_platforms.each do |p|
  task "native:#{p}" => :setup
end

begin
  require "rspec/core/rake_task"
  RSpec::Core::RakeTask.new(:spec, [] => [:compile])
  task test: :spec
  task default: %i[test]
rescue LoadError
  # Ok
end

begin
  require "rubocop/rake_task"

  RuboCop::RakeTask.new
rescue LoadError
  # Ok
end


begin
  require "yard"

  YARD::Rake::YardocTask.new

  task :docs do
    `yard server --reload`
  end
rescue LoadError
  # Ok
end
