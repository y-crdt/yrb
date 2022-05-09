# frozen_string_literal: true

module Y
  # rubocop:disable Lint/EmptyClass
  class Text
    # @!method insert(transaction, index, chunk)
    #   Insert into text at position
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [String] chunk
    # @return [nil]

    # @!method insert_embed(transaction, index, content)
    #   Insert into text at position
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [Y::Text, Y::Array, Y::Map] content
    # @return [nil]

    # @!method insert_embed_with_attributes(transaction, index, embed, attrs)
    #   Insert into text at position
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [Y::Text, Y::Array, Y::Map] embed
    # @param [Hash] attrs
    # @return [nil]

    # @!method insert_with_attributes(transaction, index, chunk, attrs)
    #   Insert into text at position
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [String] chunk
    # @param [Hash] attrs
    # @return [nil]

    # @!method length
    #   Returns length of text
    #
    # @return [Integer] Length of text

    # @!method push(transaction, value)
    #   Returns length of text
    #
    # @param [Y::Transaction] transaction
    # @param [String] value
    # @return [nil]

    # @!method remove_range(transaction, index, length)
    #   Removes a range from text
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [Integer] length
    # @return [nil]

    # @!method format(transaction, index, length, attrs)
    #   Formats a text range
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [Integer] length
    # @param [Hash] attrs
    # @return [nil]

    # @!method to_s
    #   Prints string representation of Text
    #
    # @return [String]
  end
  # rubocop:enable Lint/EmptyClass
end
