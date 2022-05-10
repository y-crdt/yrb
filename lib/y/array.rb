# frozen_string_literal: true

module Y
  # An array can be used to store and retrieve elements.
  #
  # The array is the replicated counterpart to a Ruby Array. It supports a
  # subset of the Ruby Array operations, like adding, getting and deleting
  # values by position or ranges.
  #
  # Someone should not instantiate an array directly, but use {Y::Doc#get_array}
  # instead.
  #
  # @example
  #   doc = Y::Doc.new
  #   array = doc.get_array("my array")
  #
  #   array << 1
  #   array.push(2)
  #   array.concat([3, 4, 5])
  #
  #   array.to_a == [1, 2, 3, 4, 5] # true
  class Array
    # @!attribute [r] document
    #
    # @return [Y::Doc] The document this array belongs to
    attr_accessor :document

    # Create a new array instance
    #
    # @param [Y::Doc] doc
    def initialize(doc = nil)
      @document = doc || Y::Doc.new

      super()
    end

    # Retrieves element at position
    #
    # @return [Object]
    def [](index)
      yarray_get(index)
    end

    # Adds an element to the end of the array
    #
    # @return [void]
    def <<(value)
      yarray_push_back(transaction, value)
    end

    # Adds to array all elements from each Array in `other_arrays`.
    #
    # If one of the arguments isn't an Array, it is silently ignored.
    #
    # @example Add multiple values to array
    #   doc = Y::Doc.new
    #   arr = doc.get_array("my array")
    #   arr.concat([1, 2, 3])
    #
    #   arr.to_a == [1, 2, 3] # true
    #
    # @param [Array<Array<Object>>] other_arrays
    # @return [void]
    def concat(*other_arrays)
      combined = other_arrays.reduce([]) do |values, arr|
        values.concat(arr) if arr.is_a?(::Array)
      end

      yarray_insert_range(transaction, size, combined)
    end

    # @return [void]
    def each(&block)
      yarray_each(block)
    end

    # Check if the array is empty
    #
    # @return [true|false]
    def empty?
      size.zero?
    end

    # Returns first element in array if there is at least one
    #
    # @return [Object|nil]
    def first
      yarray_get(0)
    end

    # Returns last element in array if there is at least one element
    #
    # @return [Object|nil]
    def last
      len = yarray_length
      return yarray_get(yarray_length - 1) if len.positive?

      nil
    end

    # rubocop:disable Naming/MethodParameterName

    # Removes last (n) element(s) from array
    #
    # @param [Integer|nil] n Number of elements to remove
    # @return [void]
    def pop(n = nil)
      len = size
      yarray_remove(transaction, len - 1) if n.nil?
      yarray_remove_range(transaction, len - n, n) unless n.nil?
    end

    # rubocop:enable Naming/MethodParameterName

    alias push <<

    # rubocop:disable Naming/MethodParameterName

    # Removes first (n) element(s) from array
    #
    # @param [Integer|nil] n Number of elements to remove
    # @return [void]
    def shift(n = nil)
      yarray_remove(transaction, 0) if n.nil?
      yarray_remove_range(transaction, 0, n) unless nil?
    end

    # rubocop:enable Naming/MethodParameterName

    # Size of array
    #
    # @return [Integer]
    def size
      yarray_length
    end

    alias length size

    # rubocop:disable Metrics/AbcSize, Metrics/CyclomaticComplexity, Metrics/MethodLength, Metrics/PerceivedComplexity

    # Removes one or more elements from array
    #
    # **Attention:** In comparison to Array#slice, {Array#slice!} will not
    # return the values that got removed. Even this being technically
    # possible, it requires us to read the elements before removing them, which
    # is not desirable in most situations.
    #
    # @example Removes a single element
    #   doc = Y::Doc.new
    #
    #   arr = doc.get_text("my array")
    #   arr << 1
    #   arr << 2
    #   arr << 3
    #
    #   arr.slice!(1)
    #
    #   arr.to_a == [1, 3] # true
    #
    # @overload slice!(n)
    #   Removes nth element from array
    #
    # @overload slice!(start, length)
    #   Removes a range of elements
    #
    # @overload slice!(range)
    #   Removes a range of elements
    #
    # @return [void]
    def slice!(*args)
      if args.size.zero?
        raise ArgumentError,
              "Provide one of `index`, `range`, `start, length` as arguments"
      end

      if args.size == 1
        arg = args.first

        if arg.is_a?(Range)
          if arg.exclude_end?
            yarray_remove_range(transaction, arg.first,
                                arg.last - arg.first)
          end
          unless arg.exclude_end?
            yarray_remove_range(transaction, arg.first,
                                arg.last + 1 - arg.first)
          end
          return nil
        end

        if arg.is_a?(Numeric)
          yarray_remove(transaction, arg.to_int)
          return nil
        end
      end

      if args.size == 2
        first, second = args

        if first.is_a?(Numeric) && second.is_a?(Numeric)
          yarray_remove_range(transaction, first, second)
          return nil
        end
      end

      raise ArgumentError, "Please check your arguments, can't slice."
    end

    # rubocop:enable Metrics/AbcSize, Metrics/CyclomaticComplexity, Metrics/MethodLength, Metrics/PerceivedComplexity

    # Convert this array to a Ruby Array
    #
    # @return [Array<Object>]
    def to_a
      yarray_to_a
    end

    # Adds an element to the beginning of the array
    #
    # @return [void]
    def unshift(value)
      yarray_push_front(transaction, value)
    end

    alias prepend unshift

    private

    # @!method yarray_each(proc)
    #   Iterates over all elements in Array by calling the provided proc
    #   with the value as argument.
    #
    # @param [Proc<Object>] proc A proc that is called for every element

    # @!method yarray_get(index)
    #   Retrieves content as specified index
    #
    # @param [Integer] index
    # @return [Object]

    # @!method yarray_insert(transaction, index, content)
    #   Inserts content at specified index
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [Boolean, Float, Integer, Array, Hash, Text] content
    # @return [void]

    # @!method yarray_insert_range(transaction, index, arr)
    #   Inserts all elements of a given array at specified index
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [Array<Boolean, Float, Integer, Array, Hash, Text>] arr
    # @return [void]

    # @!method yarray_length
    #   Returns length of array
    #
    # @return [Integer] Length of array

    # @!method yarray_push_back(transaction, value)
    #   Adds an element to the end of the array
    #
    # @param [Y::Transaction] transaction
    # @param [Object] value
    # @return [void]

    # @!method yarray_push_front(transaction, value)
    #   Adds an element to the front of the array
    #
    # @param [Y::Transaction] transaction
    # @param [Object] value
    # @return [void]

    # @!method yarray_remove(transaction, index)
    #   Removes a single element from array at index
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @return [void]

    # @!method yarray_remove_range(transaction, index, length)
    #   Removes a range of elements from array
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [Integer] length
    # @return [void]

    # @!method yarray_to_a
    #   Transforms the array into a Ruby array
    #
    # @return [Array]

    # A reference to the current active transaction of the document this map
    # belongs to.
    #
    # @return [Y::Transaction] A transaction object
    def transaction
      document.current_transaction
    end
  end
end
