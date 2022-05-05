# frozen_string_literal: true

require "rutie"
require_relative "array"
require_relative "doc"
require_relative "map"
require_relative "text"
require_relative "transaction"
require_relative "version"
require_relative "xml"

module Y
  Rutie.new(:y_rb).init(
    "Init_yrb",
    File.join(__dir__, "..")
  )
end
