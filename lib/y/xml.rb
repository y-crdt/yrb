# frozen_string_literal: true

module Y
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
      yxml_element_get(index)
    end

    # Create a node at index
    #
    # @param [Integer] index
    # @param [String] name Name of node, e.g. `<p />`
    # @return [Y::XMLElement]
    def []=(index, name)
      yxml_element_insert_element(transaction, index, name)
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
      yxml_element_first_child
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
      text << input unless input.nil?
      text
    end

    # Retrieve element or text adjacent (next) to this element
    #
    # @return [Y::XMLElement|Y::XMLText|nil]
    def next_sibling
      yxml_element_next_sibling
    end

    # Retrieve parent element
    #
    # @return [Y::XMLElement|nil]
    def parent
      yxml_element_parent
    end

    # Retrieve element or text adjacent (previous) to this element
    #
    # @return [Y::XMLElement|Y::XMLText|nil]
    def prev_sibling
      yxml_element_prev_sibling
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
      text << args.first unless str.nil?
      text
    end

    # Number of children
    #
    # @return [Integer]
    def size
      yxml_element_size
    end

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

    # A reference to the current active transaction of the document this map
    # belongs to.
    #
    # @return [Y::Transaction] A transaction object
    def transaction
      document.current_transaction
    end
  end

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
  end
end
