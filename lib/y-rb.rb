# frozen_string_literal: true

# load native extension
begin
  ruby_version = /(\d+\.\d+)/.match(::RUBY_VERSION)
  require_relative "#{ruby_version}/yrb"
rescue LoadError
  require "yrb"
end

require_relative "y/array"
require_relative "y/awareness"
require_relative "y/doc"
require_relative "y/map"
require_relative "y/text"
require_relative "y/xml"
require_relative "y/transaction"
require_relative "y/version"

module Y
end
