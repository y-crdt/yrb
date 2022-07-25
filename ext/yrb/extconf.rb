# frozen_string_literal: true

require "mkmf"
require "rb_sys/mkmf"

create_rust_makefile("yrb") do |r|
  if r.target&.include? "musl"
    r.extra_rustflags = %w[-C target-feature=-crt-static]
  end
end
