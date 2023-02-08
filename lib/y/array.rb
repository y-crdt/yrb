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
  class Array # rubocop:disable Metrics/ClassLength
    include Enumerable

    # @!attribute [r] document
    #
    # @return [Y::Doc] The document this array belongs to
    attr_accessor :document

    # Create a new array instance
    #
    # @param doc [Y::Doc]
    def initialize(doc = nil)
      @document = doc || Y::Doc.new

      super()
    end

    # Retrieves element at position
    #
    # @return [true|false|Float|Integer|String|Array|Hash]
    def [](index)
      document.current_transaction { |tx| yarray_get(tx, index) }
    end

    # Inserts value at position
    #
    # @param index [Integer]
    # @param value [true|false|Float|Integer|String|Array|Hash]
    # @return [void]
    def []=(index, value)
      document.current_transaction { |tx| yarray_insert(tx, index, value) }
    end

    # Adds an element to the end of the array
    #
    # @param value [true|false|Float|Integer|String|::Array|Hash]
    # @return [void]
    def <<(value, *values)
      document.current_transaction do |tx|
        yarray_push_back(tx, value)
        values.each { |v| yarray_push_back(tx, v) }
      end
    end

    # Attach listener to array changes
    #
    # @example Listen to changes in array type
    #   local = Y::Doc.new
    #
    #   arr = local.get_array("my array")
    #   arr.attach { |delta| pp delta }
    #
    #   local.transact do
    #     arr << 1
    #   end
    #
    # @param block [Block]
    # @return [Integer]
    def attach(&block)
      raise "provide block" unless block

      yarray_observe(block.to_proc)
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
    # @param other_arrays [Array<Array<Object>>]
    # @return [void]
    def concat(*other_arrays)
      document.current_transaction do |tx|
        combined = other_arrays.reduce([]) do |values, arr|
          values.concat(arr) if arr.is_a?(::Array)
        end

        yarray_insert_range(tx, yarray_length(tx), combined)
      end
    end

    # Detach listener
    #
    # @param subscription_id [Integer]
    # @return [void]
    def detach(subscription_id)
      yarray_unobserve(subscription_id)
    end

    # @return [void]
    def each(&block)
      document.current_transaction { |tx| yarray_each(tx, &block) }
    end

    # Check if the array is empty
    #
    # @return [true|false]
    def empty?
      size.zero?
    end

    # Returns first element in array if there is at least one
    #
    # @return [true|false|Float|Integer|String|::Array|Hash|nil]
    def first
      document.current_transaction { |tx| yarray_get(tx, 0) }
    end

    # Returns last element in array if there is at least one element
    #
    # @return [true|false|Float|Integer|String|::Array|Hash|nil]
    def last
      document.current_transaction do |tx|
        len = yarray_length(tx)
        return yarray_get(tx, len - 1) if len.positive?

        nil
      end
    end

    # rubocop:disable Naming/MethodParameterName

    # Removes last (n) element(s) from array
    #
    # @param n [Integer|nil] Number of elements to remove
    # @return [void]
    def pop(n = nil)
      document.current_transaction do |tx|
        len = yarray_length(tx)
        yarray_remove(tx, len - 1) if n.nil?
        yarray_remove_range(tx, len - n, n) unless n.nil?
      end
    end

    # rubocop:enable Naming/MethodParameterName

    alias push <<

    # rubocop:disable Naming/MethodParameterName

    # Removes first (n) element(s) from array
    #
    # @param n [Integer|nil] Number of elements to remove
    # @return [void]
    def shift(n = nil)
      document.current_transaction do |tx|
        yarray_remove(tx, 0) if n.nil?

        yarray_remove_range(tx, 0, n) unless nil?
      end
    end

    # rubocop:enable Naming/MethodParameterName

    # Size of array
    #
    # @return [Integer]
    def size
      document.current_transaction { |tx| yarray_length(tx) }
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
      document.current_transaction do |tx| # rubocop:disable Metrics/BlockLength
        if args.empty?
          raise ArgumentError,
                "Provide one of `index`, `range`, `start, length` as arguments"
        end

        if args.size == 1
          arg = args.first

          if arg.is_a?(Range)
            if arg.exclude_end?
              yarray_remove_range(tx, arg.first,
                                  arg.last - arg.first)
            end
            unless arg.exclude_end?
              yarray_remove_range(tx, arg.first,
                                  arg.last + 1 - arg.first)
            end
            return nil
          end

          if arg.is_a?(Numeric)
            yarray_remove(tx, arg.to_int)
            return nil
          end
        end

        if args.size == 2
          first, second = args

          if first.is_a?(Numeric) && second.is_a?(Numeric)
            yarray_remove_range(tx, first, second)
            return nil
          end
        end

        raise ArgumentError, "Please check your arguments, can't slice."
      end
    end

    # rubocop:enable Metrics/AbcSize, Metrics/CyclomaticComplexity, Metrics/MethodLength, Metrics/PerceivedComplexity

    # Convert this array to a Ruby Array
    #
    # @return [Array<true|false|Float|Integer|String|::Array|Hash>]
    def to_a
      document.current_transaction { |tx| yarray_to_a(tx) }
    end

    # Adds an element to the beginning of the array
    #
    # @return [void]
    def unshift(value)
      document.current_transaction { |tx| yarray_push_front(tx, value) }
    end

    alias prepend unshift

    # @!method yarray_each(proc)
    #   Iterates over all elements in Array by calling the provided proc
    #   with the value as argument.
    #
    # @param proc [Proc<Object>] A proc that is called for every element
    # @!visibility private

    # @!method yarray_get(transaction, index)
    #   Retrieves content as specified index
    #
    # @param index [Integer]
    # @return [Object]
    # @!visibility private

    # @!method yarray_insert(transaction, index, content)
    #   Inserts content at specified index
    #
    # @param transaction [Y::Transaction]
    # @param index [Integer]
    # @param content [Boolean, Float, Integer, Array, Hash, Text]
    # @return [void]
    # @!visibility private

    # @!method yarray_insert_range(transaction, index, arr)
    #   Inserts all elements of a given array at specified index
    #
    # @param transaction [Y::Transaction]
    # @param index [Integer]
    # @param arr [Array<Boolean|Float|Integer|Array|Hash|Text>]
    # @return [void]
    # @!visibility private

    # @!method yarray_length(transaction)
    #   Returns length of array
    #
    # @param transaction [Y::Transaction]
    # @return [Integer] Length of array
    # @!visibility private

    # @!method yarray_push_back(transaction, value)
    #   Adds an element to the end of the array
    #
    # @param transaction [Y::Transaction]
    # @param value [Object]
    # @return [void]
    # @!visibility private

    # @!method yarray_push_front(transaction, value)
    #   Adds an element to the front of the array
    #
    # @param transaction [Y::Transaction]
    # @param value [Object]
    # @return [void]
    # @!visibility private

    # @!method yarray_observe(proc)
    #
    # @param proc [Proc]
    # @return [Integer]
    # @!visibility private

    # @!method yarray_remove(transaction, index)
    #   Removes a single element from array at index
    #
    # @param transaction [Y::Transaction]
    # @param index [Integer]
    # @return [void]
    # @!visibility private

    # @!method yarray_remove_range(transaction, index, length)
    #   Removes a range of elements from array
    #
    # @param transaction [Y::Transaction]
    # @param index [Integer]
    # @param length [Integer]
    # @return [void]
    # @!visibility private

    # @!method yarray_to_a(transaction)
    #   Transforms the array into a Ruby array
    # @param transaction [Y::Transaction]
    # @return [Array]
    # @!visibility private

    # @!method yarray_unobserve(subscription_id)
    #
    # @param subscription_id [Integer]
    # @return [void]
    # @!visibility private
  end
end
