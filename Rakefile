# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/extensiontask"
require "rspec/core/rake_task"
require "rubocop/rake_task"
require "yard"

task default: %i[test]

Rake::ExtensionTask.new("yrb") do |ext|
  ext.lib_dir = "lib"
  ext.cross_compile = true
end

RSpec::Core::RakeTask.new(:spec, [] => [:compile])

task test: :spec

task :docs do
  `yard server --reload`
end

RuboCop::RakeTask.new
YARD::Rake::YardocTask.new
