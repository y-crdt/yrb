# frozen_string_literal: true

require "bundler"
require "bundler/gem_tasks"
require "rake/extensiontask"
require "rspec/core/rake_task"
require "rubocop/rake_task"
require "yard"

task default: %i[test]

spec = Bundler.load_gemspec("y-rb.gemspec")

Gem::PackageTask.new(spec)

Rake::ExtensionTask.new('yrb', spec) do |ext|
  ext.lib_dir = "lib/yrb"
  ext.source_pattern = "*.{rs,toml}"
  ext.cross_compile = true
  ext.cross_platform = %w[x86_64-linux x86_64-darwin arm64-darwin]
  ext.config_script = ENV["ALTERNATE_CONFIG_SCRIPT"] || "extconf.rb"
end

RSpec::Core::RakeTask.new(:spec, [] => [:compile])

task test: :spec

task :docs do
  `yard server --reload`
end

RuboCop::RakeTask.new
YARD::Rake::YardocTask.new
