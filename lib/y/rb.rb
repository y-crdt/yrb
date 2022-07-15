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
  # support pre-built and local built libraries
  lib_path = if Dir.exist?(File.join(__dir__, "..", "..", "target", "release"))
               nil
             else
               File.join(__dir__, "..", "..")
             end

  Rutie.new(
    :y_rb,
    lib_path: lib_path
  ).init(
    "Init_yrb",
    File.join(__dir__, "..")
  )
end
