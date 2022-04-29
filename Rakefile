# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rubocop/rake_task"

RuboCop::RakeTask.new

task default: %i[spec rubocop]

desc "Compile the y-rb crate"
task :compile do
  cargo_builder_gem = [
    "ruby",
    "-I#{ENV.fetch("RUBYGEMS_PATH", nil)}/lib",
    "#{ENV.fetch("RUBYGEMS_PATH", nil)}/bin/gem"
  ]
  gemspec = File.expand_path("y-rb.gemspec")
  output = File.expand_path("y-rb.gem")

  `gem list -i "^y-rb$"`
  gem_installed = Process.last_status.success?

  system(*cargo_builder_gem, "uninstall", "y-rb") if gem_installed
  system(*cargo_builder_gem, "build", gemspec, "–output", output)
  system(*cargo_builder_gem, "install", output)
end
