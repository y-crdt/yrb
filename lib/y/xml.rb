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
    # @param [Y::Doc] doc
    def initialize(doc = nil)
      @document = doc || Y::Doc.new

      super()
    end

    # Retrieve node at index
    #
    # @param [Integer] index
    # @return [Y::XMLElement|nil]
    def [](index)
      node = yxml_element_get(index)
      node&.document = document
      node
    end

    # Create a node at index
    #
    # @param [Integer] index
    # @param [String] name Name of node, e.g. `<p />`
    # @return [Y::XMLElement]
    def []=(index, name)
      node = yxml_element_insert_element(transaction, index, name)
      node&.document = document
      node
    end

    # Returns first child in list or nil if no child exists
    #
    # @return [Hash]
    def attrs
      yxml_element_attributes
    end

    alias attributes attrs

    # Returns first child in list or nil if no child exists
    #
    # @return [Y::XMLElement]
    def first_child
      child = yxml_element_first_child
      child&.document = document
      child
    end

    # Insert text into element at given index
    #
    # Optional input is pushed to the text if provided
    #
    # @param [Integer] index
    # @param [String|nil] input
    # @return [Y::XMLText]
    def insert_text(index, input = nil)
      text = yxml_element_insert_text(transaction, index)
      text&.document = document
      text << input unless input.nil?
      text
    end

    # Retrieve element or text adjacent (next) to this element
    #
    # @return [Y::XMLElement|Y::XMLText|nil]
    def next_sibling
      node = yxml_element_next_sibling
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
    # @param [Proc] callback
    # @param [Block] block
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
      node = yxml_element_prev_sibling
      node&.document = document
      node
    end

    # Creates a new child an inserts at the end of the children list
    #
    # @param [String] name
    # @return [Y::XMLElement]
    def <<(name)
      xml_element = yxml_element_push_elem_back(transaction, name)
      xml_element.document = document
      xml_element
    end

    alias push_child <<

    # Insert new text at the end of this elements child list
    #
    # The optional str argument initializes the text node with its value
    #
    # @param [String] str
    # @return [Y::XMLText]
    def push_text(str = nil)
      text = yxml_element_push_text_back(transaction)
      text.document = document
      text << str unless str.nil?
      text
    end

    # Number of children
    #
    # @return [Integer]
    def size
      yxml_element_size
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
      if args.size.zero?
        raise ArgumentError,
              "Provide one of `index`, `range`, `start, length` as arguments"
      end

      if args.size == 1
        arg = args.first

        if arg.is_a?(Range)
          if arg.exclude_end?
            yxml_element_remove_range(transaction, arg.first,
                                      arg.last - arg.first)
          end
          unless arg.exclude_end?
            yxml_element_remove_range(transaction, arg.first,
                                      arg.last + 1 - arg.first)
          end
          return nil
        end

        if arg.is_a?(Numeric)
          yxml_element_remove_range(transaction, arg.to_int, 1)
          return nil
        end
      end

      if args.size == 2
        first, second = args

        if first.is_a?(Numeric) && second.is_a?(Numeric)
          yxml_element_remove_range(transaction, first, second)
          return nil
        end
      end

      raise ArgumentError, "Please check your arguments, can't slice."
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
      yxml_element_to_s
    end

    # Detach a listener
    #
    # @param [Integer] subscription_id
    # @return [void]
    def detach(subscription_id)
      yxml_element_unobserve(subscription_id)
    end

    # Creates a new node and puts it in front of the child list
    #
    # @param [String] name
    # @return [Y::XMLElement]
    def unshift_child(name)
      xml_element = yxml_element_push_elem_front(transaction, name)
      xml_element.document = document
      xml_element
    end

    # Insert new text at the front of this elements child list
    #
    # The optional str argument initializes the text node with its value
    #
    # @param [String] str
    # @return [Y::XMLText]
    def unshift_text(str = nil)
      text = yxml_element_push_text_front(transaction)
      text.document = document
      text << args.first unless str.nil?
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
      getter = getter.to_s.slice(0...-1).to_sym if is_setter

      define_singleton_method(setter.to_sym) do |new_val|
        yxml_element_insert_attribute(transaction,
                                      method_name.to_s
                                                 .delete_suffix("=")
                                                 .delete_prefix("attr_"),
                                      new_val)
      end

      define_singleton_method(getter) do
        yxml_element_get_attribute(method_name.to_s.delete_prefix("attr_"))
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

    # @!method yxml_element_attributes
    #
    # @return [Hash]

    # @!method yxml_element_first_child
    #
    # @return [Y::XMLElement|Y::XMLText]

    # @!method yxml_element_get_attribute(name)
    #
    # @param [String] name
    # @return [String|nil]

    # @!method yxml_element_get(index)
    #
    # @param [Integer] index
    # @return [Y::XMLElement|Y::XMLText|nil]

    # @!method yxml_element_insert_attribute(transaction, name, value)
    #
    # @param [Y::Transaction] transaction
    # @param [String] name
    # @param [String] value
    # @return [String|nil]

    # @!method yxml_element_insert_element(transaction, index, name)
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [String] name
    # @return [Y::XMLElement]

    # @!method yxml_element_insert_text(transaction, index)
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @return [Y::XMLText]

    # @!method yxml_element_next_sibling()
    #
    # @return [Y::XMLElement|XMLText|nil]

    # @!method yxml_element_observe(callback)
    #
    # @param [Proc] callback
    # @return [Integer] The subscription ID

    # @!method yxml_element_parent()
    #
    # @return [Y::XMLElement|nil]

    # @!method yxml_element_prev_sibling()
    #
    # @return [Y::XMLElement|XMLText|nil]

    # @!method yxml_element_push_elem_back(transaction, name)
    #
    # @param [Y::Transaction] transaction
    # @param [String] name
    # @return [Y::XMLElement]

    # @!method yxml_element_push_elem_front(transaction, name)
    #
    # @param [Y::Transaction] transaction
    # @param [String] name
    # @return [Y::XMLElement]

    # @!method yxml_element_push_text_back(transaction)
    #
    # @param [Y::Transaction] transaction
    # @return [Y::XMLText]

    # @!method yxml_element_push_text_front(transaction)
    #
    # @param [Y::Transaction] transaction
    # @return [Y::XMLText]

    # @!method yxml_element_remove_attribute(transaction, name)
    #
    # @param [Y::Transaction] transaction
    # @param [String] name
    #
    # @return [void]

    # @!method yxml_element_remove_range(transaction, index, length)
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [Integer] length
    #
    # @return [void]

    # @!method yxml_element_size()
    #
    # @return [Integer]

    # @!method yxml_element_tag()
    #
    # @return [String]

    # @!method yxml_element_to_s()
    #
    # @return [String]

    # @!method yxml_element_unobserve(subscription_id)
    #
    # @param [Integer] subscription_id
    # @return [void]

    # A reference to the current active transaction of the document this element
    # belongs to.
    #
    # @return [Y::Transaction] A transaction object
    def transaction
      document.current_transaction
    end
  end

  # A XMLText
  #
  # Someone should not instantiate a text directly, but use
  # {Y::Doc#get_text_element}, {Y::XMLElement#insert_text},
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
    # @param [Y::Doc] doc
    def initialize(doc = nil)
      @document = doc || Y::Doc.new

      super()
    end

    # Push a string to the end of the text node
    #
    # @param [String] str
    # @return {void}
    def <<(str)
      yxml_text_push(transaction, str)
    end

    alias push <<

    # Attach a listener to get notified about changes
    #
    # @param [Proc] callback
    # @return [Integer] subscription_id
    def attach(callback = nil, &block)
      yxml_text_observe(callback) unless callback.nil?
      yxml_text_observe(block.to_proc) unless block.nil?
    end

    # Return text attributes
    #
    # @return [Hash]
    def attrs
      yxml_text_attributes
    end

    # Detach a listener
    #
    # @param [Integer] subscription_id
    # @return [void]
    def detach(subscription_id)
      yxml_text_unobserve(subscription_id)
    end

    # Format text
    #
    # @param [Integer] index
    # @param [Integer] length
    # @param [Hash] attrs
    # @return [void]
    def format(index, length, attrs)
      yxml_text_format(transaction, index, length, attrs)
    end

    # rubocop:disable Metrics/AbcSize, Metrics/MethodLength

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
        yxml_text_insert(transaction, index, value) if attrs.nil?
        unless attrs.nil?
          yxml_text_insert_with_attrs(transaction, index, value,
                                      attrs)
        end
        return nil
      end

      if can_insert?(value)
        yxml_text_insert_embed(transaction, index, value) if attrs.nil?
        unless attrs.nil?
          yxml_text_insert_embed_with_attrs(transaction, index, value,
                                            attrs)
        end
        return nil
      end

      raise ArgumentError,
            "Can't insert value. `#{value.class.name}` isn't supported."
    end

    # rubocop:enable Metrics/AbcSize, Metrics/MethodLength

    # Return length of string
    #
    # @return [void]
    def length
      yxml_text_length
    end

    alias size length

    # Return adjacent XMLElement or XMLText node (next)
    #
    # @return [Y::XMLElement|Y::XMLText|nil]
    def next_sibling
      yxml_text_next_sibling
    end

    # Return parent XMLElement
    #
    # @return [Y::XMLElement|nil]
    def parent
      yxml_text_parent
    end

    # Return adjacent XMLElement or XMLText node (prev)
    #
    # @return [Y::XMLElement|Y::XMLText|nil]
    def prev_sibling
      yxml_text_prev_sibling
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
      if args.size.zero?
        raise ArgumentError,
              "Provide one of `index`, `range`, `start, length` as arguments"
      end

      if args.size == 1
        arg = args.first

        if arg.is_a?(Range)
          yxml_text_remove_range(transaction, arg.first, arg.last - arg.first)
          return nil
        end

        if arg.is_a?(Numeric)
          yxml_text_remove_range(transaction, arg.to_int, 1)
          return nil
        end
      end

      if args.size == 2
        first, second = args

        if first.is_a?(Numeric) && second.is_a?(Numeric)
          yxml_text_remove_range(transaction, first, second)
          return nil
        end
      end

      raise ArgumentError, "Please check your arguments, can't slice."
    end

    # rubocop:enable Metrics/AbcSize, Metrics/CyclomaticComplexity, Metrics/MethodLength

    # Returns string representation of XMLText
    #
    # @return [String]
    def to_s
      yxml_text_to_s
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
        yxml_text_insert_attribute(transaction,
                                   method_name.to_s
                                              .delete_suffix("=")
                                              .delete_prefix("attr_"),
                                   new_val)
      end

      define_singleton_method(getter) do
        yxml_text_get_attribute(method_name.to_s.delete_prefix("attr_"))
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

    # @!method yxml_text_format(transaction, index, length, attrs)
    #
    # @param [Integer] index
    # @param [Integer] length
    # @param [Hash] attrs
    # @return [void]

    # @!method yxml_text_get_attribute(name)
    #
    # @param [String] name
    # @return [String|nil]

    # @!method yxml_text_insert(transaction, index, str)
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [String] str
    # @return [void]

    # @!method yxml_text_insert_attribute(transaction, name, value)
    #
    # @param [Y::Transaction] transaction
    # @param [String] name
    # @param [String] value
    # @return [void]

    # @!method yxml_text_insert_with_attrs(transaction, index, value, attrs)
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [String] value
    # @param [Hash] attrs
    # @return [void]

    # @!method yxml_text_insert_embed(transaction, index, value)
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [String] value
    # @return [void]

    # @!method yxml_text_insert_embed_with_attrs(txn, index, value, attrs)
    #
    # @param [Y::Transaction] txn
    # @param [Integer] index
    # @param [true|false|Float|Integer|Array|Hash] value
    # @param [Hash] attrs
    # @return [void]

    # @!method yxml_text_length
    #
    # @return [Integer]

    # @!method yxml_text_next_sibling
    #
    # @return [Y::XMLElement|Y::XMLText|nil]

    # @!method yxml_text_observe(callback)
    #
    # @param [Proc] callback
    # @return [Integer] A subscription ID

    # @!method yxml_text_parent
    #
    # @return [Y::XMLElement|nil]

    # @!method yxml_text_prev_sibling
    #
    # @return [Y::XMLElement|Y::XMLText|nil]

    # @!method yxml_text_push(transaction, str)
    #
    # @param [Y::Transaction] transaction
    # @param [String] str
    # @return [void]

    # @!method yxml_text_remove_range(transaction, index, length)
    #
    # @param [Y::Transaction] transaction
    # @param [Integer] index
    # @param [Integer] length
    # @return [void]

    # @!method yxml_text_to_s()
    #
    # @return [void]

    # @!method yxml_text_unobserve(subscription_id)
    #
    # @param [Integer] subscription_id
    # @return [void]

    # A reference to the current active transaction of the document this text
    # belongs to.
    #
    # @return [Y::Transaction] A transaction object
    def transaction
      document.current_transaction
    end
  end

  # rubocop:enable Metrics/ClassLength
end
