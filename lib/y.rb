# frozen_string_literal: true

require "rutie"

module Y
  Rutie.new(:y_rb).init(
    "Init_yrb",
    __dir__
  )

  def apply_update(doc, diff); end

  def encode_state_vector(doc); end

  def encode_state_as_update(doc, state_vector); end
end
