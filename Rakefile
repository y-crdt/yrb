# frozen_string_literal: true

require "bundler/gem_tasks"
require "rubygems/package_task"
require "rake/testtask"
require "rake/extensiontask"
require "rb_sys"

cross_rubies = %w[3.1.0 3.0.0 2.7.0]
cross_platforms = %w[
  aarch64-linux
  arm64-darwin
  x86_64-darwin
  x86_64-linux
  x86_64-linux-musl
  x64-mingw32
  x64-mingw-ucrt
]

spec = Bundler.load_gemspec("y-rb.gemspec")

Gem::PackageTask.new(spec)

Rake::ExtensionTask.new("yrb", spec) do |ext|
  ext.source_pattern = "*.{rs,toml}"
  ext.cross_compile = true
  ext.cross_platform = cross_platforms
  ext.config_script = ENV["ALTERNATE_CONFIG_SCRIPT"] || "extconf.rb"
  ext.cross_compiling do |c|
    c.files.reject! { |file| File.fnmatch?("*.tar.gz", file) }
    c.dependencies.reject! { |dep| dep.name == "rb-sys" }
  end
end

namespace "gem" do
  task "prepare" do
    sh "bundle"
  end

  cross_platforms.each do |plat|
    desc "Build the native gem for #{plat}"
    task plat => "prepare" do
      require "rake_compiler_dock"

      ENV["RCD_IMAGE"] = "rbsys/#{plat}:#{RbSys::VERSION}"

      RakeCompilerDock.sh <<~SH, platform: plat
        bundle && \
        RUBY_CC_VERSION="#{cross_rubies.join(":")}" \
        rake native:#{plat} pkg/#{spec.full_name}-#{plat}.gem
      SH
    end
  end
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
