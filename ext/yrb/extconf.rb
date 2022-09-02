# frozen_string_literal: true

require "mkmf"
require "rb_sys/mkmf"

RbConfig::CONFIG["CC"] = RbConfig::MAKEFILE_CONFIG["CC"] = ENV["CC"] if ENV["CC"]
ENV["CC"] = RbConfig::CONFIG["CC"]

create_rust_makefile("yrb")
