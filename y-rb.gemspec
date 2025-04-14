# frozen_string_literal: true

begin
  require_relative "lib/y/version"
rescue LoadError
  puts "WARNING: Could not load Y::VERSION"
end

Gem::Specification.new do |spec|
  spec.name = "y-rb"
  spec.version = defined?(Y::VERSION) ? Y::VERSION : "0.0.0"
  spec.authors = ["Hannes Moser"]
  spec.email = %w[hmoser@gitlab.com box@hannesmoser.at]

  spec.summary = "Ruby bindings for yrs"
  spec.description = "Ruby bindings for yrs. Yrs \"wires\" is a Rust port of the Yjs framework."
  spec.homepage = "https://github.com/y-crdt/yrb"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.1.0"
  # https://github.com/rubygems/rubygems/pull/5852#issuecomment-1231118509
  spec.required_rubygems_version = ">= 3.3.22"

  spec.metadata["allowed_push_host"] = "https://rubygems.org"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/y-crdt/yrb"
  spec.metadata["documentation_uri"] = "https://y-crdt.github.io/yrb/"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir["lib/**/*.rb", "ext/**/*.{rs,toml,lock,rb}"]

  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  spec.metadata["rubygems_mfa_required"] = "true"

  spec.add_dependency "rake", "~> 13.2"
  spec.add_dependency "rb_sys", "~> 0.9.110"

  spec.add_development_dependency "base64"
  spec.add_development_dependency "rake-compiler", "~> 1.3.0"

  spec.extensions = ["ext/yrb/extconf.rb"]
end
