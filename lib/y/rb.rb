# frozen_string_literal: true

require "rutie"
require_relative "version"
require_relative "doc"

module Y
  Rutie.new(:y_rb).init(
    "Init_yrb",
    File.join(__dir__, "..")
  )
end
