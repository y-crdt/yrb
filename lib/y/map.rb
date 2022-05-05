# frozen_string_literal: true

require "json"

module Y
  class Map
    # Returns JSON representation of map
    # @return [String] JSON string
    def to_json(*_args)
      to_h.to_json
    end
  end
end
