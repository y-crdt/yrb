# frozen_string_literal: true

require_relative "lib/y/version"

# rubocop:disable Metrics/BlockLength
Gem::Specification.new do |spec|
  spec.name = "y-rb"
  spec.version = Y::VERSION
  spec.authors = ["Hannes Moser"]
  spec.email = %w[hmoser@gitlab.com box@hannesmoser.at]

  spec.summary = "Ruby bindings for yrs"
  spec.description = "Ruby bindings for yrs. Yrs \"wires\" is a Rust port of the Yjs framework."
  spec.homepage = "https://github.com/y-crdt/yrb"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 2.6.0"

  spec.metadata["allowed_push_host"] = "https://rubygems.org"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/y-crdt/yrb"
  spec.metadata["documentation_uri"] = "https://y-crdt.github.io/yrb/"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(File.expand_path(__dir__)) do
    `git ls-files -z`.split("\x0").reject do |f|
      (f == __FILE__) || f.match(%r{\A(?:(?:test|spec|features)/|\.(?:git|travis|circleci)|appveyor)})
    end
  end

  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  spec.metadata["rubygems_mfa_required"] = "true"

  spec.add_dependency "rake", "~> 13.0"
  spec.add_dependency "rutie", "~> 0.0.4"
  spec.add_dependency "thermite", "~> 0"

  spec.extensions << "ext/Rakefile"
end
# rubocop:enable Metrics/BlockLength
