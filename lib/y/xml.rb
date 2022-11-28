# frozen_string_literal: true

module Y
  # rubocop:disable Metrics/ClassLength

  # A XMLElement
  #
  # Someone should not instantiate an element directly, but use
  # {Y::Doc#get_xml_element} instead
  #
  # @example
  #   doc = Y::Doc.new
  #   xml_element = doc.get_xml_element("my xml")
  #
  #   puts xml_element.to_s
  class XMLElement
    # @!attribute [r] document
    #
    # @return [Y::Doc] The document this array belongs to
    attr_accessor :document

    # Create a new XMLElement instance
    #
    # @param doc [Y::Doc]
    def initialize(doc = nil)
      @document = doc || Y::Doc.new

      super()
    end

    # Retrieve node at index
    #
    # @param index [Integer]
    # @return [Y::XMLElement|nil]
    def [](index)
      node = document.current_transaction { |tx| yxml_element_get(tx, index) }
      node&.document = document
      node
    end

    # Create a node at index
    #
    # @param index [Integer]
    # @param name [String] Name of node, e.g. `<p />`
    # @return [Y::XMLElement]
    # rubocop:disable Lint/Void
    def []=(index, name)
      node = document.current_transaction do |tx|
        yxml_element_insert_element(tx, index, name)
      end
      node.document = document
      node
    end
    # rubocop:enable Lint/Void

    # Returns first child in list or nil if no child exists
    #
    # @return [Hash]
    def attrs
      document.current_transaction { |tx| yxml_element_attributes(tx) }
    end

    alias attributes attrs

    # Returns first child in list or nil if no child exists
    #
    # @return [Y::XMLElement]
    def first_child
      child = document.current_transaction { |tx| yxml_element_first_child(tx) }
      child&.document = document
      child
    end

    # Insert text into element at given index
    #
    # Optional input is pushed to the text if provided
    #
    # @param index [Integer]
    # @param input [String|nil]
    # @return [Y::XMLText]
    def insert_text(index, input = "")
      text = document.current_transaction do |tx|
        yxml_element_insert_text(tx, index, input)
      end
      text.document = document
      text
    end

    # Retrieve element or text adjacent (next) to this element
    #
    # @return [Y::XMLElement|Y::XMLText|nil]
    def next_sibling
      node = document.current_transaction { |tx| yxml_element_next_sibling(tx) }
      node&.document = document
      node
    end

    # Attach listener to get notified about changes to the element
    #
    # This supports either a `Proc` or a `Block`.
    #
    # @example Receive changes via Proc
    #   doc = Y::Doc.new
    #   xml_element = doc.get_xml_element("my xml element")
    #   xml_element.attach ->(changes) { … }
    #
    # @example Receive changes via Block
    #   doc = Y::Doc.new
    #   xml_element = doc.get_xml_element("my xml element")
    #   xml_element.attach { |changes| … }
    #
    # @param callback [Proc]
    # @param block [Block]
    # @return [Integer] The subscription ID
    def attach(callback = nil, &block)
      return yxml_element_observe(callback) unless callback.nil?

      yxml_element_observe(block.to_proc) unless block.nil?
    end

    # Retrieve parent element
    #
    # @return [Y::XMLElement|nil]
    def parent
      node = yxml_element_parent
      node.document = document
      node
    end

    # Retrieve element or text adjacent (previous) to this element
    #
    # @return [Y::XMLElement|Y::XMLText|nil]
    def prev_sibling
      node = document.current_transaction { |tx| yxml_element_prev_sibling(tx) }
      node&.document = document
      node
    end

    # Creates a new child an inserts at the end of the children list
    #
    # @param name [String]
    # @return [Y::XMLElement]
    def <<(name)
      xml_element = document.current_transaction do |tx|
        yxml_element_push_element_back(tx, name)
      end
      xml_element.document = document
      xml_element
    end

    alias push_child <<

    # Insert new text at the end of this elements child list
    #
    # The optional str argument initializes the text node with its value
    #
    # @param str [String]
    # @return [Y::XMLText]
    def push_text(str = "")
      text = document.current_transaction do |tx|
        yxml_element_push_text_back(tx, str)
      end
      text.document = document
      text
    end

    # Number of children
    #
    # @return [Integer]
    def size
      document.current_transaction { |tx| yxml_element_size(tx) }
    end

    # rubocop:disable Metrics/AbcSize, Metrics/CyclomaticComplexity, Metrics/MethodLength, Metrics/PerceivedComplexity

    # Removes one or more children from XML Element
    #
    # @example Removes a single element
    #   doc = Y::Doc.new
    #
    #   xml_element = doc.get_xml_element("my xml")
    #   xml_element << "A"
    #   xml_element << "B"
    #   xml_element << "C"
    #
    #   xml_element.slice!(1)
    #
    #   xml_element.to_s # <UNDEFINED><A></A><C></C></UNDEFINED>
    #
    # @overload slice!(n)
    #   Removes nth node from child list
    #
    # @overload slice!(start, length)
    #   Removes a range of nodes
    #
    # @overload slice!(range)
    #   Removes a range of nodes
    #
    # @return [void]
    def slice!(*args)
      document.current_transaction do |tx| # rubocop:disable Metrics/BlockLength
        if args.size.zero?
          raise ArgumentError,
                "Provide one of `index`, `range`, `start, length` as arguments"
        end

        if args.size == 1
          arg = args.first

          if arg.is_a?(Range)
            if arg.exclude_end?
              yxml_element_remove_range(tx, arg.first,
                                        arg.last - arg.first)
            end
            unless arg.exclude_end?
              yxml_element_remove_range(tx, arg.first,
                                        arg.last + 1 - arg.first)
            end
            return nil
          end

          if arg.is_a?(Numeric)
            yxml_element_remove_range(tx, arg.to_int, 1)
            return nil
          end
        end

        if args.size == 2
          first, second = args

          if first.is_a?(Numeric) && second.is_a?(Numeric)
            yxml_element_remove_range(tx, first, second)
            return nil
          end
        end

        raise ArgumentError, "Please check your arguments, can't slice."
      end
    end

    # rubocop:enable Metrics/AbcSize, Metrics/CyclomaticComplexity, Metrics/MethodLength, Metrics/PerceivedComplexity

    # Tag name
    #
    # @return [String]
    def tag
      yxml_element_tag
    end

    # String representation of this node and all its children
    #
    # @return [String]
    def to_s
      document.current_transaction { |tx| yxml_element_to_s(tx) }
    end

    # Detach a listener
    #
    # @param subscription_id [Integer]
    # @return [void]
    def detach(subscription_id)
      yxml_element_unobserve(subscription_id)
    end

    # Creates a new node and puts it in front of the child list
    #
    # @param name [String]
    # @return [Y::XMLElement]
    def unshift_child(name)
      xml_element = document.current_transaction do |tx|
        yxml_element_push_element_front(tx, name)
      end
      xml_element.document = document
      xml_element
    end

    # Insert new text at the front of this elements child list
    #
    # The optional str argument initializes the text node with its value
    #
    # @param str [String]
    # @return [Y::XMLText]
    def unshift_text(str = "")
      text = document.current_transaction do |tx|
        yxml_element_push_text_front(tx, str)
      end
      text.document = document
      text
    end

    # rubocop:disable Metrics/AbcSize, Metrics/MethodLength

    # make attributes just work on an element in the form of `attr_name` and
    # `attr_name=`
    #
    # @example Set and get an attribute
    #   doc = Y::Doc.new
    #   xml_element = doc.get_xml_element("my xml")
    #   xml_element.attr_name = "Hello"
    #
    #   puts xml_element.attr_name # "Hello"
    #
    # @!visibility private
    def method_missing(method_name, *args, &block)
      is_setter = method_name.to_s.end_with?("=")

      setter = method_name
      setter += "=" unless is_setter
      getter = method_name
      getter = getter.to_s.slice(0...-1)&.to_sym if is_setter

      define_singleton_method(setter.to_sym) do |new_val|
        document.current_transaction do |tx|
          yxml_element_insert_attribute(tx,
                                        method_name.to_s
                                                   .delete_suffix("=")
                                                   .delete_prefix("attr_"),
                                        new_val)
        end
      end

      define_singleton_method(getter) do
        document.current_transaction do |tx|
          yxml_element_get_attribute(tx,
                                     method_name.to_s.delete_prefix("attr_"))
        end
      end

      if is_setter
        value = args[0]
        send(setter, value)
      end
    rescue StandardError
      super(method_name, *args, &block)
    end

    # rubocop:enable Metrics/AbcSize, Metrics/MethodLength

    # Make sure we only respond to attributes
    # @!visibility private
    def respond_to_missing?(method_name, include_private = false)
      method_name.to_s.start_with?("attr_") || super
    end

    # @!method yxml_element_attributes
    #
    # @return [Hash]

    # @!method yxml_element_first_child(tx)
    #
    # @param tx [Y::Transaction]
    # @return [Y::XMLElement|Y::XMLText]

    # @!method yxml_element_get_attribute(tx, name)
    #
    # @param tx [Y::Transaction]
    # @param name [String]
    # @return [String|nil]

    # @!method yxml_element_get(tx, index)
    #
    # @param tx [Y::Transaction]
    # @param index [Integer]
    # @return [Y::XMLElement|Y::XMLText|nil]

    # @!method yxml_element_insert_attribute(tx, name, value)
    #
    # @param tx [Y::Transaction]
    # @param name [String]
    # @param value [String]
    # @return [String|nil]

    # @!method yxml_element_insert_element(tx, index, name)
    # Insert XML element into this XML element
    #
    # @!visibility private
    # @param tx [Y::Transaction]
    # @param index [Integer]
    # @param name [String]
    # @return [Y::XMLElement]

    # @!method yxml_element_insert_text(tx, index, text)
    #
    # @param tx [Y::Transaction]
    # @param index [Integer]
    # @param text [String]
    # @return [Y::XMLText]

    # @!method yxml_element_next_sibling(tx)
    #
    # @param tx [Y::Transaction]
    # @return [Y::XMLElement|XMLText|nil]

    # @!method yxml_element_observe(callback)
    #
    # @param callback [Proc]
    # @return [Integer] The subscription ID

    # @!method yxml_element_parent()
    #
    # @return [Y::XMLElement|nil]

    # @!method yxml_element_prev_sibling(tx)
    #
    # @param tx [Y::Transaction]
    # @return [Y::XMLElement|XMLText|nil]

    # @!method yxml_element_push_element_back(tx, name)
    #
    # @param tx [Y::Transaction]
    # @param name [String]
    # @return [Y::XMLElement]

    # @!method yxml_element_push_element_front(tx, name)
    #
    # @param tx [Y::Transaction]
    # @param name [String]
    # @return [Y::XMLElement]

    # @!method yxml_element_push_text_back(tx, text)
    #
    # @param tx [Y::Transaction]
    # @param text [string]
    # @return [Y::XMLText]

    # @!method yxml_element_push_text_front(tx, text)
    #
    # @param tx [Y::Transaction]
    # @param text [string]
    # @return [Y::XMLText]

    # @!method yxml_element_remove_attribute(tx, name)
    #
    # @param tx [Y::Transaction]
    # @param name [String] name
    # @return [void]

    # @!method yxml_element_remove_range(tx, index, length)
    #
    # @param tx [Y::Transaction]
    # @param index [Integer]
    # @param length [Integer]
    #
    # @return [void]

    # @!method yxml_element_size(tx)
    #
    # @param tx [Y::Transaction]
    # @return [Integer]

    # @!method yxml_element_tag
    #
    # @return [String]

    # @!method yxml_element_to_s(tx)
    #
    # @param tx [Y::Transaction]
    # @return [String]

    # @!method yxml_element_unobserve(subscription_id)
    #
    # @param subscription_id [Integer]
    # @return [void]
  end

  # A XMLText
  #
  # Someone should not instantiate a text directly, but use
  # {Y::Doc#get_xml_text}, {Y::XMLElement#insert_text},
  # {Y::XMLElement#push_text}, {Y::XMLElement#unshift_text} instead.
  #
  # The XMLText API is similar to {Y::Text}, but adds a few methods to make it
  # easier to work in structured XML documents.
  #
  # @example
  #   doc = Y::Doc.new
  #   xml_text = doc.get_xml_text("my xml text")
  #
  #   puts xml_text.to_s
  class XMLText
    # @!attribute [r] document
    #
    # @return [Y::Doc] The document this array belongs to
    attr_accessor :document

    # Create a new XMLText instance
    #
    # @param doc [Y::Doc]
    def initialize(doc = nil)
      @document = doc || Y::Doc.new

      super()
    end

    # Push a string to the end of the text node
    #
    # @param str [String]
    # @return {void}
    def <<(str)
      document.current_transaction { |tx| yxml_text_push(tx, str) }
    end

    alias push <<

    # Attach a listener to get notified about changes
    #
    # @param callback [Proc]
    # @return [Integer] subscription_id
    def attach(callback = nil, &block)
      yxml_text_observe(callback) unless callback.nil?
      yxml_text_observe(block.to_proc) unless block.nil?
    end

    # Return text attributes
    #
    # @return [Hash]
    def attrs
      document.current_transaction { |tx| yxml_text_attributes(tx) }
    end

    # Detach a listener
    #
    # @param subscription_id [Integer]
    # @return [void]
    def detach(subscription_id)
      yxml_text_unobserve(subscription_id)
    end

    # Format text
    #
    # @param index [Integer]
    # @param length [Integer]
    # @param attrs [Hash]
    # @return [void]
    def format(index, length, attrs)
      document.current_transaction do |tx|
        yxml_text_format(tx, index, length, attrs)
      end
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
    # @param value [String, Float, Integer, Array, Hash, Boolean]
    # @param attrs [Hash|nil]
    # @return [void]
    def insert(index, value, attrs = nil)
      document.current_transaction do |tx|
        if value.is_a?(String)
          yxml_text_insert(tx, index, value) if attrs.nil?
          unless attrs.nil?
            yxml_text_insert_with_attrs(tx, index, value,
                                        attrs)
          end

          return nil
        end

        if can_insert?(value)
          yxml_text_insert_embed(tx, index, value) if attrs.nil?
          unless attrs.nil?
            yxml_text_insert_embed_with_attrs(tx, index, value,
                                              attrs)
          end

          return nil
        end

        raise ArgumentError,
              "Can't insert value. `#{value.class.name}` isn't supported."
      end
    end

    # rubocop:enable Metrics/MethodLength

    # Return length of string
    #
    # @return [void]
    def length
      document.current_transaction { |tx| yxml_text_length(tx) }
    end

    alias size length

    # Return adjacent XMLElement or XMLText node (next)
    #
    # @return [Y::XMLElement|Y::XMLText|nil]
    def next_sibling
      node = document.current_transaction { |tx| yxml_text_next_sibling(tx) }
      node.document = document
      node
    end

    # Return parent XMLElement
    #
    # @return [Y::XMLElement|nil]
    def parent
      node = yxml_text_parent
      node.document = document
      node
    end

    # Return adjacent XMLElement or XMLText node (prev)
    #
    # @return [Y::XMLElement|Y::XMLText|nil]
    def prev_sibling
      node = document.current_transaction { |tx| yxml_text_prev_sibling(tx) }
      node&.document = document
      node
    end

    # rubocop:disable Metrics/AbcSize, Metrics/CyclomaticComplexity, Metrics/MethodLength

    # Removes a part from text
    #
    # **Attention:** In comparison to String#slice, {XMLText#slice!} will not
    # return the substring that gets removed. Even this being technically
    # possible, it requires us to read the substring before removing it, which
    # is not desirable in most situations.
    #
    # @example Removes a single character
    #   doc = Y::Doc.new
    #
    #   text = doc.get_xml_text("my xml text")
    #   text << "Hello"
    #
    #   text.slice!(0)
    #
    #   text.to_s == "ello" # true
    #
    # @example Removes a range of characters
    #   doc = Y::Doc.new
    #
    #   text = doc.get_xml_text("my xml text")
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
    #   text = doc.get_xml_text("my xml text")
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
        if args.size.zero?
          raise ArgumentError,
                "Provide one of `index`, `range`, `start, length` as arguments"
        end

        if args.size == 1
          arg = args.first

          if arg.is_a?(Range)
            yxml_text_remove_range(tx, arg.first, arg.last - arg.first)
            return nil
          end

          if arg.is_a?(Numeric)
            yxml_text_remove_range(tx, arg.to_int, 1)
            return nil
          end
        end

        if args.size == 2
          first, second = args

          if first.is_a?(Numeric) && second.is_a?(Numeric)
            yxml_text_remove_range(tx, first, second)
            return nil
          end
        end

        raise ArgumentError, "Please check your arguments, can't slice."
      end
    end

    # rubocop:enable Metrics/AbcSize, Metrics/CyclomaticComplexity, Metrics/MethodLength

    # Returns string representation of XMLText
    #
    # @return [String]
    def to_s
      document.current_transaction { |tx| yxml_text_to_s(tx) }
    end

    # rubocop:disable Metrics/AbcSize, Metrics/MethodLength

    # make attributes just work on an element in the form of `attr_name` and
    # `attr_name=`
    #
    # @example Set and get an attribute
    #   doc = Y::Doc.new
    #   xml_element = doc.get_xml_element("my xml")
    #   xml_element.attr_name = "Hello"
    #
    #   puts xml_element.attr_name # "Hello"
    #
    # @!visibility private
    def method_missing(method_name, *args, &block)
      is_setter = method_name.to_s.end_with?("=")

      setter = method_name
      setter += "=" unless is_setter
      getter = method_name
      getter = getter.to_s.slice(0...-1).to_sym if is_setter

      define_singleton_method(setter.to_sym) do |new_val|
        document.current_transaction do |tx|
          yxml_text_insert_attribute(tx,
                                     method_name.to_s
                                                .delete_suffix("=")
                                                .delete_prefix("attr_"),
                                     new_val)
        end
      end

      define_singleton_method(getter) do
        document.current_transaction do |tx|
          yxml_text_get_attribute(tx, method_name.to_s.delete_prefix("attr_"))
        end
      end

      if is_setter
        value = args[0]
        send(setter, value)
      end
    rescue StandardError
      super(method_name, *args, &block)
    end

    # rubocop:enable Metrics/AbcSize, Metrics/MethodLength

    # Make sure we only respond to attributes
    # @!visibility private
    def respond_to_missing?(method_name, include_private = false)
      method_name.to_s.start_with?("attr_") || super
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

    # @!method yxml_text_attributes
    #
    # @return [Hash]

    # @!method yxml_text_format(tx, index, length, attrs)
    #
    # @param tx [Y::Transaction]
    # @param index [Integer]
    # @param length [Integer]
    # @param attrs [Hash]
    # @return [void]

    # @!method yxml_text_get_attribute(tx, name)
    #
    # @param tx [Y::Transaction]
    # @param name [String]
    # @return [String|nil]

    # @!method yxml_text_insert(tx, index, str)
    #
    # @param tx [Y::Transaction]
    # @param index [Integer]
    # @param str [String]
    # @return [void]

    # @!method yxml_text_insert_attribute(tx, name, value)
    #
    # @param tx [Y::Transaction]
    # @param name [String] name
    # @param value [String] value
    # @return [void]

    # @!method yxml_text_insert_with_attrs(tx, index, value, attrs)
    #
    # @param tx [Y::Transaction]
    # @param index [Integer]
    # @param value [String]
    # @param attrs [Hash]
    # @return [void]

    # @!method yxml_text_insert_embed(tx, index, value)
    #
    # @param tx [Y::Transaction]
    # @param index [Integer]
    # @param value [String]
    # @return [void]

    # @!method yxml_text_insert_embed_with_attrs(tx, index, value, attrs)
    #
    # @param tx [Y::Transaction]
    # @param index [Integer]
    # @param value [true|false|Float|Integer|Array|Hash]
    # @param attrs [Hash]
    # @return [void]

    # @!method yxml_text_length(tx)
    #
    # @param tx [Y::Transaction]
    # @return [Integer]

    # @!method yxml_text_next_sibling(tx)
    #
    # @param tx [Y::Transaction]
    # @return [Y::XMLElement|Y::XMLText|nil]

    # @!method yxml_text_observe(callback)
    #
    # @param callback [Proc]
    # @return [Integer] A subscription ID

    # @!method yxml_text_parent
    #
    # @return [Y::XMLElement|nil]

    # @!method yxml_text_prev_sibling(tx)
    #
    # @param tx [Y::Transaction]
    # @return [Y::XMLElement|Y::XMLText|nil]

    # @!method yxml_text_push(tx, str)
    #
    # @param tx [Y::Transaction]
    # @param str [String]
    # @return [void]

    # @!method yxml_text_remove_range(tx, index, length)
    #
    # @param tx [Y::Transaction]
    # @param index [Integer]
    # @param length [Integer]
    # @return [void]

    # @!method yxml_text_to_s(tx)
    #
    # @param tx [Y::Transaction]
    # @return [void]

    # @!method yxml_text_unobserve(subscription_id)
    #
    # @param subscription_id [Integer]
    # @return [void]
  end

  # @!visibility private
  class XMLFragment
    # @!attribute [r] document
    #
    # @return [Y::Doc] The document this array belongs to
    attr_accessor :document
  end

  # rubocop:enable Metrics/ClassLength
end
