# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"
require "yard"
require "rubocop/rake_task"
require "thermite/tasks"

RSpec::Core::RakeTask.new(:spec)

RuboCop::RakeTask.new

task default: %i[spec rubocop]

desc "Compile the y-rb crate"
task :compile do
  # MacOS: ARM + x64
  `cargo build --release --target=aarch64-apple-darwin`
  `cargo build --release --target=x86_64-apple-darwin`

  # Copy to target folder
  `cp target/aarch64-apple-darwin/release/liby_rb.dylib target/release/`
end

task :clean do
  `cargo clean`
end

task test: :spec

task :docs do
  `yard server --reload`
end

RuboCop::RakeTask.new

YARD::Rake::YardocTask.new

Thermite::Tasks.new
