# frozen_string_literal: true

require "bundler/gem_tasks"
require "rubygems/package_task"
require "rake/testtask"
require "rake/extensiontask"
require "rake_compiler_dock"
# require "rspec/core/rake_task"
# require "rubocop/rake_task"
# require "yard"

task default: %i[test]

cross_rubies = %w[3.1.0 3.0.0 2.7.0]
cross_platforms = %w[x86_64-linux x86_64-darwin arm64-darwin]
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
  end
end

task :setup do
  RakeCompilerDock.sh "bundle"
end

cross_platforms.each do |p|
  task "native:#{p}" => :setup
end

# RSpec::Core::RakeTask.new(:spec, [] => [:compile])
#
# task test: :spec
#
# task :docs do
#   `yard server --reload`
# end

# RuboCop::RakeTask.new
# YARD::Rake::YardocTask.new
