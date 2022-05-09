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
    # @param [Y::Doc] doc
    def initialize(doc = nil)
      @document = doc || Y::Doc.new

      super()
    end

    # Appends a string at the end of the text
    #
    # @return [void]
    def <<(str)
      ytext_push(transaction, str)
    end

    # Checks if text is empty
    #
    # @example Check if text is empty
    #   doc = Y::Doc.new
    #   text = doc.get_text("my text")
    #
    #   text.empty? # true
    #
    # @return [true|false]
    def empty?
      length.zero?
    end

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
    # @param [Integer] index
    # @param [String, Float, Array, Hash] value
    # @param [Hash|nil] attrs
    # @return [void]
    def insert(index, value, attrs = nil)
      if value.is_a?(String)
        ytext_insert(transaction, index, value) if attrs.nil?
        unless attrs.nil?
          ytext_insert_with_attrs(transaction, index, value,
                                  attrs)
        end
        return nil
      end

      if can_insert?(value)
        ytext_insert_embed(transaction, index, value) if attrs.nil?
        unless attrs.nil?
          ytext_insert_embed_with_attrs(transaction, index, value,
                                        attrs)
        end
        return nil
      end

      raise ArgumentError,
            "Can't insert value. `#{value.class.name}` isn't supported."
    end

    # Applies formatting to text
    #
    # @example Add formatting to first word
    #   doc = Y::Doc.new
    #   text = doc.get_text("my text")
    #
    #   attrs = {format: "bold"}
    #   text.format(0, 2, attrs)
    #
    # @param [Integer] index
    # @param [Integer] length
    # @param [Hash] attrs
    # @return [void]
    def format(index, length, attrs)
      ytext_format(transaction, index, length, attrs)
    end

    # Returns length of text
    #
    # @return [Integer] Length of text
    def length
      ytext_length
    end

    alias size length

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
      if args.size.zero?
        raise ArgumentError,
              "Provide one of `index`, `range`, `start, length` as arguments"
      end

      if args.size == 1
        arg = args.first

        if arg.is_a?(Range)
          ytext_remove_range(transaction, arg.first, arg.last - arg.first)
          return nil
        end

        if arg.is_a?(Numeric)
          ytext_remove_range(transaction, arg.to_int, 1)
          return nil
        end
      end

      if args.size == 2
        first, second = args

        if first.is_a?(Numeric) && second.is_a?(Numeric)
          ytext_remove_range(transaction, first, second)
          return nil
        end
      end

      raise ArgumentError, "Please check your arguments, can't slice."
    end

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
      ytext_to_s
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

    # @!method ytext_insert(transaction, index, chunk)
    #   Insert into text at position
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [String] chunk
    # @return [nil]

    # @!method ytext_insert_embed(transaction, index, content)
    #   Insert into text at position
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [Y::Text, Y::Array, Y::Map] content
    # @return [nil]

    # @!method ytext_insert_embed_with_attrs(transaction, index, embed, attrs)
    #   Insert into text at position
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [Y::Text, Y::Array, Y::Map] embed
    # @param [Hash] attrs
    # @return [nil]

    # @!method ytext_insert_with_attrs(transaction, index, chunk, attrs)
    #   Insert into text at position
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [String] chunk
    # @param [Hash] attrs
    # @return [nil]

    # @!method ytext_push(transaction, value)
    #   Returns length of text
    #
    # @param [Y::Transaction] transaction
    # @param [String] value
    # @return [nil]

    # @!method ytext_remove_range(transaction, index, length)
    #   Removes a range from text
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [Integer] length
    # @return [nil]

    # @!method ytext_format(transaction, index, length, attrs)
    #   Formats a text range
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [Integer] length
    # @param [Hash] attrs
    # @return [nil]

    # @!method ytext_length()
    #   Returns length of text
    #
    # @return [Integer]

    # @!method ytext_to_s()
    #   Returns string representation of text
    #
    # @return [String]

    # A reference to the current active transaction of the document this map
    # belongs to.
    #
    # @return [Y::Transaction] A transaction object
    def transaction
      document.current_transaction
    end
  end
end
