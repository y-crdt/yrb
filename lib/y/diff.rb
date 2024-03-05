# frozen_string_literal: true

module Y
  # A representation of an uniformly-formatted chunk of rich context stored by
  # Text or XmlText. It contains a value (which could be a string, embedded
  # object or another shared type) with optional formatting attributes wrapping
  # around this chunk.
  class Diff
    # @return [Object]
    def insert
      ydiff_insert
    end

    # @return [Hash]
    def attrs
      ydiff_attrs
    end

    # Convert the diff to a Hash representation
    #
    # @return [Hash]
    def to_h
      {
        insert: ydiff_insert,
        attrs: ydiff_attrs
      }
    end

    # @!method ydiff_insert()
    #   Returns string representation of text
    #
    # @return [Object]

    # @!method ydiff_attrs()
    #
    # @return [Hash]
  end
end
