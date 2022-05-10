# frozen_string_literal: true

require_relative "transaction"

module Y
  # @example Create a local and remote doc and syncs the diff
  #   local = Y::Doc.new
  #   local_map = local.get_map("my map")
  #   local_map[:hello] = "world"
  #
  #   remote = Y::Doc.new
  #
  #   diff = local.diff(remote.state)
  #   remote.sync(diff)
  #
  #   remote_map = remote.get_map("my_map")
  #   pp remote_map.to_h #=> {hello: "world"}
  class Doc
    # The currently active transaction for this document
    # @return [Y::Transaction]
    def current_transaction
      @current_transaction ||= ydoc_transact
    end

    # Create a diff between this document and another document. The diff is
    # created based on a state vector provided by the other document. It only
    # returns the missing blocks, as binary encoded sequence.
    #
    # @param [::Array<Int>] state The state to create the diff against
    # @return [::Array<Int>] Binary encoded diff
    def diff(state)
      ydoc_encode_diff_v1(state)
    end

    # Gets or creates a new array by name
    #
    # If the optional values array is present, fills the array up with elements
    # from the provided array. If the array already exists and isn't
    # empty, elements are pushed to the end of the array.
    #
    # @param [String] name The name of the structure
    # @param [::Array] values Optional initial values
    # @return [Y::Array]
    def get_array(name, values = nil)
      array = current_transaction.get_array(name)
      array.document = self
      array.concat(values) unless values.nil?
      array
    end

    # Gets or creates a new map by name
    #
    # If the optional input hash is present, fills the map up with key-value
    # pairs from the provided input hash. If the map already exists and isn't
    # empty, any existing keys are overridden and new keys are added.
    #
    # @param [String] name The name of the structure
    # @param [Hash] input Optional initial map key-value pairs
    # @return [Y::Map]
    def get_map(name, input = nil)
      map = current_transaction.get_map(name)
      map.document = self
      input&.each { |key, value| map[key] = value }
      map
    end

    # Gets or creates a new text by name
    #
    # If the optional input string is provided, fills a new text with the string
    # at creation time. If the text isn't new and not empty, appends the input
    # to the end of the text.
    #
    # @param [String] name The name of the structure
    # @param [String] input Optional initial text value
    # @return [Y::Text]
    def get_text(name, input = nil)
      text = current_transaction.get_text(name)
      text.document = self
      text.push(current_transaction, input) unless input.nil?
      text
    end

    # Gets or creates a new XMLElement by name
    #
    # @param [String] name The name of the structure
    # @return [Y::XMLElement]
    def get_xml_element(name)
      xml_element = current_transaction.get_xml_element(name)
      xml_element.document = self
      xml_element
    end

    # Gets or creates a new XMLText by name
    #
    # @param [String] name The name of the structure
    # @return [Y::XMLText]
    def get_xml_text(name)
      xml_text = current_transaction.get_xml_text(name)
      xml_text.document = self
      xml_text
    end

    # Creates a state vector of this document. This can be used to compare the
    # state of two documents with each other and to later on sync them.
    #
    # @return [::Array<Int>] Binary encoded state vector
    def state
      current_transaction.state
    end

    # Synchronizes this document with the diff from another document
    #
    # @param [::Array<Int>] diff Binary encoded update
    # @return [void]
    def sync(diff)
      current_transaction.apply(diff)
    end

    # @!method ydoc_encode_diff_v1
    #   Encodes the diff of current document state vs provided state
    #
    #   @example Create transaction on doc
    #     doc = Y::Doc.new
    #     tx = doc.ydoc_encode_diff_v1(other_state)
    #
    # @return [Array<Integer>] Binary encoded update
    # @!visibility private

    # @!method ydoc_transact
    #   Creates a new transaction for the document
    #
    #   @example Create transaction on doc
    #     doc = Y::Doc.new
    #     tx = doc.ydoc_transact
    #
    # @return [Y::Transaction] The transaction object
    # @!visibility private
  end
end
