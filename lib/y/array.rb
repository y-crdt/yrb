# frozen_string_literal: true

module Y
  # rubocop:disable Lint/EmptyClass
  class Array
    # @!method length
    #   Returns length of array
    #
    #   @return [Integer] Length of array

    # @!method insert(transaction, index, content)
    #   Inserts content as specified index
    #   @param [Y::Transaction] transaction
    #   @param [Integer] index
    #   @param [Float, Integer, Array, Hash, Text] content
    #   @return [void]

    # @!method remove(transaction, index)
    #   Removes a single element from array at index
    #
    #   @param [Y::Transaction] transaction
    #   @param [Integer] index
    #   @return [void]

    # @!method remove_range(transaction, index, length)
    #   Removes a range of elements from array
    #
    #   @param [Y::Transaction] transaction
    #   @param [Integer] index
    #   @param [Integer] length
    #   @return [void]

    # @!method to_arr
    #   Transforms the array into a Ruby array
    #
    #   @return [Array]
  end
  # rubocop:enable Lint/EmptyClass
end
