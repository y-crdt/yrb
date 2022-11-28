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
    ZERO_STATE = [0].freeze
    private_constant :ZERO_STATE

    # Commit current transaction
    #
    # This is a convenience method that invokes {Y::Transaction#commit} on the
    # current transaction used by this document.
    #
    # @return [void]
    def commit
      current_transaction(&:commit)
    end

    # Create a diff between this document and another document. The diff is
    # created based on a state vector provided by the other document. It only
    # returns the missing blocks, as binary encoded sequence.
    #
    # @param state [::Array<Integer>] The state to create the diff against
    # @return [::Array<Integer>] Binary encoded diff
    def diff(state = ZERO_STATE)
      current_transaction { |tx| ydoc_encode_diff_v1(tx, state) }
    end

    # Creates a full diff for the current document. It is similar to {#diff},
    # but does not take a state. Instead it creates an empty state and passes it
    # to the encode_diff function.
    #
    # @return [::Array<Integer>] Binary encoded diff
    def full_diff
      diff
    end

    # Gets or creates a new array by name
    #
    # If the optional values array is present, fills the array up with elements
    # from the provided array. If the array already exists and isn't
    # empty, elements are pushed to the end of the array.
    #
    # @param name [String] The name of the structure
    # @param values [::Array] Optional initial values
    # @return [Y::Array]
    def get_array(name, values = nil)
      array = ydoc_get_or_insert_array(name)
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
    # @param name [String] The name of the structure
    # @param input [Hash] Optional initial map key-value pairs
    # @return [Y::Map]
    def get_map(name, input = nil)
      map = ydoc_get_or_insert_map(name)
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
    # @param name [String] The name of the structure
    # @param input [String] Optional initial text value
    # @return [Y::Text]
    def get_text(name, input = nil)
      text = ydoc_get_or_insert_text(name)
      text.document = self
      text << input unless input.nil?
      text
    end

    # Gets or creates a new XMLElement by name
    #
    # @param name [String] The name of the structure
    # @return [Y::XMLElement]
    def get_xml_element(name)
      xml_element = ydoc_get_or_insert_xml_element(name)
      xml_element.document = self
      xml_element
    end

    # Gets or creates a new XMLFragment by name
    #
    # @param name [String] The name of the fragment
    # @return [Y::XMLFragment]
    def get_xml_fragment(name)
      xml_fragment = ydoc_get_or_insert_xml_fragment(name)
      xml_fragment.document = self
      xml_fragment
    end

    # Gets or creates a new XMLText by name
    #
    # @param name [String] The name of the structure
    # @param input [String] Optional initial text value
    # @return [Y::XMLText]
    def get_xml_text(name, input = nil)
      xml_text = ydoc_get_or_insert_xml_text(name)
      xml_text.document = self
      xml_text << input unless input.nil?
      xml_text
    end

    # Creates a state vector of this document. This can be used to compare the
    # state of two documents with each other and to later on sync them.
    #
    # @return [::Array<Integer>] Binary encoded state vector
    def state
      current_transaction(&:state)
    end

    # Synchronizes this document with the diff from another document
    #
    # @param diff [::Array<Integer>] Binary encoded update
    # @return [void]
    def sync(diff)
      current_transaction { |tx| tx.apply(diff) }
    end

    # Restores a specific document from an update that contains full state
    #
    # This is doing the same as {#sync}, but it exists to be explicit about
    # the intent. This is the companion to {#full_diff}.
    #
    # @param full_diff [::Array<Integer>] Binary encoded update
    # @return [void]
    def restore(full_diff)
      current_transaction { |tx| tx.apply(full_diff) }
    end

    # Creates a new transaction
    def transact
      # 1. release potentially existing transaction
      if @current_transaction
        @current_transaction.free
        @current_transaction = nil
      end

      # 2. store new transaction in instance variable
      @current_transaction = ydoc_transact
      @current_transaction.document = self

      # 3. call block with reference to current_transaction
      yield @current_transaction
    ensure
      @current_transaction&.free
      @current_transaction = nil
    end

    # @!visibility private
    def current_transaction(&block)
      raise "provide a block" unless block

      # 1. instance variable is set, just use it
      return yield @current_transaction if @current_transaction

      # 2. forward block to transact
      transact(&block) unless @current_transaction
    end

    # @!method ydoc_encode_diff_v1(tx, state_vector)
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

    # @!method ydoc_get_or_insert_xml_element(name)
    #   Creates a new XMLText for the document
    #
    # @return [Y::XMLElement]
    # @!visibility private

    # @!method ydoc_get_or_insert_xml_fragment(name)
    #   Creates a new XMLFragment for the document
    #
    # @return [Y::XMLFragment]
    # @!visibility private

    # @!method ydoc_get_or_insert_xml_text(name)
    #   Creates a new XMLText for the document
    #
    # @return [Y::XMLText]
    # @!visibility private
  end
end
