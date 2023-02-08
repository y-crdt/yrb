# frozen_string_literal: true

module Y
  # A text can be used insert and remove string fragments. It also supports
  # formatting and the concept of embeds, which are supported data types that
  # added as metadata.
  #
  # The text is the replicated counterpart to a String. It supports a subset
  # of String operations, like appending, insert at position and slicing.
  #
  # Someone should not instantiate a text directly, but use {Y::Doc#get_text}
  # instead.
  #
  # @example
  #   doc = Y::Doc.new
  #   text = doc.get_text("my text")
  #
  #   text << "Hello, World!"
  #   puts text.to_s
  class Text
    # @!attribute [r] document
    #
    # @return [Y::Doc] The document this text belongs to
    attr_accessor :document

    # Create a new text instance
    #
    # @param doc [Y::Doc]
    def initialize(doc = nil)
      @document = doc || Y::Doc.new

      super()
    end

    # Appends a string at the end of the text
    #
    # @param str [String]
    # @return [void]
    def <<(str)
      document.current_transaction { |tx| ytext_push(tx, str) }
    end

    # Attach listener to text changes
    #
    # @example Listen to changes in text type
    #   local = Y::Doc.new
    #
    #   text = local.get_text("my text")
    #   text.attach(->(delta) { pp delta }) # { insert: "Hello, World!" }
    #
    #   local.transact do
    #     text << "Hello, World!"
    #   end
    #
    # @example Listen to changes in text type
    #   local = Y::Doc.new
    #
    #   text = local.get_text("my text")
    #   text.attach(->(delta) { pp delta }) # { insert: "Hello, World!" }
    #
    #   text << "Hello, World!"
    #
    #   # todo: required, otherwise segfault
    #   local.commit
    #
    # @param callback [Proc]
    # @param block [Block]
    # @return [Integer]
    def attach(callback, &block)
      return ytext_observe(callback) unless callback.nil?

      ytext_observe(block.to_proc) unless block.nil?
    end

    # Detach listener
    #
    # @param subscription_id [Integer]
    # @return [void]
    def detach(subscription_id)
      ytext_unobserve(subscription_id)
    end

    # Checks if text is empty
    #
    # @example Check if text is empty
    #   doc = Y::Doc.new
    #   text = doc.get_text("my text")
    #
    #   text.empty? # true
    #
    # @return [TrueClass,FalseClass]
    def empty?
      length.zero?
    end

    # rubocop:disable Metrics/MethodLength

    # Insert a value at position and with optional attributes. This method is
    # similar to [String#insert](https://ruby-doc.org/core-3.1.2/String.html),
    # except for the optional third `attrs` argument.
    #
    # @example Insert a string at position
    #   doc = Y::Doc.new
    #   text = doc.get_text("my text")
    #   text << "Hello, "
    #
    #   text.insert(7, "World!")
    #
    #   puts text.to_s == "Hello, World!" # true
    #
    # The value can be any of the supported types:
    # - Boolean
    # - String
    # - Numeric
    # - Array (where element types must be supported)
    # - Hash (where the the types of key and values must be supported)
    #
    # @param index [Integer]
    # @param value [String, Numeric, Array, Hash]
    # @param attrs [Hash, nil]
    # @return [void]
    def insert(index, value, attrs = nil)
      document.current_transaction do |tx|
        if value.is_a?(String)
          ytext_insert(tx, index, value) if attrs.nil?
          unless attrs.nil?
            ytext_insert_with_attributes(tx, index, value, attrs)
          end
          return nil
        end

        if can_insert?(value)
          ytext_insert_embed(tx, index, value) if attrs.nil?
          unless attrs.nil?
            ytext_insert_embed_with_attributes(tx, index, value, attrs)
          end
          return nil
        end

        raise ArgumentError,
              "Can't insert value. `#{value.class.name}` isn't supported."
      end
    end

    # rubocop:enable Metrics/MethodLength

    # Applies formatting to text
    #
    # @example Add formatting to first word
    #   doc = Y::Doc.new
    #   text = doc.get_text("my text")
    #
    #   attrs = {format: "bold"}
    #   text.format(0, 2, attrs)
    #
    # @param index [Integer]
    # @param length [Integer]
    # @param attrs [Hash]
    # @return [void]
    def format(index, length, attrs)
      document.current_transaction do |tx|
        ytext_format(tx, index, length, attrs)
      end
    end

    # Returns length of text
    #
    # @return [Integer] Length of text
    def length
      document.current_transaction { |tx| ytext_length(tx) }
    end

    alias size length

    # rubocop:disable Metrics/AbcSize, Metrics/CyclomaticComplexity, Metrics/MethodLength

    # Removes a part from text
    #
    # **Attention:** In comparison to String#slice, {Text#slice!} will not
    # return the substring that gets removed. Even this being technically
    # possible, it requires us to read the substring before removing it, which
    # is not desirable in most situations.
    #
    # @example Removes a single character
    #   doc = Y::Doc.new
    #
    #   text = doc.get_text("my text")
    #   text << "Hello"
    #
    #   text.slice!(0)
    #
    #   text.to_s == "ello" # true
    #
    # @example Removes a range of characters
    #   doc = Y::Doc.new
    #
    #   text = doc.get_text("my text")
    #   text << "Hello"
    #
    #   text.slice!(1..2)
    #   text.to_s == "Hlo" # true
    #
    #   text.slice!(1...2)
    #   text.to_s == "Ho" # true
    #
    # @example Removes a range of chars from start and for given length
    #   doc = Y::Doc.new
    #
    #   text = doc.get_text("my text")
    #   text << "Hello"
    #
    #   text.slice!(0, 3)
    #
    #   text.to_s == "lo" # true
    #
    # @overload slice!(index)
    #   Removes a single character at index
    #
    # @overload slice!(start, length)
    #   Removes a range of characters
    #
    # @overload slice!(range)
    #   Removes a range of characters
    #
    # @return [void]
    def slice!(*args)
      document.current_transaction do |tx|
        if args.empty?
          raise ArgumentError,
                "Provide one of `index`, `range`, `start, length` as arguments"
        end

        if args.size == 1
          arg = args.first

          if arg.is_a?(Range)
            ytext_remove_range(tx, arg.first, arg.last - arg.first)
            return nil
          end

          if arg.is_a?(Numeric)
            ytext_remove_range(tx, arg.to_int, 1)
            return nil
          end
        end

        if args.size == 2
          start, length = args

          if start.is_a?(Numeric) && length.is_a?(Numeric)
            ytext_remove_range(tx, start, length)
            return nil
          end
        end

        raise ArgumentError, "Please check your arguments, can't slice."
      end
    end

    # rubocop:enable Metrics/AbcSize, Metrics/CyclomaticComplexity, Metrics/MethodLength

    # Returns string representation of text
    #
    # @example
    #   doc = Y::Doc.new
    #   text = doc.get_text("my text")
    #   text << "Hello"
    #
    #   puts text.to_s # "Hello"
    #
    # @return [String]
    def to_s
      document.current_transaction { |tx| ytext_to_s(tx) }
    end

    private

    def can_insert?(value)
      value.is_a?(NilClass) ||
        value.is_a?(Symbol) ||
        [true, false].include?(value) ||
        value.is_a?(Numeric) ||
        value.is_a?(Enumerable) ||
        value.is_a?(Hash)
    end

    # @!method ytext_insert(tx, index, chunk)
    #   Insert into text at position
    #
    # @param transaction [Y::Transaction]
    # @param index [Integer]
    # @param chunk [String]
    # @return [nil]

    # @!method ytext_insert_embed(tx, index, content)
    #   Insert into text at position
    #
    # @param tx [Y::Transaction]
    # @param index [Integer]
    # @param content [Y::Text|Y::Array|Y::Map]
    # @return [nil]

    # @!method ytext_insert_embed_with_attributes(tx, index, embed, attrs)
    #   Insert into text at position
    #
    # @param tx [Y::Transaction]
    # @param index [Integer]
    # @param embed [Y::Text, Y::Array, Y::Map]
    # @param attrs [Hash]
    # @return [nil]

    # @!method ytext_insert_with_attributes(tx, index, chunk, attrs)
    #   Insert into text at position
    #
    # @param tx [Y::Transaction]
    # @param index [Integer]
    # @param chunk [String]
    # @param attrs [Hash]
    # @return [nil]

    # @!method ytext_push(tx, value)
    #   Returns length of text
    #
    # @param tx [Y::Transaction]
    # @param value [String]
    # @return [nil]

    # @!method ytext_remove_range(tx, index, length)
    #   Removes a range from text
    #
    # @param tx [Y::Transaction]
    # @param index [Integer]
    # @param length [Integer]
    # @return [nil]

    # @!method ytext_format(tx, index, length, attrs)
    #   Formats a text range
    #
    # @param tx [Y::Transaction]
    # @param index [Integer]
    # @param length [Integer]
    # @param attrs [Hash]
    # @return [nil]

    # @!method ytext_length(tx)
    #   Returns length of text
    #
    # @param tx [Y::Transaction]
    # @return [Integer]

    # @!method ytext_observe(proc)
    #   Observe text changes
    #
    # @param proc [Proc]
    # @return [Integer]

    # @!method ytext_to_s()
    #   Returns string representation of text
    #
    # @return [String]

    # @!method ytext_unobserve(subscription_id)
    #   Detach listener
    #
    # @param subscription_id [Integer]
    # @return [void]
  end
end
